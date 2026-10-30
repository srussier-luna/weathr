use super::checks::{Check, Status};

pub fn render(checks: &[Check]) -> String {
    let mut report: String = checks.iter().map(line).collect();
    report.push_str(&summary(problem_count(checks)));
    report.push('\n');
    report
}

pub fn problem_count(checks: &[Check]) -> usize {
    checks.iter().filter(|check| !check.is_ok()).count()
}

fn line(check: &Check) -> String {
    match &check.status {
        Status::Ok(detail) => format!("✓ {} {detail}\n", check.name),
        Status::Problem { problem, fix } => format!("✗ {} {problem} → {fix}\n", check.name),
    }
}

fn summary(problems: usize) -> String {
    match problems {
        0 => "Tout est en ordre.".to_string(),
        n => format!("{n} problème(s) à corriger."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(name: &'static str, detail: &str) -> Check {
        Check {
            name,
            status: Status::Ok(detail.to_string()),
        }
    }

    fn problem(name: &'static str, problem: &str, fix: &str) -> Check {
        Check {
            name,
            status: Status::Problem {
                problem: problem.to_string(),
                fix: fix.to_string(),
            },
        }
    }

    #[test]
    fn test_all_ok_ends_with_everything_fine() {
        let checks = [ok("git", "2.43.0"), ok("cargo", "1.82.0")];
        assert_eq!(
            render(&checks),
            "✓ git 2.43.0\n✓ cargo 1.82.0\nTout est en ordre.\n"
        );
        assert_eq!(problem_count(&checks), 0);
    }

    #[test]
    fn test_problems_show_fix_and_count() {
        let checks = [
            ok("git", "2.43.0"),
            problem(
                "cargo",
                "introuvable",
                "installe Rust\u{202F}: https://rustup.rs",
            ),
            problem("Open-Meteo", "injoignable", "vérifie ta connexion"),
        ];
        assert_eq!(
            render(&checks),
            "✓ git 2.43.0\n\
             ✗ cargo introuvable → installe Rust\u{202F}: https://rustup.rs\n\
             ✗ Open-Meteo injoignable → vérifie ta connexion\n\
             2 problème(s) à corriger.\n"
        );
        assert_eq!(problem_count(&checks), 2);
    }
}
