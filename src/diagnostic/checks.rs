use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub struct Check {
    pub name: &'static str,
    pub status: Status,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Ok(String),
    Problem { problem: String, fix: String },
}

impl Check {
    pub(super) fn ok(name: &'static str, detail: impl Into<String>) -> Self {
        Self {
            name,
            status: Status::Ok(detail.into()),
        }
    }

    pub(super) fn problem(
        name: &'static str,
        problem: impl Into<String>,
        fix: impl Into<String>,
    ) -> Self {
        Self {
            name,
            status: Status::Problem {
                problem: problem.into(),
                fix: fix.into(),
            },
        }
    }

    pub fn is_ok(&self) -> bool {
        matches!(self.status, Status::Ok(_))
    }
}

pub enum CloneState {
    Missing,
    NotGit,
    GitMissing,
    Dirty,
    NoUpstream,
    Behind(usize),
}

pub enum Reachability {
    Reachable,
    HttpError(u16),
    Unreachable,
}

pub enum FetchState {
    Fetched,
    Failed,
    GitMissing,
    CloneMissing,
}

pub enum DirAccess {
    Unknown,
    Writable(PathBuf),
    ReadOnly(PathBuf),
    NotOwned { dir: PathBuf, existing: PathBuf },
}

pub enum ProgressFile {
    Absent,
    Unreadable(io::Error),
    Invalid,
    Valid,
}

pub fn git(version: Option<&str>) -> Check {
    tool(
        "git",
        version,
        "installe git\u{202F}: https://git-scm.com/downloads",
    )
}

pub fn cargo(version: Option<&str>) -> Check {
    tool("cargo", version, "installe Rust\u{202F}: https://rustup.rs")
}

fn tool(name: &'static str, version: Option<&str>, fix: &str) -> Check {
    match version {
        Some(version) => Check::ok(name, version_number(name, version)),
        None => Check::problem(name, "introuvable", fix),
    }
}

fn version_number<'a>(name: &str, version: &'a str) -> &'a str {
    let version = version.trim();
    version
        .strip_prefix(name)
        .map(|rest| rest.trim_start().trim_start_matches("version").trim_start())
        .unwrap_or(version)
}

pub fn source_clone(dir: &Path, state: CloneState) -> Check {
    let quoted_dir = quoted(dir);
    let dir = dir.display();
    match state {
        CloneState::Missing => Check::problem(
            "clone",
            format!("{dir} introuvable"),
            "reclone weathr à cet endroit",
        ),
        CloneState::NotGit => Check::problem(
            "clone",
            format!("{dir} n'est pas un dépôt git"),
            "reclone weathr à cet endroit",
        ),
        CloneState::GitMissing => Check::problem(
            "clone",
            format!("{dir} impossible à vérifier sans git"),
            "installe git d'abord",
        ),
        CloneState::Dirty => Check::problem(
            "clone",
            format!("{dir} a des modifications locales"),
            format!("commite-les ou annule-les (git -C {quoted_dir} status)"),
        ),
        CloneState::NoUpstream => Check::problem(
            "clone",
            format!("{dir} ne suit aucune branche distante"),
            format!(
                "fais-lui suivre origin/main\u{202F}: git -C {quoted_dir} branch -u origin/main"
            ),
        ),
        CloneState::Behind(0) => Check::ok("clone", format!("{dir}, à jour")),
        CloneState::Behind(count) => Check::ok(
            "clone",
            format!("{dir}, {count} commit(s) de retard, lance weathr --update"),
        ),
    }
}

pub fn open_meteo(reachability: Reachability) -> Check {
    match reachability {
        Reachability::Reachable => Check::ok("Open-Meteo", "joignable"),
        Reachability::HttpError(status) => Check::problem(
            "Open-Meteo",
            format!("répond {status}"),
            "réessaie plus tard",
        ),
        Reachability::Unreachable => {
            Check::problem("Open-Meteo", "injoignable", "vérifie ta connexion")
        }
    }
}

pub fn git_fetch(state: FetchState) -> Check {
    match state {
        FetchState::Fetched => Check::ok("git fetch", "OK"),
        FetchState::Failed => Check::problem(
            "git fetch",
            "a échoué",
            "vérifie ta connexion et l'accès au dépôt (git remote -v)",
        ),
        FetchState::GitMissing => {
            Check::problem("git fetch", "impossible sans git", "installe git d'abord")
        }
        FetchState::CloneMissing => Check::problem(
            "git fetch",
            "impossible sans le clone",
            "reclone weathr d'abord",
        ),
    }
}

pub fn config_dir(access: DirAccess) -> Check {
    writable_dir(
        "dossier de config",
        access,
        "vérifie la variable HOME",
        take_ownership,
    )
}

fn take_ownership(dir: &Path, existing: &Path) -> String {
    let chown = format!("sudo chown -R \"$USER\" {}", quoted(dir));
    if dir == existing {
        format!("récupère-le\u{202F}: {chown}")
    } else {
        format!(
            "crée-le à ton nom\u{202F}: sudo mkdir -p {} && {chown}",
            quoted(dir)
        )
    }
}

pub fn binary_dir(access: DirAccess) -> Check {
    writable_dir(
        "dossier du binaire",
        access,
        "réinstalle weathr dans ~/.local/bin",
        |_, _| "installe weathr dans un dossier à toi, comme ~/.local/bin".to_string(),
    )
}

fn writable_dir(
    name: &'static str,
    access: DirAccess,
    unknown_fix: &str,
    not_owned_fix: impl FnOnce(&Path, &Path) -> String,
) -> Check {
    match access {
        DirAccess::Unknown => Check::problem(name, "introuvable", unknown_fix),
        DirAccess::Writable(dir) => Check::ok(name, dir.display().to_string()),
        DirAccess::ReadOnly(dir) => Check::problem(
            name,
            format!("{} en lecture seule", dir.display()),
            format!("donne-toi les droits\u{202F}: chmod u+w {}", quoted(&dir)),
        ),
        DirAccess::NotOwned { dir, existing } => Check::problem(
            name,
            format!("{} n'est pas à toi", existing.display()),
            not_owned_fix(&dir, &existing),
        ),
    }
}

fn quoted(path: &Path) -> String {
    format!("\"{}\"", path.display())
}

pub fn progress_file(path: Option<&Path>, file: ProgressFile) -> Check {
    let Some(path) = path else {
        return Check::problem("progress.json", "introuvable", "vérifie la variable HOME");
    };
    let path = path.display();
    match file {
        ProgressFile::Absent => Check::ok("progress.json", "pas encore créé"),
        ProgressFile::Valid => Check::ok("progress.json", path.to_string()),
        ProgressFile::Unreadable(error) => Check::problem(
            "progress.json",
            format!("illisible ({error})"),
            format!("vérifie les droits de {path}"),
        ),
        ProgressFile::Invalid => Check::problem(
            "progress.json",
            "JSON mal formé",
            format!("corrige {path} ou supprime-le (ta partie repartira de zéro)"),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPACED_DIR: &str = "/Users/sam/Library/Application Support/weathr";

    fn fix(check: &Check) -> &str {
        match &check.status {
            Status::Problem { fix, .. } => fix,
            Status::Ok(_) => panic!("expected a problem"),
        }
    }

    fn problem(check: &Check) -> &str {
        match &check.status {
            Status::Problem { problem, .. } => problem,
            Status::Ok(_) => panic!("expected a problem"),
        }
    }

    #[test]
    fn test_tool_shows_version_without_its_name() {
        assert_eq!(
            git(Some("git version 2.39.5 (Apple Git-154)\n")),
            Check::ok("git", "2.39.5 (Apple Git-154)")
        );
        assert_eq!(
            cargo(Some("cargo 1.82.0 (8f40fc59f 2024-08-21)\n")),
            Check::ok("cargo", "1.82.0 (8f40fc59f 2024-08-21)")
        );
    }

    #[test]
    fn test_missing_tool_gives_the_fix() {
        let check = cargo(None);
        assert!(!check.is_ok());
        assert_eq!(fix(&check), "installe Rust\u{202F}: https://rustup.rs");
        assert_eq!(
            fix(&git(None)),
            "installe git\u{202F}: https://git-scm.com/downloads"
        );
    }

    #[test]
    fn test_dirty_clone_asks_to_commit_or_discard() {
        let check = source_clone(Path::new("/src/weathr"), CloneState::Dirty);
        assert!(!check.is_ok());
        assert!(fix(&check).starts_with("commite-les ou annule-les"));
    }

    #[test]
    fn test_clone_commands_quote_the_path() {
        let dir = Path::new(SPACED_DIR);
        assert_eq!(
            fix(&source_clone(dir, CloneState::Dirty)),
            format!("commite-les ou annule-les (git -C \"{SPACED_DIR}\" status)")
        );
        assert_eq!(
            fix(&source_clone(dir, CloneState::NoUpstream)),
            format!(
                "fais-lui suivre origin/main\u{202F}: git -C \"{SPACED_DIR}\" branch -u origin/main"
            )
        );
    }

    #[test]
    fn test_clone_behind_is_fine() {
        assert!(source_clone(Path::new("/src/weathr"), CloneState::Behind(0)).is_ok());
        assert!(source_clone(Path::new("/src/weathr"), CloneState::Behind(3)).is_ok());
    }

    #[test]
    fn test_unreachable_network_points_to_connection() {
        assert_eq!(
            fix(&open_meteo(Reachability::Unreachable)),
            "vérifie ta connexion"
        );
        assert!(fix(&git_fetch(FetchState::Failed)).starts_with("vérifie ta connexion"));
    }

    #[test]
    fn test_successful_fetch_is_fine() {
        assert!(git_fetch(FetchState::Fetched).is_ok());
    }

    #[test]
    fn test_skipped_fetch_points_to_its_real_cause() {
        let without_git = git_fetch(FetchState::GitMissing);
        assert_eq!(problem(&without_git), "impossible sans git");
        assert_eq!(fix(&without_git), "installe git d'abord");

        let without_clone = git_fetch(FetchState::CloneMissing);
        assert_eq!(problem(&without_clone), "impossible sans le clone");
        assert_eq!(fix(&without_clone), "reclone weathr d'abord");
    }

    #[test]
    fn test_invalid_progress_points_to_the_file() {
        let check = progress_file(
            Some(Path::new("/home/weathr/progress.json")),
            ProgressFile::Invalid,
        );
        assert_eq!(problem(&check), "JSON mal formé");
        assert!(fix(&check).contains("/home/weathr/progress.json"));
    }

    #[test]
    fn test_absent_progress_is_fine() {
        assert!(progress_file(Some(Path::new("/p.json")), ProgressFile::Absent).is_ok());
    }

    #[test]
    fn test_writable_dir_is_fine() {
        assert_eq!(
            config_dir(DirAccess::Writable(PathBuf::from(SPACED_DIR))),
            Check::ok("dossier de config", SPACED_DIR)
        );
    }

    #[test]
    fn test_read_only_dir_gives_quoted_chmod() {
        let check = config_dir(DirAccess::ReadOnly(PathBuf::from(SPACED_DIR)));
        assert_eq!(
            fix(&check),
            format!("donne-toi les droits\u{202F}: chmod u+w \"{SPACED_DIR}\"")
        );
        assert_eq!(
            fix(&binary_dir(DirAccess::ReadOnly(PathBuf::from("/opt/bin")))),
            "donne-toi les droits\u{202F}: chmod u+w \"/opt/bin\""
        );
    }

    #[test]
    fn test_dir_not_owned_skips_chmod() {
        let config = config_dir(DirAccess::NotOwned {
            dir: PathBuf::from(SPACED_DIR),
            existing: PathBuf::from(SPACED_DIR),
        });
        assert_eq!(problem(&config), format!("{SPACED_DIR} n'est pas à toi"));
        assert_eq!(
            fix(&config),
            format!("récupère-le\u{202F}: sudo chown -R \"$USER\" \"{SPACED_DIR}\"")
        );

        let binary = binary_dir(DirAccess::NotOwned {
            dir: PathBuf::from("/nix/store/weathr/bin"),
            existing: PathBuf::from("/nix/store/weathr/bin"),
        });
        assert_eq!(
            fix(&binary),
            "installe weathr dans un dossier à toi, comme ~/.local/bin"
        );
    }

    #[test]
    fn test_config_ancestor_not_owned_chowns_only_the_weathr_folder() {
        let ancestor = "/Users/sam/Library/Application Support";
        let check = config_dir(DirAccess::NotOwned {
            dir: PathBuf::from(SPACED_DIR),
            existing: PathBuf::from(ancestor),
        });
        assert_eq!(problem(&check), format!("{ancestor} n'est pas à toi"));
        assert_eq!(
            fix(&check),
            format!(
                "crée-le à ton nom\u{202F}: sudo mkdir -p \"{SPACED_DIR}\" && sudo chown -R \"$USER\" \"{SPACED_DIR}\""
            )
        );
    }

    #[test]
    fn test_unknown_dir_hint_depends_on_the_check() {
        assert_eq!(
            fix(&config_dir(DirAccess::Unknown)),
            "vérifie la variable HOME"
        );
        assert_eq!(
            fix(&binary_dir(DirAccess::Unknown)),
            "réinstalle weathr dans ~/.local/bin"
        );
    }
}
