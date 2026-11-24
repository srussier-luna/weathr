use crate::badges::{self, BADGES, Badge, BadgeKind, KINDS, SAM_TRAIL};
use crate::hud::PINK;
use crate::render::TerminalRenderer;
use chrono::Local;
use crossterm::style::Color;
use std::collections::BTreeSet;
use std::io;

const MAX_BOX_WIDTH: u16 = 66;
const MAX_BADGES_WIDTH: u16 = 80;
const MAX_BADGE_COLUMNS: usize = 3;
const HIDDEN_BADGE: &str = "???";
const MARK_END: &str = "] ";
const BADGES_FOOTER: &str = "[b] ou [Echap] fermer";
const FOOTER_GAP: &str = "  ";
const FOLDER_TAB: &str = "/  Archives  \\";
const STAMP: &str = "[~]";
const WET_STAIN: &str = "~~~~";
const WET_TITLE_INDENT: &str = "  ";
const FIRST_DROP: usize = 4;
const DROP_GAP: usize = 7;
const DROPS: [&str; 2] = ["'", "."];
const WET_CUT: [&str; 3] = ["/", " /", "  /"];
const FLOPPY_LABEL: &str = "-[#]";
const ROOFS: &str = "/\\_";
const MAX_BUBBLE_WIDTH: u16 = 40;
const BUBBLE_GAP: &str = ". .";
const BUBBLE_TAIL: &str = "V";
const TAIL_ROWS_ABOVE_SPEAKER: u16 = 2;
const CHEVRON_STEP_MILLIS: i64 = 500;
const CHEVRON_GAP: u16 = 1;

type Piece = (u16, String, Color);

#[derive(Debug, PartialEq, Eq)]
struct Highlight {
    line: usize,
    start: usize,
    len: usize,
}

struct BadgeLines {
    lines: Vec<String>,
    highlights: Vec<Highlight>,
}

#[derive(Clone, Copy)]
struct BoxArea {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl Side {
    fn chevron(self) -> char {
        match self {
            Self::Left => '«',
            Self::Right => '»',
        }
    }
}

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
    Wet,
    Newspaper,
    Postcard,
    Folder,
    Recipe,
    Waves,
    Photo,
    Stars,
    Bubble,
    Notice,
    Floppy,
    Roofs,
}

impl Frame {
    fn color(self) -> Option<Color> {
        let (r, g, b) = match self {
            Self::Plain | Self::Bubble | Self::Notice => return None,
            Self::Parchment => (222, 196, 140),
            Self::Wet => (150, 200, 235),
            Self::Newspaper => (205, 205, 195),
            Self::Postcard => (190, 170, 235),
            Self::Folder => (230, 160, 80),
            Self::Recipe => (240, 140, 140),
            Self::Waves => (100, 180, 225),
            Self::Photo => (150, 210, 150),
            Self::Stars => (255, 210, 90),
            Self::Floppy => (150, 165, 190),
            Self::Roofs => (205, 95, 70),
        };
        Some(Color::Rgb { r, g, b })
    }

    fn border(self) -> Color {
        self.color().unwrap_or(Color::Grey)
    }

    fn fit(self, width: u16) -> u16 {
        match self {
            Self::Stars if width % 2 == 0 => width.saturating_sub(1),
            _ => width,
        }
    }

    fn edges(self, row: usize, rows_below: Option<usize>) -> (u16, &'static str, &'static str) {
        match self {
            Self::Plain
            | Self::Postcard
            | Self::Folder
            | Self::Photo
            | Self::Bubble
            | Self::Floppy
            | Self::Roofs => (0, "| ", " |"),
            Self::Parchment if row % 2 == 0 => (0, "(  ", "  )"),
            Self::Parchment => (1, ") ", " ("),
            Self::Wet => (
                0,
                match row {
                    0 => "(~~",
                    1 => "|~ ",
                    _ => "|  ",
                },
                rows_below
                    .and_then(|rows| WET_CUT.get(rows))
                    .copied()
                    .unwrap_or("  |"),
            ),
            Self::Newspaper => (0, "|| ", " ||"),
            Self::Recipe => (0, "o| ", " |"),
            Self::Waves => (0, "  ", "  "),
            Self::Stars => (0, "* ", " *"),
            Self::Notice => (0, "║ ", " ║"),
        }
    }

    fn inner_width(self, box_width: u16) -> usize {
        let (indent, left, right) = self.edges(0, None);
        usize::from(box_width)
            .saturating_sub(usize::from(indent) + left.chars().count() + right.chars().count())
    }

    fn top(self, width: usize) -> Vec<(u16, String)> {
        match self {
            Self::Plain | Self::Postcard => vec![(0, rule('+', "-", '+', width))],
            Self::Parchment => vec![(1, rule('.', "~", '.', width.saturating_sub(2)))],
            Self::Wet => vec![(
                0,
                format!(
                    "+{WET_STAIN}{}+",
                    "-".repeat(width.saturating_sub(WET_STAIN.len() + 2))
                ),
            )],
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
            Self::Bubble => vec![(0, rule('.', "-", '.', width))],
            Self::Notice => vec![(0, rule('╔', "═", '╗', width))],
            Self::Floppy => vec![(
                0,
                format!(
                    "+{FLOPPY_LABEL}{}+",
                    "-".repeat(width.saturating_sub(FLOPPY_LABEL.len() + 2))
                ),
            )],
            Self::Roofs => vec![(0, ROOFS.chars().cycle().take(width).collect())],
        }
    }

    fn bottom(self, width: usize) -> Vec<(u16, String)> {
        match self {
            Self::Parchment => vec![(1, rule('\'', "~", '\'', width.saturating_sub(2)))],
            Self::Wet => {
                let drops = self.drops(width);
                let edge = (1..wet_bottom_end(width))
                    .map(|column| {
                        if drops.iter().any(|(drop, _)| usize::from(*drop) == column) {
                            ','
                        } else {
                            '-'
                        }
                    })
                    .collect::<String>();
                vec![(0, format!("+{edge}'"))]
            }
            Self::Folder | Self::Roofs => vec![(0, rule('|', "_", '|', width))],
            Self::Photo => vec![(0, rule('\\', "-", '/', width))],
            Self::Bubble => vec![(0, rule('\'', "-", '\'', width))],
            Self::Notice => vec![(0, rule('╚', "═", '╝', width))],
            Self::Floppy => vec![(0, rule('+', "-", '+', width))],
            _ => self.top(width),
        }
    }

    fn drops(self, width: usize) -> Vec<(u16, &'static str)> {
        match self {
            Self::Wet => (FIRST_DROP..wet_bottom_end(width))
                .step_by(DROP_GAP)
                .zip(DROPS.into_iter().cycle())
                .map(|(column, drop)| (column as u16, drop))
                .collect(),
            _ => Vec::new(),
        }
    }

    fn title_lines(self, title: &str, inner: usize) -> Vec<String> {
        match self {
            Self::Wet => {
                let width = inner.saturating_sub(WET_TITLE_INDENT.len());
                let lines = wrap(title, width);
                if lines.iter().all(|line| line.chars().count() <= width) {
                    lines
                        .into_iter()
                        .map(|line| format!("{WET_TITLE_INDENT}{line}"))
                        .collect()
                } else {
                    wrap(title, inner)
                }
            }
            Self::Newspaper => center(title, inner),
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

pub fn text_width(renderer: &TerminalRenderer, frame: Frame) -> usize {
    frame.inner_width(frame.fit(box_width(renderer)))
}

pub fn lay_out(lines: &[Line], width: usize) -> Vec<String> {
    lines
        .iter()
        .flat_map(|line| match line {
            Line::Text(text) => wrap(text, width),
            Line::Preformatted(text) => vec![text.chars().take(width).collect()],
            Line::Centered(text) => center(text, width),
            Line::Rule => vec!["-".repeat(width)],
            Line::Blank => vec![String::new()],
        })
        .collect()
}

pub fn render_view(renderer: &mut TerminalRenderer, view: &View) -> io::Result<()> {
    render_view_box(renderer, view)?;
    Ok(())
}

pub fn blink_tick() -> usize {
    (Local::now().timestamp_millis() / CHEVRON_STEP_MILLIS) as usize
}

pub fn is_blink_lit() -> bool {
    blink_tick() % 2 == 0
}

pub fn render_with_chevrons(
    renderer: &mut TerminalRenderer,
    view: &View,
    has_previous: bool,
    has_next: bool,
) -> io::Result<()> {
    let area = render_view_box(renderer, view)?;
    let tick = blink_tick();
    let width = renderer.get_size().0;
    for (side, is_shown) in [(Side::Left, has_previous), (Side::Right, has_next)] {
        if is_shown && let Some((x, y)) = chevron_cell(area, side, width, tick) {
            renderer.render_char(x, y, side.chevron(), Color::Yellow)?;
        }
    }
    Ok(())
}

fn render_view_box(renderer: &mut TerminalRenderer, view: &View) -> io::Result<BoxArea> {
    let box_width = view.frame.fit(box_width(renderer));
    let lines = lay_out(&view.lines, view.frame.inner_width(box_width));
    render_box(
        renderer,
        view.frame,
        &view.title,
        &lines,
        &[],
        &view.footer,
        box_width,
    )
}

fn chevron_cell(area: BoxArea, side: Side, terminal_width: u16, tick: usize) -> Option<(u16, u16)> {
    let offset = CHEVRON_GAP + (tick % 2) as u16;
    let x = match side {
        Side::Left => (area.x > CHEVRON_GAP + 1).then(|| area.x - 1 - offset),
        Side::Right => {
            let right = area.x + area.width;
            (right + CHEVRON_GAP + 1 < terminal_width).then_some(right + offset)
        }
    }?;
    Some((x, area.y + area.height / 2))
}

pub fn render_bubble(
    renderer: &mut TerminalRenderer,
    text: &str,
    footer: &str,
    speaker: Option<(u16, u16)>,
) -> io::Result<()> {
    let (width, height) = renderer.get_size();
    let box_width = MAX_BUBBLE_WIDTH.min(width.saturating_sub(4));
    let Some((speaker_x, speaker_y)) = speaker else {
        let rows = bubble_rows(text, footer, box_width, None);
        let x = width.saturating_sub(box_width) / 2;
        let y = height.saturating_sub(rows.len() as u16 + 1).max(2);
        return draw_rows(renderer, &rows, x, y);
    };
    let x = speaker_x
        .saturating_sub(box_width / 2)
        .min(width.saturating_sub(box_width));
    let tail = speaker_x
        .saturating_sub(x)
        .clamp(2, box_width.saturating_sub(3).max(2));
    let rows = bubble_rows(text, footer, box_width, Some(tail));
    let tail_y = speaker_y.saturating_sub(TAIL_ROWS_ABOVE_SPEAKER);
    let y = (tail_y + 1).saturating_sub(rows.len() as u16).max(2);
    draw_rows(renderer, &rows, x, y)
}

fn bubble_rows(text: &str, footer: &str, box_width: u16, tail: Option<u16>) -> Vec<Vec<Piece>> {
    let frame = Frame::Bubble;
    let lines = wrap(text, frame.inner_width(box_width));
    let mut rows = frame_rows(frame, "", &lines, &[], footer, box_width);
    if let Some(tail) = tail {
        if let Some(bottom) = rows.last_mut() {
            bottom.push((tail - 1, BUBBLE_GAP.to_string(), frame.border()));
        }
        rows.push(vec![(tail, BUBBLE_TAIL.to_string(), frame.border())]);
    }
    rows
}

pub fn render_badges(
    renderer: &mut TerminalRenderer,
    earned: &BTreeSet<String>,
    unseen: &BTreeSet<String>,
    streak: u32,
    sam_cities: usize,
) -> io::Result<()> {
    let box_width = Frame::Stars.fit(MAX_BADGES_WIDTH.min(renderer.get_size().0.saturating_sub(4)));
    let BadgeLines { lines, highlights } = badge_lines(
        earned,
        unseen,
        streak,
        sam_cities,
        Frame::Stars.inner_width(box_width),
    );
    let highlights = if is_blink_lit() { &highlights[..] } else { &[] };
    render_box(
        renderer,
        Frame::Stars,
        &badges_title(earned),
        &lines,
        highlights,
        BADGES_FOOTER,
        box_width,
    )?;
    Ok(())
}

fn badges_title(earned: &BTreeSet<String>) -> String {
    let total_earned = badges::collection()
        .filter(|badge| earned.contains(badge.id))
        .count();
    format!("Badges {}/{}", total_earned, badges::collection().count())
}

fn badge_lines(
    earned: &BTreeSet<String>,
    unseen: &BTreeSet<String>,
    streak: u32,
    sam_cities: usize,
    inner: usize,
) -> BadgeLines {
    let plural = if streak > 1 { "s" } else { "" };
    let mut lines = vec![format!("Série en cours : {streak} jour{plural} d'affilée")];
    let mut highlights = Vec::new();

    for kind in KINDS {
        let badges: Vec<_> = BADGES.iter().filter(|badge| badge.kind == kind).collect();
        let count = badges
            .iter()
            .filter(|badge| earned.contains(badge.id))
            .count();
        let is_demerit = kind == BadgeKind::Demerit;
        if is_demerit && count == 0 {
            continue;
        }
        lines.push(String::new());
        lines.push(if is_demerit {
            kind.title().to_string()
        } else {
            format!("{} ({}/{})", kind.title(), count, badges.len())
        });

        let (own_rows, grid): (Vec<_>, Vec<_>) = badges
            .iter()
            .map(|badge| (*badge, earned.contains(badge.id)))
            .partition(|(badge, is_earned)| *is_earned && badge.id == SAM_TRAIL.id);
        let cells: Vec<(String, bool)> = grid
            .into_iter()
            .map(|(badge, is_earned)| {
                (
                    badge_cell(badge, is_earned, sam_cities),
                    unseen.contains(badge.id),
                )
            })
            .collect();
        let longest_cell = cells
            .iter()
            .map(|(cell, _)| cell.chars().count() + 1)
            .max()
            .unwrap_or(1);
        let columns = (inner / longest_cell).clamp(1, MAX_BADGE_COLUMNS);
        let column_width = inner / columns;

        for row in cells.chunks(columns) {
            let line: String = row
                .iter()
                .map(|(cell, _)| format!("{cell:<width$} ", width = column_width.saturating_sub(1)))
                .collect();
            highlights.extend(
                row.iter()
                    .enumerate()
                    .filter(|(_, (_, is_unseen))| *is_unseen)
                    .map(|(column, (cell, _))| Highlight {
                        line: lines.len(),
                        start: column * column_width,
                        len: cell.chars().count(),
                    }),
            );
            lines.push(line.trim_end().to_string());
        }
        for (badge, is_earned) in own_rows {
            for row in wrap_cell(&badge_cell(badge, is_earned, sam_cities), inner) {
                if unseen.contains(badge.id) {
                    highlights.push(Highlight {
                        line: lines.len(),
                        start: 0,
                        len: row.chars().count(),
                    });
                }
                lines.push(row);
            }
        }
    }
    BadgeLines { lines, highlights }
}

fn badge_cell(badge: &Badge, is_earned: bool, sam_cities: usize) -> String {
    let mark = if is_earned { 'x' } else { ' ' };
    match (is_earned, badge.kind, badge.hint) {
        (false, BadgeKind::Secret, _) => format!("[{mark}] {HIDDEN_BADGE}"),
        (true, _, _) if badge.id == SAM_TRAIL.id => format!(
            "[{mark}] {}",
            badges::sam_trail_line(sam_cities).unwrap_or_else(|| badge.name.to_string())
        ),
        (true, _, Some(hint)) => format!("[{mark}] {} ({hint})", badge.name),
        _ => format!("[{mark}] {}", badge.name),
    }
}

fn wrap_cell(cell: &str, width: usize) -> Vec<String> {
    if cell.chars().count() <= width {
        return vec![cell.to_string()];
    }
    let mark_width = cell
        .find(MARK_END)
        .map_or(0, |index| index + MARK_END.len());
    let (mark, text) = cell.split_at(mark_width);
    let indent = " ".repeat(mark_width);
    let text_width = width.saturating_sub(mark_width);
    let lines = match text.split_once(" (") {
        Some((name, hint)) => [
            wrap(name, text_width),
            wrap(&format!("({hint}"), text_width),
        ]
        .concat(),
        None => wrap(text, text_width),
    };
    lines
        .into_iter()
        .enumerate()
        .map(|(index, line)| {
            let prefix = if index == 0 { mark } else { &indent };
            format!("{prefix}{line}")
        })
        .collect()
}

fn box_width(renderer: &TerminalRenderer) -> u16 {
    MAX_BOX_WIDTH.min(renderer.get_size().0.saturating_sub(4))
}

fn wet_bottom_end(width: usize) -> usize {
    width.saturating_sub(WET_CUT.len() + 1)
}

fn rule(left: char, fill: &str, right: char, width: usize) -> String {
    format!("{left}{}{right}", fill.repeat(width.saturating_sub(2)))
}

fn render_box(
    renderer: &mut TerminalRenderer,
    frame: Frame,
    title: &str,
    lines: &[String],
    highlights: &[Highlight],
    footer: &str,
    box_width: u16,
) -> io::Result<BoxArea> {
    let (width, height) = renderer.get_size();
    let rows = frame_rows(frame, title, lines, highlights, footer, box_width);
    let area = BoxArea {
        x: width.saturating_sub(box_width) / 2,
        y: (height.saturating_sub(rows.len() as u16) / 2).max(2),
        width: box_width,
        height: rows.len() as u16,
    };
    draw_rows(renderer, &rows, area.x, area.y)?;
    Ok(area)
}

fn draw_rows(
    renderer: &mut TerminalRenderer,
    rows: &[Vec<Piece>],
    x: u16,
    y: u16,
) -> io::Result<()> {
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
    highlights: &[Highlight],
    footer: &str,
    box_width: u16,
) -> Vec<Vec<Piece>> {
    let width = usize::from(box_width);
    let inner = frame.inner_width(box_width);
    let border = frame.border();
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
    let first_body_line = body.len();
    body.extend(lines.iter().map(|line| (line.clone(), Color::White)));
    body.push((String::new(), Color::White));
    body.extend(
        pack(footer.split(FOOTER_GAP), FOOTER_GAP, inner)
            .into_iter()
            .map(|line| (line, Color::DarkGrey)),
    );

    let mut rows: Vec<Vec<Piece>> = rule_rows(frame.top(width)).collect();
    let first_line_row = rows.len() + first_body_line;
    let count = body.len();
    rows.extend(body.into_iter().enumerate().map(|(index, (text, color))| {
        let (indent, left, right) = frame.edges(index, Some(count - index - 1));
        let edge = frame.color().unwrap_or(color);
        let start = indent + left.chars().count() as u16;
        vec![
            (indent, left.to_string(), edge),
            (start, format!("{text:<inner$}"), color),
            (start + inner as u16, right.to_string(), edge),
        ]
    }));
    for highlight in highlights {
        let row = &mut rows[first_line_row + highlight.line];
        let (start, text, _) = &row[1];
        let cell = text
            .chars()
            .skip(highlight.start)
            .take(highlight.len)
            .collect();
        row.push((start + highlight.start as u16, cell, PINK));
    }
    rows.extend(rule_rows(frame.bottom(width)));
    let drops = frame.drops(width);
    if !drops.is_empty() {
        rows.push(
            drops
                .into_iter()
                .map(|(column, drop)| (column, drop.to_string(), border))
                .collect(),
        );
    }
    rows
}

pub fn wrap(text: &str, width: usize) -> Vec<String> {
    pack(text.split_ascii_whitespace(), " ", width)
}

fn center(text: &str, width: usize) -> Vec<String> {
    let lines = if text.chars().count() <= width {
        vec![text.to_string()]
    } else {
        wrap(text, width)
    };
    lines.iter().map(|line| format!("{line:^width$}")).collect()
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
        assert_eq!(badge_cell(patience, false, 0), "[ ] ???");
        assert_eq!(
            badge_cell(patience, true, 0),
            "[x] Patience (weathr ouvert 1 h d'affilée)"
        );
        assert_eq!(badge_cell(clear, true, 0), "[x] Ciel dégagé");
        assert!(
            BADGES
                .iter()
                .filter(|badge| badge.kind == BadgeKind::Secret)
                .all(|badge| badge.hint.is_some())
        );
    }

    #[test]
    fn test_cheater_family_shows_only_once_earned_and_never_counts() {
        let honest = BTreeSet::from(["rain".to_string()]);
        let cheater = BTreeSet::from(["rain".to_string(), badges::CHEATER.to_string()]);
        let total = BADGES.len() - 1;

        let lines = badge_lines(&honest, &BTreeSet::new(), 1, 0, 76).lines;
        assert!(!lines.iter().any(|line| line.contains("Mauvais points")));
        assert!(!lines.iter().any(|line| line.contains("Tricheur")));
        assert_eq!(badges_title(&honest), format!("Badges 1/{total}"));

        let lines = badge_lines(&cheater, &BTreeSet::new(), 1, 0, 76).lines;
        let family = lines
            .iter()
            .position(|line| line == "Mauvais points")
            .unwrap();
        assert_eq!(lines[family + 1], "[x] Tricheur (pris la main dans le sac)");
        assert_eq!(badges_title(&cheater), format!("Badges 1/{total}"));
    }

    #[test]
    fn test_sam_trail_shows_its_tier_once_earned() {
        assert_eq!(badge_cell(&SAM_TRAIL, false, 0), "[ ] ???");
        assert_eq!(
            badge_cell(&SAM_TRAIL, true, 20),
            "[x] Sur les traces de Sam · argent (20 villes, prochain palier\u{202F}: or à 30)"
        );
        assert_eq!(
            badge_cell(&SAM_TRAIL, true, 40),
            "[x] Sur les traces de Sam · platine (40 villes)"
        );
    }

    #[test]
    fn test_long_cell_puts_its_hint_under_the_name() {
        let line = badge_cell(&SAM_TRAIL, true, 12);
        assert_eq!(
            wrap_cell(&line, 75),
            [
                "[x] Sur les traces de Sam · bronze",
                "    (12 villes, prochain palier\u{202F}: argent à 15)"
            ]
        );
        assert!(
            wrap_cell(&line, 30)
                .iter()
                .all(|row| row.chars().count() <= 30)
        );
        assert_eq!(wrap_cell(&line, 77), [line]);
    }

    fn highlighted(badges: &BadgeLines) -> Vec<String> {
        badges
            .highlights
            .iter()
            .map(|highlight| {
                badges.lines[highlight.line]
                    .chars()
                    .skip(highlight.start)
                    .take(highlight.len)
                    .collect()
            })
            .collect()
    }

    #[test]
    fn test_only_unseen_badges_are_highlighted() {
        let earned = BTreeSet::from(["rain".to_string(), "clear".to_string()]);
        let unseen = BTreeSet::from(["rain".to_string()]);

        let badges = badge_lines(&earned, &unseen, 1, 0, 76);
        assert_eq!(highlighted(&badges), ["[x] Pluie"]);
        assert!(
            badge_lines(&earned, &BTreeSet::new(), 1, 0, 76)
                .highlights
                .is_empty()
        );
    }

    #[test]
    fn test_two_unseen_badges_are_both_highlighted() {
        let earned = BTreeSet::from([
            "rain".to_string(),
            "clear".to_string(),
            "patience".to_string(),
        ]);
        let unseen = BTreeSet::from(["clear".to_string(), "patience".to_string()]);

        let badges = badge_lines(&earned, &unseen, 1, 0, 76);
        assert_eq!(
            highlighted(&badges),
            [
                "[x] Ciel dégagé",
                "[x] Patience (weathr ouvert 1 h d'affilée)"
            ]
        );
    }

    #[test]
    fn test_unseen_sam_trail_highlights_each_wrapped_row() {
        let earned = BTreeSet::from([SAM_TRAIL.id.to_string()]);
        let unseen = earned.clone();

        let badges = badge_lines(&earned, &unseen, 1, 12, 40);
        let rows = wrap_cell(&badge_cell(&SAM_TRAIL, true, 12), 40);
        assert!(rows.len() > 1);
        assert_eq!(badges.highlights.len(), rows.len());
        assert!(
            highlighted(&badges)
                .iter()
                .all(|row| !row.trim().is_empty())
        );
    }

    #[test]
    fn test_highlight_draws_its_cell_in_pink_over_the_line() {
        let lines = vec!["[x] Pluie   [x] Ciel dégagé".to_string()];
        let highlight = Highlight {
            line: 0,
            start: 12,
            len: 15,
        };
        let rows = frame_rows(Frame::Stars, "Badges", &lines, &[highlight], "", 41);
        let pink: Vec<_> = rows
            .iter()
            .flatten()
            .filter(|(_, _, color)| *color == PINK)
            .collect();
        assert_eq!(pink.len(), 1);
        let (column, text, _) = pink[0];
        assert_eq!(text, "[x] Ciel dégagé");
        let row = rows.iter().find(|row| row.contains(pink[0])).unwrap();
        assert_eq!(*column, row[1].0 + 12);
        assert!(row[1].1.starts_with("[x] Pluie"));
    }

    #[test]
    fn test_travel_badges_show_their_name_only() {
        let traveller = BADGES.iter().find(|badge| badge.id == "traveller").unwrap();
        assert_eq!(badge_cell(traveller, false, 0), "[ ] Voyageur");
        assert_eq!(badge_cell(traveller, true, 0), "[x] Voyageur");
    }

    #[test]
    fn test_wrap_respects_width() {
        let lines = wrap("Bienvenue dans la maison au bout du chemin", 12);
        assert_eq!(
            lines,
            ["Bienvenue", "dans la", "maison au", "bout du", "chemin"]
        );
    }

    #[test]
    fn test_wrap_keeps_french_punctuation_with_its_word() {
        let text = "Règle n°1 du village\u{202F}: on lit son courrier. Règle n°2\u{202F}: on ne demande pas qui habite au n°13.";
        assert_eq!(wrap(text, 22)[0], "Règle n°1 du village\u{202F}:");
        for width in 8..40 {
            assert!(
                wrap(text, width)
                    .iter()
                    .all(|line| !line.starts_with([':', ';', '!', '?'])),
                "{width}"
            );
        }
    }

    const FRAMES: [Frame; 14] = [
        Frame::Plain,
        Frame::Parchment,
        Frame::Wet,
        Frame::Newspaper,
        Frame::Postcard,
        Frame::Folder,
        Frame::Recipe,
        Frame::Waves,
        Frame::Photo,
        Frame::Stars,
        Frame::Bubble,
        Frame::Notice,
        Frame::Floppy,
        Frame::Roofs,
    ];

    fn draw(frame: Frame, title: &str, lines: &[&str], footer: &str, width: u16) -> Vec<String> {
        let lines: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
        rows_text(&frame_rows(frame, title, &lines, &[], footer, width), width)
    }

    fn rows_text(rows: &[Vec<Piece>], width: u16) -> Vec<String> {
        rows.iter()
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
    fn test_wet_frame_has_a_stain_a_cut_corner_and_drops() {
        let rows = draw(Frame::Wet, "Lettre 5/6", &["Je te vois."], "[c]", 20);
        assert_eq!(
            rows,
            [
                "+~~~~--------------+",
                "(~~  Lettre 5/6    |",
                "|~                 |",
                "|  Je te vois.     /",
                "|                 /",
                "|  [c]           /",
                "+---,------,----'",
                "    '      .",
            ]
        );
    }

    fn assert_wet_text_clears_the_cut(width: u16, title: &str, body: &str, footer: &str) {
        let inner = Frame::Wet.inner_width(width);
        let lines = wrap(body, inner);
        let rows = frame_rows(Frame::Wet, title, &lines, &[], footer, width);
        let body_rows = &rows[1..rows.len() - 2];
        for (index, row) in body_rows.iter().enumerate() {
            let [(indent, left, _), (start, text, _), (end, right, _)] = row.as_slice() else {
                panic!("{width}: {row:?}");
            };
            assert_eq!(*start, 3, "{width}");
            assert_eq!(usize::from(*indent) + left.chars().count(), 3, "{width}");
            let rows_below = body_rows.len() - index - 1;
            let edge = if rows_below < WET_CUT.len() {
                usize::from(width) - WET_CUT.len() + rows_below
            } else {
                usize::from(width) - 1
            };
            assert_eq!(
                usize::from(*end) + right.chars().count() - 1,
                edge,
                "{width}"
            );
            assert_eq!(text.chars().count(), inner, "{width}: {text}");
            let text_end = usize::from(*start) + text.trim_end().chars().count();
            assert!(text_end <= edge, "{width}: {text}");
        }
    }

    #[test]
    fn test_wet_frame_keeps_text_in_place_on_narrow_terminals() {
        for width in 12..=MAX_BOX_WIDTH {
            assert_wet_text_clears_the_cut(width, "Lettre 5/6", "", "[c]");
        }
        let body = "Je sais que tu lis mes lettres le soir, quand la lampe du salon reste allumée.";
        let footer = "[c] suivante  [<] [>] parcourir  [l] sommaire  [Echap] fermer";
        for width in 23..=MAX_BOX_WIDTH {
            assert_wet_text_clears_the_cut(
                width,
                "Lettre 12/26 (glissée sous la porte)",
                body,
                footer,
            );
        }
    }

    #[test]
    fn test_wet_frame_cuts_short_letters_over_the_rows_they_have() {
        let rows = draw(Frame::Wet, "", &[], "", 12);
        assert_eq!(rows, ["+~~~~------+", "(~~      /", "+---,---'", "    '"]);
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

        let notice = draw(Frame::Notice, "", &["Pleine."], "", 12);
        assert_eq!(notice[0], "╔══════════╗");
        assert_eq!(notice[1], "║ Pleine.  ║");
        assert_eq!(notice.last().unwrap(), "╚══════════╝");

        let floppy = draw(Frame::Floppy, "Sauvegarde", &[], "", 16);
        assert_eq!(floppy[0], "+-[#]----------+");
        assert_eq!(floppy[1], "| Sauvegarde   |");
        assert_eq!(floppy.last().unwrap(), "+--------------+");

        let roofs = draw(Frame::Roofs, "", &["Tricher, c'est nul."], "", 24);
        assert_eq!(roofs[0], "/\\_/\\_/\\_/\\_/\\_/\\_/\\_/\\_");
        assert_eq!(roofs[1], "| Tricher, c'est nul.  |");
        assert_eq!(roofs.last().unwrap(), "|______________________|");
    }

    #[test]
    fn test_bubble_points_its_tail_at_the_speaker() {
        let rows = rows_text(&bubble_rows("Salut.", "[Entrée] suite", 20, Some(6)), 20);
        assert_eq!(
            rows,
            [
                ".------------------.",
                "| Salut.           |",
                "|                  |",
                "| [Entrée] suite   |",
                "'----. .-----------'",
                "      V",
            ]
        );
        let centered = rows_text(&bubble_rows("Salut.", "", 20, None), 20);
        assert_eq!(centered.last().unwrap(), "'------------------'");
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
            for row in frame_rows(frame, title, &lines, &[], footer, width) {
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

    #[test]
    fn test_chevrons_go_back_and_forth_one_column_off_the_box_at_mid_height() {
        let area = BoxArea {
            x: 7,
            y: 4,
            width: 66,
            height: 21,
        };
        assert_eq!(chevron_cell(area, Side::Right, 80, 0), Some((74, 14)));
        assert_eq!(chevron_cell(area, Side::Right, 80, 1), Some((75, 14)));
        assert_eq!(chevron_cell(area, Side::Right, 80, 2), Some((74, 14)));
        assert_eq!(chevron_cell(area, Side::Left, 80, 0), Some((5, 14)));
        assert_eq!(chevron_cell(area, Side::Left, 80, 1), Some((4, 14)));
    }

    #[test]
    fn test_chevrons_hide_without_room_beside_the_box() {
        let area = |x, width| BoxArea {
            x,
            y: 2,
            width,
            height: 9,
        };
        assert_eq!(chevron_cell(area(3, 14), Side::Right, 20, 1), Some((19, 6)));
        assert_eq!(chevron_cell(area(2, 16), Side::Right, 20, 0), None);
        assert_eq!(chevron_cell(area(3, 14), Side::Left, 20, 1), Some((0, 6)));
        assert_eq!(chevron_cell(area(2, 16), Side::Left, 20, 0), None);
    }
}
