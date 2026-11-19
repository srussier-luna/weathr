use std::io::{self, Write};

pub(crate) const PREFIX: &str = "[weathr]";

pub(crate) fn confirm(question: &str, default_yes: bool) -> bool {
    let choices = if default_yes { "[O/n]" } else { "[o/N]" };
    print!("{PREFIX} {question} {choices} ");
    let _ = io::stdout().flush();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).is_ok() && accepts(&answer, default_yes)
}

fn accepts(answer: &str, default_yes: bool) -> bool {
    match answer.trim().to_lowercase().as_str() {
        "" => default_yes,
        "o" | "oui" | "y" | "yes" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_or_yes_answers_accept_when_yes_is_the_default() {
        for answer in ["\n", " O \n", "oui", "Y"] {
            assert!(accepts(answer, true), "{answer:?}");
        }
        for answer in ["n", "non"] {
            assert!(!accepts(answer, true), "{answer:?}");
        }
    }

    #[test]
    fn test_only_an_explicit_yes_accepts_when_no_is_the_default() {
        for answer in ["o\n", "oui", "Y", " yes \n"] {
            assert!(accepts(answer, false), "{answer:?}");
        }
        for answer in ["", "\n", "n", "non", "no", "ok"] {
            assert!(!accepts(answer, false), "{answer:?}");
        }
    }
}
