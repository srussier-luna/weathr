use std::io;
use std::path::Path;

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
    fn ok(name: &'static str, detail: impl Into<String>) -> Self {
        Self {
            name,
            status: Status::Ok(detail.into()),
        }
    }

    fn problem(name: &'static str, problem: impl Into<String>, fix: impl Into<String>) -> Self {
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

pub enum ProgressFile {
    Absent,
    Unreadable(io::Error),
    Invalid,
    Valid,
}

pub fn tool(name: &'static str, version: Option<&str>, fix: &str) -> Check {
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
            format!("commite-les ou annule-les (git -C {dir} status)"),
        ),
        CloneState::NoUpstream => Check::problem(
            "clone",
            format!("{dir} ne suit aucune branche distante"),
            format!("relie-la\u{202F}: git -C {dir} branch -u origin/main"),
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

pub fn git_fetch(fetched: bool) -> Check {
    if fetched {
        Check::ok("git fetch", "OK")
    } else {
        Check::problem(
            "git fetch",
            "a échoué",
            "vérifie ta connexion et l'accès au dépôt (git remote -v)",
        )
    }
}

pub fn writable_dir(name: &'static str, dir: Option<&Path>, writable: bool) -> Check {
    match dir {
        None => Check::problem(name, "introuvable", "vérifie la variable HOME"),
        Some(dir) if writable => Check::ok(name, dir.display().to_string()),
        Some(dir) => Check::problem(
            name,
            format!("{} en lecture seule", dir.display()),
            format!("donne-toi les droits\u{202F}: chmod u+w {}", dir.display()),
        ),
    }
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
            "invalide",
            format!("corrige {path} ou supprime-le (ta partie repartira de zéro)"),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fix(check: &Check) -> &str {
        match &check.status {
            Status::Problem { fix, .. } => fix,
            Status::Ok(_) => panic!("expected a problem"),
        }
    }

    #[test]
    fn test_tool_shows_version_without_its_name() {
        assert_eq!(
            tool("git", Some("git version 2.39.5 (Apple Git-154)\n"), ""),
            Check::ok("git", "2.39.5 (Apple Git-154)")
        );
        assert_eq!(
            tool("cargo", Some("cargo 1.82.0 (8f40fc59f 2024-08-21)\n"), ""),
            Check::ok("cargo", "1.82.0 (8f40fc59f 2024-08-21)")
        );
    }

    #[test]
    fn test_missing_tool_gives_the_fix() {
        let check = tool("cargo", None, "installe Rust\u{202F}: https://rustup.rs");
        assert!(!check.is_ok());
        assert_eq!(fix(&check), "installe Rust\u{202F}: https://rustup.rs");
    }

    #[test]
    fn test_dirty_clone_asks_to_commit_or_discard() {
        let check = source_clone(Path::new("/src/weathr"), CloneState::Dirty);
        assert!(!check.is_ok());
        assert!(fix(&check).starts_with("commite-les ou annule-les"));
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
        assert!(fix(&git_fetch(false)).starts_with("vérifie ta connexion"));
    }

    #[test]
    fn test_invalid_progress_points_to_the_file() {
        let check = progress_file(
            Some(Path::new("/home/weathr/progress.json")),
            ProgressFile::Invalid,
        );
        assert!(fix(&check).contains("/home/weathr/progress.json"));
    }

    #[test]
    fn test_absent_progress_is_fine() {
        assert!(progress_file(Some(Path::new("/p.json")), ProgressFile::Absent).is_ok());
    }

    #[test]
    fn test_read_only_dir_gives_chmod() {
        let check = writable_dir("binaire", Some(Path::new("/usr/bin")), false);
        assert_eq!(
            fix(&check),
            "donne-toi les droits\u{202F}: chmod u+w /usr/bin"
        );
    }
}
