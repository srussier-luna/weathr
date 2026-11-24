pub mod toolbar;
pub mod weather_bar;

use crossterm::style::Color;
use weather_bar::WeatherBar;

const MARGIN: u16 = 2;
const TOP_ROW: u16 = 1;
const BLOCK_GAP: usize = 2;
const ELLIPSIS: char = '…';
const MUTED: Color = Color::DarkGrey;

#[derive(Clone, Debug, PartialEq)]
pub struct Segment {
    pub text: String,
    pub color: Color,
}

impl Segment {
    pub fn new(text: impl Into<String>, color: Color) -> Self {
        Self {
            text: text.into(),
            color,
        }
    }
}

pub type Line = Vec<Segment>;

#[derive(Debug, PartialEq)]
pub struct Placed {
    pub x: u16,
    pub y: u16,
    pub line: Line,
}

pub fn width(line: &[Segment]) -> usize {
    line.iter()
        .map(|segment| segment.text.chars().count())
        .sum()
}

pub fn inner_width(terminal_width: u16) -> usize {
    usize::from(terminal_width.saturating_sub(2 * MARGIN))
}

pub fn lay_out(
    bar: &WeatherBar,
    toolbar: Line,
    terminal_width: u16,
    terminal_height: u16,
) -> Vec<Placed> {
    let inner = inner_width(terminal_width);
    let mut placed: Vec<Placed> = bar
        .left
        .iter()
        .zip(TOP_ROW..)
        .map(|(line, y)| Placed {
            x: MARGIN,
            y,
            line: truncate(line, inner),
        })
        .collect();
    for ((row, line), y) in bar.right.iter().enumerate().zip(TOP_ROW..) {
        let taken = bar.left.get(row).map_or(0, |left| width(left) + BLOCK_GAP);
        let line = truncate(line, inner.saturating_sub(taken));
        if !line.is_empty() {
            placed.push(right_aligned(line, y, terminal_width));
        }
    }
    placed.push(right_aligned(
        truncate(&toolbar, inner),
        terminal_height.saturating_sub(1),
        terminal_width,
    ));
    placed
}

fn right_aligned(line: Line, y: u16, terminal_width: u16) -> Placed {
    let line_width = u16::try_from(width(&line)).unwrap_or(u16::MAX);
    Placed {
        x: terminal_width
            .saturating_sub(MARGIN)
            .saturating_sub(line_width),
        y,
        line,
    }
}

fn truncate(line: &[Segment], max_width: usize) -> Line {
    if width(line) <= max_width {
        return line.to_vec();
    }
    let mut room = max_width.saturating_sub(1);
    let mut kept = Line::new();
    for segment in line {
        let text: String = segment.text.chars().take(room).collect();
        room -= text.chars().count();
        if room == 0 {
            if max_width > 0 {
                kept.push(Segment::new(format!("{text}{ELLIPSIS}"), segment.color));
            }
            break;
        }
        kept.push(Segment::new(text, segment.color));
    }
    kept
}

#[cfg(test)]
pub fn text(line: &[Segment]) -> String {
    line.iter().map(|segment| segment.text.as_str()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bar(left: &[&str], right: &[&str]) -> WeatherBar {
        let lines = |texts: &[&str]| {
            texts
                .iter()
                .map(|text| vec![Segment::new(*text, Color::Reset)])
                .collect()
        };
        WeatherBar {
            left: lines(left),
            right: lines(right),
        }
    }

    fn toolbar(text: &str) -> Line {
        vec![Segment::new(text, Color::DarkGrey)]
    }

    fn assert_no_overlap(placed: &[Placed], terminal_width: u16) {
        for (index, a) in placed.iter().enumerate() {
            let a_end = a.x as usize + width(&a.line);
            assert!(
                a_end <= terminal_width as usize,
                "{} overflows",
                text(&a.line)
            );
            for b in &placed[index + 1..] {
                let b_end = b.x as usize + width(&b.line);
                assert!(
                    a.y != b.y || a_end <= b.x as usize || b_end <= a.x as usize,
                    "{} overlaps {}",
                    text(&a.line),
                    text(&b.line)
                );
            }
        }
    }

    #[test]
    fn test_left_block_starts_top_left_one_line_per_info() {
        let placed = lay_out(
            &bar(&["Éclaircies 12 °C", "Vent 8 km/h"], &[]),
            toolbar(""),
            80,
            24,
        );

        assert_eq!((placed[0].x, placed[0].y), (2, 1));
        assert_eq!(text(&placed[0].line), "Éclaircies 12 °C");
        assert_eq!((placed[1].x, placed[1].y), (2, 2));
    }

    #[test]
    fn test_right_block_ends_two_columns_before_the_edge() {
        let placed = lay_out(
            &bar(&["Couvert 3 °C"], &["14:32", "Lyon • Jour 12"]),
            toolbar(""),
            80,
            24,
        );

        let clock = &placed[1];
        assert_eq!((clock.x, clock.y), (73, 1));
        let place = &placed[2];
        assert_eq!((place.x, place.y), (64, 2));
        assert_eq!(text(&place.line), "Lyon • Jour 12");
    }

    #[test]
    fn test_toolbar_sits_right_aligned_on_the_last_row() {
        let placed = lay_out(&bar(&[], &[]), toolbar("[q] Quitter"), 80, 24);

        assert_eq!((placed[0].x, placed[0].y), (67, 23));
    }

    #[test]
    fn test_toolbar_wider_than_a_narrow_terminal_is_cut_and_keeps_the_alert() {
        let placed = lay_out(
            &bar(&[], &[]),
            toolbar("[c] Lettre !  [r] Objet et bouteille !  [b] Badges 54/54  [h]  [q]"),
            50,
            20,
        );

        assert_no_overlap(&placed, 50);
        let line = text(&placed[0].line);
        assert_eq!(placed[0].x, MARGIN);
        assert!(line.starts_with("[c] Lettre !"));
        assert!(line.ends_with('…'));
    }

    #[test]
    fn test_long_place_is_cut_before_it_meets_the_left_block_at_70_columns() {
        let placed = lay_out(
            &bar(
                &["Orage avec grêle 34 °C", "Vent 120 km/h"],
                &[
                    "14:32",
                    "Saint-Rémy-de-Provence-sur-Mer (43.79°N, 4.83°E) • Jour 120",
                ],
            ),
            toolbar("[c] Courrier   [b] Badges 12/54   [h] Aide   [q] Quitter"),
            70,
            20,
        );

        assert_no_overlap(&placed, 70);
        let place = text(&placed[3].line);
        assert!(place.starts_with("Saint-Rémy"));
        assert!(place.ends_with('…'));
        assert_eq!(
            placed[3].x as usize - (2 + "Vent 120 km/h".chars().count()),
            BLOCK_GAP
        );
    }

    #[test]
    fn test_truncate_keeps_colors_and_adds_an_ellipsis() {
        let line = vec![
            Segment::new("Lyon", Color::Red),
            Segment::new(" • Jour 12", Color::Blue),
        ];

        assert_eq!(
            truncate(&line, 7),
            vec![
                Segment::new("Lyon", Color::Red),
                Segment::new(" •…", Color::Blue),
            ]
        );
        assert_eq!(truncate(&line, 0), Line::new());
        assert_eq!(truncate(&line, 20), line);
    }
}
