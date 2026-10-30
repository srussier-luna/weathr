mod checks;
mod report;

use crate::progress::{Progress, progress_path};
use crate::remote_weather::FORECAST_URL;
use crate::updater;
use checks::{Check, CloneState, ProgressFile, Reachability};
use std::env;
use std::fs::{self, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
const PROBE_FILE: &str = ".weathr-diagnostic";

pub async fn run() -> bool {
    let checks = collect().await;
    print!("{}", report::render(&checks));
    report::problem_count(&checks) == 0
}

async fn collect() -> Vec<Check> {
    let source = Path::new(updater::SOURCE_DIR);
    let git = version("git");
    let cargo = version("cargo");
    let (reachability, fetched) = tokio::join!(reach_open_meteo(), fetch(source, git.is_some()));
    let config_dir = updater::config_dir();
    let progress_path = progress_path();
    let binary_dir = binary_dir();

    vec![
        checks::tool(
            "git",
            git.as_deref(),
            "installe git\u{202F}: https://git-scm.com/downloads",
        ),
        checks::tool(
            "cargo",
            cargo.as_deref(),
            "installe Rust\u{202F}: https://rustup.rs",
        ),
        checks::source_clone(source, clone_state(source, git.is_some())),
        checks::open_meteo(reachability),
        checks::git_fetch(fetched),
        checks::writable_dir(
            "dossier de config",
            config_dir.as_deref(),
            config_dir.as_deref().is_some_and(is_writable),
        ),
        checks::progress_file(
            progress_path.as_deref(),
            read_progress(progress_path.as_deref()),
        ),
        checks::writable_dir(
            "dossier du binaire",
            binary_dir.as_deref(),
            binary_dir.as_deref().is_some_and(is_writable),
        ),
    ]
}

fn version(tool: &str) -> Option<String> {
    let output = Command::new(tool)
        .arg("--version")
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

fn clone_state(source: &Path, git_available: bool) -> CloneState {
    if !source.exists() {
        CloneState::Missing
    } else if !updater::is_git_checkout(source) {
        CloneState::NotGit
    } else if !git_available {
        CloneState::GitMissing
    } else if !updater::is_clean(source) {
        CloneState::Dirty
    } else {
        updater::pending_count(source).map_or(CloneState::NoUpstream, CloneState::Behind)
    }
}

async fn reach_open_meteo() -> Reachability {
    let Ok(client) = reqwest::Client::builder().timeout(REQUEST_TIMEOUT).build() else {
        return Reachability::Unreachable;
    };
    let url = format!("{FORECAST_URL}?latitude=0&longitude=0&current=temperature_2m");
    match client.get(url).send().await {
        Ok(response) if response.status().is_success() => Reachability::Reachable,
        Ok(response) => Reachability::HttpError(response.status().as_u16()),
        Err(_) => Reachability::Unreachable,
    }
}

async fn fetch(source: &Path, git_available: bool) -> bool {
    git_available && updater::is_git_checkout(source) && updater::fetch(source).await
}

fn read_progress(path: Option<&Path>) -> ProgressFile {
    let Some(path) = path else {
        return ProgressFile::Absent;
    };
    match fs::read_to_string(path) {
        Ok(content) if serde_json::from_str::<Progress>(&content).is_ok() => ProgressFile::Valid,
        Ok(_) => ProgressFile::Invalid,
        Err(error) if error.kind() == io::ErrorKind::NotFound => ProgressFile::Absent,
        Err(error) => ProgressFile::Unreadable(error),
    }
}

fn binary_dir() -> Option<PathBuf> {
    let executable = fs::canonicalize(env::current_exe().ok()?).ok()?;
    executable.parent().map(Path::to_path_buf)
}

fn is_writable(dir: &Path) -> bool {
    let Some(existing) = dir.ancestors().find(|ancestor| ancestor.exists()) else {
        return false;
    };
    let probe = existing.join(PROBE_FILE);
    let created = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&probe)
        .is_ok();
    if created {
        let _ = fs::remove_file(&probe);
    }
    created
}
