mod banner;
mod progress;

use chrono::Local;
use progress::Progress;
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, IsTerminal, Read, Write};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::Duration;

const SOURCE_DIR: &str = env!("CARGO_MANIFEST_DIR");
const CHECK_FILE: &str = "update_check";
const FETCH_TIMEOUT: Duration = Duration::from_secs(5);
const LOG_FILE: &str = "update.log";
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
    if !has_pending_changes(source) {
        return;
    }

    banner::show();
    if !ask_confirmation() {
        return;
    }
    match update(source) {
        Ok(()) => restart(),
        Err(error) => {
            eprintln!("{PREFIX} {error}");
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

fn has_pending_changes(source: &Path) -> bool {
    git_output(source, &["rev-list", "--count", "HEAD..@{u}"])
        .and_then(|count| count.trim().parse::<usize>().ok())
        .is_some_and(|count| count > 0)
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
    let progress = Progress::start();
    let mut log = String::new();
    let result = install_update(source, &progress, &mut log);
    progress.finish();
    result.map_err(|error| {
        log.push_str(&error.to_string());
        log.push('\n');
        failure(&error, &log)
    })
}

fn install_update(source: &Path, progress: &Progress, log: &mut String) -> io::Result<()> {
    let pull = Command::new("git")
        .args(["pull", "--ff-only"])
        .current_dir(source)
        .stdin(Stdio::null())
        .output();
    check(pull, "git pull", log)?;
    progress.set(progress::AFTER_PULL);

    build(source, progress, log)?;

    install(&source.join("target").join("release").join("weathr"))?;
    progress.set(progress::DONE);
    Ok(())
}

fn build(source: &Path, progress: &Progress, log: &mut String) -> io::Result<()> {
    let total = fs::read_to_string(source.join("Cargo.lock"))
        .map(|lock| progress::package_count(&lock))
        .unwrap_or_default();
    let mut child = Command::new("cargo")
        .args(["build", "--release", "--message-format=json"])
        .current_dir(source)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| command_error(error, "cargo build"))?;

    let stderr = child.stderr.take().map(|mut stderr| {
        thread::spawn(move || {
            let mut captured = Vec::new();
            let _ = stderr.read_to_end(&mut captured);
            captured
        })
    });

    let mut artifacts = 0;
    if let Some(stdout) = child.stdout.take() {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            match cargo_message(&line) {
                CargoMessage::Artifact => {
                    artifacts += 1;
                    progress.set(progress::build_percent(artifacts, total));
                }
                CargoMessage::Diagnostic(rendered) => log.push_str(&rendered),
                CargoMessage::Other => {}
                CargoMessage::Text => {
                    log.push_str(&line);
                    log.push('\n');
                }
            }
        }
    }

    let status = child.wait()?;
    let stderr = stderr
        .and_then(|reader| reader.join().ok())
        .unwrap_or_default();
    check(
        Ok(Output {
            status,
            stdout: Vec::new(),
            stderr,
        }),
        "cargo build",
        log,
    )
}

#[derive(Debug, PartialEq)]
enum CargoMessage {
    Artifact,
    Diagnostic(String),
    Other,
    Text,
}

fn cargo_message(line: &str) -> CargoMessage {
    let Ok(message) = serde_json::from_str::<serde_json::Value>(line) else {
        return CargoMessage::Text;
    };
    match message["reason"].as_str() {
        Some("compiler-artifact") => CargoMessage::Artifact,
        Some("compiler-message") => message["message"]["rendered"]
            .as_str()
            .map_or(CargoMessage::Other, |rendered| {
                CargoMessage::Diagnostic(rendered.to_string())
            }),
        _ => CargoMessage::Other,
    }
}

fn check(output: io::Result<Output>, name: &str, log: &mut String) -> io::Result<()> {
    let output = output.map_err(|error| command_error(error, name))?;
    log.push_str(&String::from_utf8_lossy(&output.stdout));
    log.push_str(&String::from_utf8_lossy(&output.stderr));
    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!("{name} a échoué")))
    }
}

fn command_error(error: io::Error, name: &str) -> io::Error {
    match error.kind() {
        io::ErrorKind::NotFound => io::Error::other(format!("{name} : commande introuvable")),
        _ => io::Error::other(format!("{name} : {error}")),
    }
}

fn failure(error: &io::Error, log: &str) -> io::Error {
    match write_log(log) {
        Some(path) => io::Error::other(failure_message(&path)),
        None => io::Error::other(format!("Mise à jour impossible : {error}")),
    }
}

fn failure_message(log_path: &Path) -> String {
    format!(
        "Mise à jour impossible. Détails dans {}",
        log_path.display()
    )
}

fn write_log(log: &str) -> Option<PathBuf> {
    let path = config_path(LOG_FILE)?;
    fs::create_dir_all(path.parent()?).ok()?;
    fs::write(&path, log).ok()?;
    Some(path)
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

fn config_path(file: &str) -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("weathr").join(file))
}

fn check_path() -> Option<PathBuf> {
    config_path(CHECK_FILE)
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

    #[test]
    fn test_cargo_artifact_line_counts_as_progress() {
        let line = r#"{"reason":"compiler-artifact","package_id":"weathr"}"#;
        assert_eq!(cargo_message(line), CargoMessage::Artifact);
    }

    #[test]
    fn test_cargo_compiler_message_keeps_rendered_error() {
        let line = r#"{"reason":"compiler-message","message":{"rendered":"error[E0425]: oops\n"}}"#;
        assert_eq!(
            cargo_message(line),
            CargoMessage::Diagnostic("error[E0425]: oops\n".to_string())
        );
    }

    #[test]
    fn test_cargo_plain_line_is_kept_as_text() {
        assert_eq!(cargo_message("warning: something"), CargoMessage::Text);
        assert_eq!(
            cargo_message(r#"{"reason":"build-finished","success":true}"#),
            CargoMessage::Other
        );
    }

    #[test]
    fn test_failure_message_points_to_log() {
        assert_eq!(
            failure_message(Path::new("/tmp/weathr/update.log")),
            "Mise à jour impossible. Détails dans /tmp/weathr/update.log"
        );
    }
}
