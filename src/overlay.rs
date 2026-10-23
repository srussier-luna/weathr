use crate::badges::{BADGES, Badge, BadgeKind, KINDS};
use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::collections::BTreeSet;
use std::io;

const MAX_BOX_WIDTH: u16 = 66;
const MAX_BADGES_WIDTH: u16 = 80;
const MAX_BADGE_COLUMNS: usize = 3;
const HIDDEN_BADGE: &str = "???";
const BADGES_FOOTER: &str = "[b] ou [Echap] fermer";

pub enum Line {
    Text(String),
    Preformatted(String),
    Blank,
}

pub struct View {
    pub title: String,
    pub lines: Vec<Line>,
    pub footer: String,
}

pub fn render_view(renderer: &mut TerminalRenderer, view: &View) -> io::Result<()> {
    let box_width = box_width(renderer);
    let inner = inner_width(box_width);
    let lines: Vec<String> = view
        .lines
        .iter()
        .flat_map(|line| match line {
            Line::Text(text) => wrap(text, inner),
            Line::Preformatted(text) => vec![text.chars().take(inner).collect()],
            Line::Blank => vec![String::new()],
        })
        .collect();
    render_box(renderer, &view.title, &lines, &view.footer, box_width)
}

pub fn render_badges(
    renderer: &mut TerminalRenderer,
    earned: &BTreeSet<String>,
    streak: u32,
) -> io::Result<()> {
    let box_width = MAX_BADGES_WIDTH.min(renderer.get_size().0.saturating_sub(4));
    let inner = inner_width(box_width);
    let plural = if streak > 1 { "s" } else { "" };
    let mut lines = vec![format!("Série en cours : {streak} jour{plural} d'affilée")];

    for kind in KINDS {
        let badges: Vec<_> = BADGES.iter().filter(|badge| badge.kind == kind).collect();
        let count = badges
            .iter()
            .filter(|badge| earned.contains(badge.id))
            .count();
        lines.push(String::new());
        lines.push(format!("{} ({}/{})", kind.title(), count, badges.len()));

        let cells: Vec<String> = badges
            .iter()
            .map(|badge| badge_cell(badge, earned.contains(badge.id)))
            .collect();
        let longest_cell = cells
            .iter()
            .map(|cell| cell.chars().count() + 1)
            .max()
            .unwrap_or(1);
        let columns = (inner / longest_cell).clamp(1, MAX_BADGE_COLUMNS);
        let column_width = inner / columns;

        for row in cells.chunks(columns) {
            let line: String = row
                .iter()
                .map(|cell| format!("{cell:<width$} ", width = column_width.saturating_sub(1)))
                .collect();
            lines.push(line.trim_end().to_string());
        }
    }

    let total_earned = BADGES
        .iter()
        .filter(|badge| earned.contains(badge.id))
        .count();
    let title = format!("Badges {}/{}", total_earned, BADGES.len());
    render_box(renderer, &title, &lines, BADGES_FOOTER, box_width)
}

fn badge_cell(badge: &Badge, is_earned: bool) -> String {
    let mark = if is_earned { 'x' } else { ' ' };
    match (is_earned, badge.kind, badge.hint) {
        (false, BadgeKind::Secret, _) => format!("[{mark}] {HIDDEN_BADGE}"),
        (true, _, Some(hint)) => format!("[{mark}] {} ({hint})", badge.name),
        _ => format!("[{mark}] {}", badge.name),
    }
}

fn box_width(renderer: &TerminalRenderer) -> u16 {
    MAX_BOX_WIDTH.min(renderer.get_size().0.saturating_sub(4))
}

fn inner_width(box_width: u16) -> usize {
    box_width.saturating_sub(4) as usize
}

fn render_box(
    renderer: &mut TerminalRenderer,
    title: &str,
    lines: &[String],
    footer: &str,
    box_width: u16,
) -> io::Result<()> {
    let (width, height) = renderer.get_size();
    let inner = inner_width(box_width);
    let box_height = lines.len() as u16 + 6;
    let x = width.saturating_sub(box_width) / 2;
    let y = (height.saturating_sub(box_height) / 2).max(2);
    let border = format!("+{}+", "-".repeat(box_width.saturating_sub(2) as usize));
    let row = |text: &str| format!("| {text:<inner$} |");

    let mut rows = vec![(border.clone(), Color::Grey), (row(title), Color::Yellow)];
    rows.push((row(""), Color::White));
    rows.extend(lines.iter().map(|line| (row(line), Color::White)));
    rows.push((row(""), Color::White));
    rows.push((row(footer), Color::DarkGrey));
    rows.push((border, Color::Grey));

    for (offset, (text, color)) in rows.iter().enumerate() {
        renderer.render_line_colored(x, y + offset as u16, text, *color)?;
    }
    Ok(())
}

fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let needed =
            current.chars().count() + word.chars().count() + usize::from(!current.is_empty());
        if needed > width && !current.is_empty() {
            lines.push(std::mem::take(&mut current));
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_badges_explain_themselves_once_earned() {
        let patience = BADGES.iter().find(|badge| badge.id == "patience").unwrap();
        let clear = BADGES.iter().find(|badge| badge.id == "clear").unwrap();
        assert_eq!(badge_cell(patience, false), "[ ] ???");
        assert_eq!(
            badge_cell(patience, true),
            "[x] Patience (weathr ouvert 1 h d'affilée)"
        );
        assert_eq!(badge_cell(clear, true), "[x] Ciel dégagé");
        assert!(
            BADGES
                .iter()
                .filter(|badge| badge.kind == BadgeKind::Secret)
                .all(|badge| badge.hint.is_some())
        );
    }

    #[test]
    fn test_wrap_respects_width() {
        let lines = wrap("Bienvenue dans la maison au bout du chemin", 12);
        assert_eq!(
            lines,
            ["Bienvenue", "dans la", "maison au", "bout du", "chemin"]
        );
    }
}
