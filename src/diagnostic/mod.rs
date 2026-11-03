mod checks;
mod report;

use crate::progress::{Progress, progress_path};
use crate::remote_weather::FORECAST_URL;
use crate::updater;
use checks::{Check, CloneState, DirAccess, FetchState, ProgressFile, Reachability};
use std::env;
use std::fs::{self, OpenOptions};
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process;
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
    let git = updater::tool_version("git");
    let cargo = updater::tool_version("cargo");
    let (reachability, fetch_state) =
        tokio::join!(reach_open_meteo(), fetch(source, git.is_some()));
    let config_dir = updater::config_dir();
    let progress_path = progress_path();
    let binary_dir = binary_dir();

    vec![
        checks::git(git.as_deref()),
        checks::cargo(cargo.as_deref()),
        checks::source_clone(source, clone_state(source, git.is_some())),
        checks::open_meteo(reachability),
        checks::git_fetch(fetch_state),
        checks::config_dir(dir_access(config_dir.as_deref())),
        checks::progress_file(
            progress_path.as_deref(),
            read_progress(progress_path.as_deref()),
        ),
        checks::binary_dir(dir_access(binary_dir.as_deref())),
    ]
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

async fn fetch(source: &Path, git_available: bool) -> FetchState {
    if !updater::is_git_checkout(source) {
        FetchState::CloneMissing
    } else if !git_available {
        FetchState::GitMissing
    } else if updater::fetch(source).await {
        FetchState::Fetched
    } else {
        FetchState::Failed
    }
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

fn dir_access(dir: Option<&Path>) -> DirAccess {
    let Some(dir) = dir else {
        return DirAccess::Unknown;
    };
    let Some(existing) = dir.ancestors().find(|ancestor| ancestor.exists()) else {
        return DirAccess::Unknown;
    };
    if can_create_file_in(existing) {
        DirAccess::Writable(dir.to_path_buf())
    } else if is_owned_by_user(existing) {
        DirAccess::ReadOnly(existing.to_path_buf())
    } else {
        DirAccess::NotOwned {
            dir: dir.to_path_buf(),
            existing: existing.to_path_buf(),
        }
    }
}

fn can_create_file_in(dir: &Path) -> bool {
    let probe = dir.join(format!("{PROBE_FILE}-{}", process::id()));
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

fn is_owned_by_user(dir: &Path) -> bool {
    let owner = |path: &Path| fs::metadata(path).map(|metadata| metadata.uid()).ok();
    match (owner(dir), dirs::home_dir().as_deref().and_then(owner)) {
        (Some(dir_owner), Some(user)) => dir_owner == user,
        _ => true,
    }
}
