use chrono::Local;
use std::env;
use std::fs;
use std::io::{self, IsTerminal, Write};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

const SOURCE_DIR: &str = env!("CARGO_MANIFEST_DIR");
const CHECK_FILE: &str = "update_check";
const FETCH_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_LISTED_CHANGES: usize = 5;
const PREFIX: &str = "[weathr]";

pub async fn offer_update() {
    if !io::stdin().is_terminal() || has_checked_today() {
        return;
    }
    record_check();

    let source = Path::new(SOURCE_DIR);
    if !is_git_checkout(source) || !is_clean(source) || !fetch(source).await {
        return;
    }
    let changes = pending_changes(source);
    if changes.is_empty() {
        return;
    }

    print_changes(&changes);
    if !ask_confirmation() {
        return;
    }
    match update(source) {
        Ok(()) => restart(),
        Err(error) => {
            eprintln!("{PREFIX} Mise à jour impossible : {error}");
            wait_for_enter();
        }
    }
}

pub fn update_now() -> io::Result<()> {
    let source = Path::new(SOURCE_DIR);
    if !is_git_checkout(source) {
        return Err(io::Error::other(format!(
            "dépôt git introuvable : {SOURCE_DIR}"
        )));
    }
    update(source)?;
    println!("{PREFIX} weathr est à jour.");
    Ok(())
}

fn is_git_checkout(source: &Path) -> bool {
    source.join(".git").exists()
}

fn git_output(source: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(source)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

fn is_clean(source: &Path) -> bool {
    git_output(source, &["status", "--porcelain", "--untracked-files=no"])
        .is_some_and(|status| status.trim().is_empty())
}

async fn fetch(source: &Path) -> bool {
    let mut command = tokio::process::Command::new("git");
    command
        .args(["fetch", "--quiet"])
        .current_dir(source)
        .env("GIT_TERMINAL_PROMPT", "0")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .kill_on_drop(true);
    if env::var_os("GIT_SSH_COMMAND").is_none() {
        command.env("GIT_SSH_COMMAND", "ssh -o BatchMode=yes");
    }
    matches!(
        tokio::time::timeout(FETCH_TIMEOUT, command.status()).await,
        Ok(Ok(status)) if status.success()
    )
}

fn pending_changes(source: &Path) -> Vec<String> {
    git_output(source, &["log", "--format=%s", "HEAD..@{u}"])
        .map(|log| log.lines().map(str::to_string).collect())
        .unwrap_or_default()
}

fn print_changes(changes: &[String]) {
    println!("{PREFIX} Une nouvelle version est disponible :");
    for change in changes.iter().take(MAX_LISTED_CHANGES) {
        println!("  • {change}");
    }
    if changes.len() > MAX_LISTED_CHANGES {
        println!("  … et {} autres", changes.len() - MAX_LISTED_CHANGES);
    }
}

fn ask_confirmation() -> bool {
    print!("{PREFIX} Mettre à jour maintenant ? [O/n] ");
    let _ = io::stdout().flush();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).is_ok() && accepts(&answer)
}

fn accepts(answer: &str) -> bool {
    matches!(
        answer.trim().to_lowercase().as_str(),
        "" | "o" | "oui" | "y" | "yes"
    )
}

fn update(source: &Path) -> io::Result<()> {
    run(
        Command::new("git")
            .args(["pull", "--ff-only"])
            .current_dir(source),
        "git pull",
    )?;
    run(
        Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(source),
        "cargo build",
    )?;
    install(&source.join("target").join("release").join("weathr"))
}

fn run(command: &mut Command, name: &str) -> io::Result<()> {
    let status = command.status().map_err(|error| match error.kind() {
        io::ErrorKind::NotFound => io::Error::other(format!("{name} : commande introuvable")),
        _ => io::Error::other(format!("{name} : {error}")),
    })?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!("{name} a échoué")))
    }
}

fn install(built: &Path) -> io::Result<()> {
    let current = fs::canonicalize(env::current_exe()?)?;
    if current == fs::canonicalize(built)? {
        return Ok(());
    }
    let staged = current.with_extension("new");
    fs::copy(built, &staged)?;
    fs::rename(&staged, &current)
}

fn restart() {
    println!("{PREFIX} Mise à jour terminée, redémarrage…");
    if let Ok(executable) = env::current_exe() {
        let error = Command::new(executable).args(env::args_os().skip(1)).exec();
        eprintln!("{PREFIX} Redémarrage impossible : {error}");
    }
}

fn wait_for_enter() {
    print!("{PREFIX} Appuie sur [Entrée] pour continuer. ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut String::new());
}

fn check_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("weathr").join(CHECK_FILE))
}

fn today() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

fn has_checked_today() -> bool {
    check_path()
        .and_then(|path| fs::read_to_string(path).ok())
        .is_some_and(|date| date.trim() == today())
}

fn record_check() {
    let Some(path) = check_path() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, today());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_or_yes_answers_accept_update() {
        assert!(accepts("\n"));
        assert!(accepts(" O \n"));
        assert!(accepts("oui"));
        assert!(accepts("Y"));
        assert!(!accepts("n"));
        assert!(!accepts("non"));
    }
}
