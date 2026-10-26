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
const FOOTER_GAP: &str = "  ";
const FOLDER_TAB: &str = "/  Archives  \\";
const STAMP: &str = "[~]";

type Piece = (u16, String, Color);

pub enum Line {
    Text(String),
    Preformatted(String),
    Centered(String),
    Rule,
    Blank,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frame {
    Plain,
    Parchment,
    Newspaper,
    Postcard,
    Folder,
    Recipe,
    Waves,
    Photo,
    Stars,
}

impl Frame {
    fn color(self) -> Option<Color> {
        let (r, g, b) = match self {
            Self::Plain => return None,
            Self::Parchment => (222, 196, 140),
            Self::Newspaper => (205, 205, 195),
            Self::Postcard => (190, 170, 235),
            Self::Folder => (230, 160, 80),
            Self::Recipe => (240, 140, 140),
            Self::Waves => (100, 180, 225),
            Self::Photo => (150, 210, 150),
            Self::Stars => (255, 210, 90),
        };
        Some(Color::Rgb { r, g, b })
    }

    fn fit(self, width: u16) -> u16 {
        match self {
            Self::Stars if width % 2 == 0 => width.saturating_sub(1),
            _ => width,
        }
    }

    fn edges(self, row: usize) -> (u16, &'static str, &'static str) {
        match self {
            Self::Plain | Self::Postcard | Self::Folder | Self::Photo => (0, "| ", " |"),
            Self::Parchment if row % 2 == 0 => (0, "(  ", "  )"),
            Self::Parchment => (1, ") ", " ("),
            Self::Newspaper => (0, "|| ", " ||"),
            Self::Recipe => (0, "o| ", " |"),
            Self::Waves => (0, "  ", "  "),
            Self::Stars => (0, "* ", " *"),
        }
    }

    fn inner_width(self, box_width: u16) -> usize {
        let (indent, left, right) = self.edges(0);
        usize::from(box_width).saturating_sub(usize::from(indent) + left.len() + right.len())
    }

    fn top(self, width: usize) -> Vec<(u16, String)> {
        match self {
            Self::Plain | Self::Postcard => vec![(0, rule('+', "-", '+', width))],
            Self::Parchment => vec![(1, rule('.', "~", '.', width.saturating_sub(2)))],
            Self::Newspaper => vec![(0, "=".repeat(width))],
            Self::Folder => vec![
                (1, "_".repeat(FOLDER_TAB.len() - 2)),
                (
                    0,
                    format!(
                        "{FOLDER_TAB}{}",
                        "_".repeat(width.saturating_sub(FOLDER_TAB.len() + 1))
                    ),
                ),
            ],
            Self::Recipe => vec![(1, rule('+', "-", '+', width.saturating_sub(1)))],
            Self::Waves => vec![(0, "~^".chars().cycle().take(width).collect())],
            Self::Photo => vec![(0, rule('/', "-", '\\', width))],
            Self::Stars => vec![(0, "*~".chars().cycle().take(width).collect())],
        }
    }

    fn bottom(self, width: usize) -> Vec<(u16, String)> {
        match self {
            Self::Parchment => vec![(1, rule('\'', "~", '\'', width.saturating_sub(2)))],
            Self::Folder => vec![(0, rule('|', "_", '|', width))],
            Self::Photo => vec![(0, rule('\\', "-", '/', width))],
            _ => self.top(width),
        }
    }

    fn title_lines(self, title: &str, inner: usize) -> Vec<String> {
        match self {
            Self::Newspaper => wrap(title, inner)
                .iter()
                .map(|line| format!("{line:^inner$}"))
                .collect(),
            Self::Postcard => {
                let width = inner.saturating_sub(STAMP.len() + 1);
                let mut lines = wrap(title, width);
                if let Some(first) = lines.first_mut() {
                    *first = format!("{first:<width$} {STAMP}");
                }
                lines
            }
            _ => wrap(title, inner),
        }
    }
}

pub struct View {
    pub title: String,
    pub lines: Vec<Line>,
    pub footer: String,
    pub frame: Frame,
}

pub fn render_view(renderer: &mut TerminalRenderer, view: &View) -> io::Result<()> {
    let box_width = view.frame.fit(box_width(renderer));
    let inner = view.frame.inner_width(box_width);
    let lines: Vec<String> = view
        .lines
        .iter()
        .flat_map(|line| match line {
            Line::Text(text) => wrap(text, inner),
            Line::Preformatted(text) => vec![text.chars().take(inner).collect()],
            Line::Centered(text) if text.chars().count() <= inner => {
                vec![format!("{text:^inner$}")]
            }
            Line::Centered(text) => wrap(text, inner)
                .iter()
                .map(|line| format!("{line:^inner$}"))
                .collect(),
            Line::Rule => vec!["-".repeat(inner)],
            Line::Blank => vec![String::new()],
        })
        .collect();
    render_box(
        renderer,
        view.frame,
        &view.title,
        &lines,
        &view.footer,
        box_width,
    )
}

pub fn render_badges(
    renderer: &mut TerminalRenderer,
    earned: &BTreeSet<String>,
    streak: u32,
) -> io::Result<()> {
    let box_width = Frame::Stars.fit(MAX_BADGES_WIDTH.min(renderer.get_size().0.saturating_sub(4)));
    let inner = Frame::Stars.inner_width(box_width);
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
    render_box(
        renderer,
        Frame::Stars,
        &title,
        &lines,
        BADGES_FOOTER,
        box_width,
    )
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

fn rule(left: char, fill: &str, right: char, width: usize) -> String {
    format!("{left}{}{right}", fill.repeat(width.saturating_sub(2)))
}

fn render_box(
    renderer: &mut TerminalRenderer,
    frame: Frame,
    title: &str,
    lines: &[String],
    footer: &str,
    box_width: u16,
) -> io::Result<()> {
    let (width, height) = renderer.get_size();
    let rows = frame_rows(frame, title, lines, footer, box_width);
    let x = width.saturating_sub(box_width) / 2;
    let y = (height.saturating_sub(rows.len() as u16) / 2).max(2);

    for (offset, row) in rows.iter().enumerate() {
        for (column, text, color) in row {
            renderer.render_line_colored(x + column, y + offset as u16, text, *color)?;
        }
    }
    Ok(())
}

fn frame_rows(
    frame: Frame,
    title: &str,
    lines: &[String],
    footer: &str,
    box_width: u16,
) -> Vec<Vec<Piece>> {
    let width = usize::from(box_width);
    let inner = frame.inner_width(box_width);
    let border = frame.color().unwrap_or(Color::Grey);
    let rule_rows = |rules: Vec<(u16, String)>| {
        rules
            .into_iter()
            .map(move |(indent, text)| vec![(indent, text, border)])
    };

    let mut body: Vec<(String, Color)> = Vec::new();
    if !title.is_empty() {
        body.extend(
            frame
                .title_lines(title, inner)
                .into_iter()
                .map(|line| (line, Color::Yellow)),
        );
        body.push((String::new(), Color::White));
    }
    body.extend(lines.iter().map(|line| (line.clone(), Color::White)));
    body.push((String::new(), Color::White));
    body.extend(
        pack(footer.split(FOOTER_GAP), FOOTER_GAP, inner)
            .into_iter()
            .map(|line| (line, Color::DarkGrey)),
    );

    let mut rows: Vec<Vec<Piece>> = rule_rows(frame.top(width)).collect();
    rows.extend(body.into_iter().enumerate().map(|(index, (text, color))| {
        let (indent, left, right) = frame.edges(index);
        let edge = frame.color().unwrap_or(color);
        let start = indent + left.len() as u16;
        vec![
            (indent, left.to_string(), edge),
            (start, format!("{text:<inner$}"), color),
            (start + inner as u16, right.to_string(), edge),
        ]
    }));
    rows.extend(rule_rows(frame.bottom(width)));
    rows
}

fn wrap(text: &str, width: usize) -> Vec<String> {
    pack(text.split_whitespace(), " ", width)
}

fn pack<'a>(words: impl Iterator<Item = &'a str>, gap: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in words {
        let needed = current.chars().count()
            + word.chars().count()
            + if current.is_empty() {
                0
            } else {
                gap.chars().count()
            };
        if needed > width && !current.is_empty() {
            lines.push(std::mem::take(&mut current));
        }
        if !current.is_empty() {
            current.push_str(gap);
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

    const FRAMES: [Frame; 9] = [
        Frame::Plain,
        Frame::Parchment,
        Frame::Newspaper,
        Frame::Postcard,
        Frame::Folder,
        Frame::Recipe,
        Frame::Waves,
        Frame::Photo,
        Frame::Stars,
    ];

    fn draw(frame: Frame, title: &str, lines: &[&str], footer: &str, width: u16) -> Vec<String> {
        let lines: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
        frame_rows(frame, title, &lines, footer, width)
            .iter()
            .map(|row| {
                let mut cells = vec![' '; usize::from(width)];
                for (column, text, _) in row {
                    for (offset, ch) in text.chars().enumerate() {
                        cells[usize::from(*column) + offset] = ch;
                    }
                }
                cells.into_iter().collect::<String>().trim_end().to_string()
            })
            .collect()
    }

    #[test]
    fn test_plain_frame_keeps_the_old_box() {
        let rows = draw(Frame::Plain, "Commandes", &["[h] aide"], "[Echap]", 20);
        assert_eq!(
            rows,
            [
                "+------------------+",
                "| Commandes        |",
                "|                  |",
                "| [h] aide         |",
                "|                  |",
                "| [Echap]          |",
                "+------------------+",
            ]
        );
    }

    #[test]
    fn test_parchment_frame_has_torn_edges() {
        let rows = draw(Frame::Parchment, "Lettre 9/12", &["Wouf !"], "[c]", 20);
        assert_eq!(
            rows,
            [
                " .~~~~~~~~~~~~~~~~.",
                "(  Lettre 9/12     )",
                " )                (",
                "(  Wouf !          )",
                " )                (",
                "(  [c]             )",
                " '~~~~~~~~~~~~~~~~'",
            ]
        );
    }

    #[test]
    fn test_each_frame_draws_its_own_border() {
        let gazette = draw(Frame::Newspaper, "LA GAZETTE · lundi 17 août", &[], "", 36);
        assert_eq!(gazette[0], "=".repeat(36));
        assert_eq!(gazette[1], "||   LA GAZETTE · lundi 17 août   ||");

        let postcard = draw(Frame::Postcard, "Carte postale 1/3 : Lyon", &[], "", 36);
        assert_eq!(postcard[1], "| Carte postale 1/3 : Lyon     [~] |");

        let archives = draw(Frame::Folder, "Archives", &[], "", 20);
        assert_eq!(archives[0], " ____________");
        assert_eq!(archives[1], "/  Archives  \\_____");
        assert_eq!(archives.last().unwrap(), "|__________________|");

        let recipe = draw(Frame::Recipe, "Recette", &[], "", 20);
        assert_eq!(recipe[0], " +-----------------+");
        assert!(
            recipe[1..recipe.len() - 1]
                .iter()
                .all(|row| row.starts_with("o|"))
        );

        let bottle = draw(Frame::Waves, "Message", &[], "", 12);
        assert_eq!(bottle[0], "~^~^~^~^~^~^");
        assert_eq!(bottle[1], "  Message");

        let album = draw(Frame::Photo, "Album", &[], "", 10);
        assert_eq!(album[0], "/--------\\");
        assert_eq!(album.last().unwrap(), "\\--------/");

        let badges = draw(Frame::Stars, "Badges", &[], "", Frame::Stars.fit(12));
        assert_eq!(badges[0], "*~*~*~*~*~*");
        assert_eq!(badges[1], "* Badges  *");
    }

    #[test]
    fn test_text_stays_inside_every_frame_at_80_columns() {
        let title = "Carte postale 12/40 : Rio de Janeiro, Brésil";
        let body = "Wouf ! C'est moi qui dors au pied de ton lit quand il pleut, tout au chaud.";
        let footer = "[c] suivante  [<] [>] parcourir  [l] sommaire  [Echap] fermer";
        for frame in FRAMES {
            let width = frame.fit(MAX_BOX_WIDTH.min(80 - 4));
            let inner = frame.inner_width(width);
            let lines = wrap(body, inner);
            for row in frame_rows(frame, title, &lines, footer, width) {
                for (column, text, _) in &row {
                    let end = usize::from(*column) + text.chars().count();
                    assert!(end <= usize::from(width), "{frame:?}: {text}");
                }
                if let [_, (_, text, _), _] = row.as_slice() {
                    assert_eq!(text.chars().count(), inner, "{frame:?}: {text}");
                }
            }
        }
    }
}
