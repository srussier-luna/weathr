use super::{Line, MUTED, Segment, width};
use crate::app_state::AppState;
use crossterm::style::Color;

const PINK: Color = Color::Rgb {
    r: 255,
    g: 120,
    b: 190,
};
const GAP: &str = "   ";
const COMPACT_GAP: &str = "  ";

enum Item {
    Key(char, &'static str),
    Counter(char, String),
    Alert(char, String),
}

impl Item {
    fn alert(key: char, label: &str) -> Self {
        Self::Alert(key, format!("{label}\u{202F}!"))
    }

    fn text(&self, is_compact: bool) -> String {
        match self {
            Self::Key(key, _) if is_compact => format!("[{key}]"),
            Self::Key(key, label) => format!("[{key}] {label}"),
            Self::Counter(key, label) | Self::Alert(key, label) => format!("[{key}] {label}"),
        }
    }

    fn color(&self, is_lit: bool) -> Color {
        match self {
            Self::Alert(..) if is_lit => PINK,
            _ => MUTED,
        }
    }
}

pub fn line(state: &AppState, is_lit: bool, max_width: usize) -> Line {
    let items = items(state);
    let full = join(&items, false, is_lit);
    if width(&full) <= max_width {
        full
    } else {
        join(&items, true, is_lit)
    }
}

fn items(state: &AppState) -> Vec<Item> {
    let mail = if state.is_mail_waiting {
        Item::alert('c', "Lettre")
    } else {
        Item::Key('c', "Courrier")
    };
    let finds = state.finds_hint.map(|hint| Item::alert('r', hint));
    let badges = Item::Counter(
        'b',
        format!("Badges {}/{}", state.badges_earned, state.badges_total),
    );
    [
        Some(mail),
        finds,
        Some(badges),
        Some(Item::Key('h', "Aide")),
        Some(Item::Key('q', "Quitter")),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn join(items: &[Item], is_compact: bool, is_lit: bool) -> Line {
    let gap = if is_compact { COMPACT_GAP } else { GAP };
    let mut line = Line::new();
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            line.push(Segment::new(gap, MUTED));
        }
        line.push(Segment::new(item.text(is_compact), item.color(is_lit)));
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LocationDisplay;
    use crate::hud::text;
    use crate::weather::{WeatherLocation, WeatherUnits};

    fn state() -> AppState {
        let location = WeatherLocation {
            latitude: 0.0,
            longitude: 0.0,
            elevation: None,
        };
        let mut state = AppState::new(
            location,
            None,
            LocationDisplay::default(),
            false,
            WeatherUnits::default(),
        );
        state.set_progress_summary(false, 3, 12);
        state
    }

    fn colors(line: &Line) -> Vec<(String, Color)> {
        line.iter()
            .filter(|segment| !segment.text.trim().is_empty())
            .map(|segment| (segment.text.clone(), segment.color))
            .collect()
    }

    #[test]
    fn test_idle_toolbar_lists_every_key_in_grey() {
        let line = line(&state(), true, 66);

        assert_eq!(
            text(&line),
            "[c] Courrier   [b] Badges 3/12   [h] Aide   [q] Quitter"
        );
        assert!(line.iter().all(|segment| segment.color == MUTED));
    }

    #[test]
    fn test_waiting_letter_blinks_pink_and_grey() {
        let mut state = state();
        state.set_progress_summary(true, 3, 12);

        let lit = line(&state, true, 66);
        assert_eq!(colors(&lit)[0], ("[c] Lettre\u{202F}!".to_string(), PINK));
        let dim = line(&state, false, 66);
        assert_eq!(colors(&dim)[0], ("[c] Lettre\u{202F}!".to_string(), MUTED));
    }

    #[test]
    fn test_visible_find_comes_right_after_the_mail_key() {
        let mut state = state();
        state.set_finds_hint(Some("Objet"));

        let line = line(&state, true, 76);
        assert_eq!(
            text(&line),
            "[c] Courrier   [r] Objet\u{202F}!   [b] Badges 3/12   [h] Aide   [q] Quitter"
        );
        assert_eq!(colors(&line)[1].1, PINK);
        assert_eq!(colors(&line)[0].1, MUTED);
    }

    #[test]
    fn test_crowded_toolbar_fits_70_columns() {
        let mut state = state();
        state.set_progress_summary(true, 54, 54);
        state.set_finds_hint(Some("Objet et bouteille"));

        let line = line(&state, true, 66);
        assert!(width(&line) <= 66, "{}", text(&line));
        assert_eq!(
            text(&line),
            "[c] Lettre\u{202F}!  [r] Objet et bouteille\u{202F}!  [b] Badges 54/54  [h]  [q]"
        );
    }
}
