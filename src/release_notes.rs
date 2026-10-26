use chrono::NaiveDate;
use std::cmp::Reverse;

const NOTES: &str = include_str!("../NOUVEAUTES.md");
const DATE_FORMAT: &str = "%Y-%m-%d";
const HEADING: &str = "## ";
const BULLET: &str = "- ";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub date: NaiveDate,
    pub items: Vec<String>,
}

pub fn load() -> Vec<Section> {
    parse(NOTES)
}

pub fn parse(content: &str) -> Vec<Section> {
    let mut sections: Vec<Section> = Vec::new();
    let mut current: Option<Section> = None;

    for line in content.lines() {
        if let Some(heading) = line.strip_prefix(HEADING) {
            sections.extend(current.take());
            current = NaiveDate::parse_from_str(heading.trim(), DATE_FORMAT)
                .ok()
                .map(|date| Section {
                    date,
                    items: Vec::new(),
                });
            continue;
        }

        let Some(section) = current.as_mut() else {
            continue;
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let bullet = trimmed.strip_prefix(BULLET);
        if bullet.is_none()
            && line.starts_with(char::is_whitespace)
            && let Some(item) = section.items.last_mut()
        {
            item.push(' ');
            item.push_str(trimmed);
        } else {
            section.items.push(bullet.unwrap_or(trimmed).to_string());
        }
    }
    sections.extend(current);

    sections.sort_by_key(|section| Reverse(section.date));
    sections
}

pub fn unseen(sections: &[Section], seen: Option<NaiveDate>) -> &[Section] {
    let count = match seen {
        None => sections.len().min(1),
        Some(seen) => sections
            .iter()
            .take_while(|section| section.date > seen)
            .count(),
    };
    &sections[..count]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(text: &str) -> NaiveDate {
        NaiveDate::parse_from_str(text, DATE_FORMAT).unwrap()
    }

    fn dates(sections: &[Section]) -> Vec<NaiveDate> {
        sections.iter().map(|section| section.date).collect()
    }

    const TWO_RELEASES: &str = "# Nouveautés

## 2026-10-12

- Ancienne

## 2026-10-26

- Première
- Deuxième
  sur deux lignes
";

    #[test]
    fn test_sections_split_on_dated_headings_newest_first() {
        let sections = parse(TWO_RELEASES);
        assert_eq!(dates(&sections), [date("2026-10-26"), date("2026-10-12")]);
        assert_eq!(sections[0].items, ["Première", "Deuxième sur deux lignes"]);
        assert_eq!(sections[1].items, ["Ancienne"]);
    }

    #[test]
    fn test_malformed_headings_are_ignored_with_their_bullets() {
        let sections = parse(
            "## 2026-10-26\n- Gardée\n## Bientôt\n- Perdue\n## 2026-13-01\n- Perdue\n## 26/10/2026\n- Perdue\n",
        );
        assert_eq!(dates(&sections), [date("2026-10-26")]);
        assert_eq!(sections[0].items, ["Gardée"]);
    }

    #[test]
    fn test_old_save_without_seen_date_shows_newest_section_only() {
        let sections = parse(TWO_RELEASES);
        assert_eq!(dates(unseen(&sections, None)), [date("2026-10-26")]);
    }

    #[test]
    fn test_every_section_after_the_seen_date_is_unseen() {
        let sections = parse(TWO_RELEASES);
        assert_eq!(
            dates(unseen(&sections, Some(date("2026-10-01")))),
            [date("2026-10-26"), date("2026-10-12")]
        );
        assert_eq!(
            dates(unseen(&sections, Some(date("2026-10-12")))),
            [date("2026-10-26")]
        );
        assert!(unseen(&sections, Some(date("2026-10-26"))).is_empty());
    }

    #[test]
    fn test_embedded_notes_have_a_dated_section() {
        let sections = load();
        assert!(!sections.is_empty());
        assert!(sections.iter().all(|section| !section.items.is_empty()));
    }
}
