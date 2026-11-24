use super::{Placed, Segment};
use crate::overlay;
use crossterm::style::Color;
use std::time::Duration;

const LETTER_DELAY: Duration = Duration::from_millis(30);
const DURATION: Duration = Duration::from_secs(4);
const PADDING: &str = "  ";
const SIDE: &str = "|";
const CORNER: &str = "+";
const RULE: &str = "-";
const CHROME_WIDTH: usize = 2 * (SIDE.len() + PADDING.len());
const TEXT_COLOR: Color = Color::Yellow;
const FRAME_COLOR: Color = Color::Reset;

pub fn lay_out(
    message: &str,
    elapsed: Duration,
    terminal_width: u16,
    terminal_height: u16,
) -> Vec<Placed> {
    let lines = wrap(message, terminal_width);
    let text_width = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);
    let frame_width = text_width + CHROME_WIDTH;
    let x = terminal_width.saturating_sub(u16::try_from(frame_width).unwrap_or(u16::MAX)) / 2;
    let bottom = terminal_height.saturating_sub(2);
    let top = bottom.saturating_sub(u16::try_from(lines.len() + 1).unwrap_or(u16::MAX));
    let rule = format!("{CORNER}{}{CORNER}", RULE.repeat(frame_width - 2));
    let mut revealed = revealed_letters(elapsed);

    let mut placed = vec![Placed {
        x,
        y: top,
        line: vec![Segment::new(rule.clone(), FRAME_COLOR)],
    }];
    for (line, y) in lines.iter().zip(top + 1..) {
        let shown: String = line.chars().take(revealed).collect();
        revealed = revealed.saturating_sub(line.chars().count());
        placed.push(Placed {
            x,
            y,
            line: vec![
                Segment::new(format!("{SIDE}{PADDING}"), FRAME_COLOR),
                Segment::new(format!("{shown:<text_width$}"), TEXT_COLOR),
                Segment::new(format!("{PADDING}{SIDE}"), FRAME_COLOR),
            ],
        });
    }
    placed.push(Placed {
        x,
        y: bottom,
        line: vec![Segment::new(rule, FRAME_COLOR)],
    });
    placed
}

pub fn is_over(message: &str, elapsed: Duration, terminal_width: u16) -> bool {
    let letters: usize = wrap(message, terminal_width)
        .iter()
        .map(|line| line.chars().count())
        .sum();
    elapsed >= LETTER_DELAY * u32::try_from(letters).unwrap_or(u32::MAX) + DURATION
}

fn revealed_letters(elapsed: Duration) -> usize {
    usize::try_from(elapsed.as_millis() / LETTER_DELAY.as_millis()).unwrap_or(usize::MAX)
}

fn wrap(message: &str, terminal_width: u16) -> Vec<String> {
    let max_text_width = super::inner_width(terminal_width)
        .saturating_sub(CHROME_WIDTH)
        .max(1);
    overlay::wrap(message, max_text_width)
        .iter()
        .flat_map(|line| {
            let chars: Vec<char> = line.chars().collect();
            chars
                .chunks(max_text_width)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::super::{text, width};
    use super::*;

    const FOUND: &str = "* Objet trouvé : une vieille clé ! *";

    fn texts(placed: &[Placed]) -> Vec<String> {
        placed.iter().map(|placed| text(&placed.line)).collect()
    }

    #[test]
    fn test_toast_is_framed_and_centred_just_above_the_last_row() {
        let placed = lay_out(FOUND, Duration::from_secs(10), 80, 24);

        let letters = FOUND.chars().count();
        let rule = format!("+{}+", "-".repeat(letters + 4));
        assert_eq!(
            texts(&placed),
            vec![rule.clone(), format!("|  {FOUND}  |"), rule]
        );
        assert_eq!(
            placed.iter().map(|placed| placed.y).collect::<Vec<_>>(),
            vec![20, 21, 22]
        );
        let frame_width = letters + 6;
        assert!(
            placed
                .iter()
                .all(|line| line.x as usize == (80 - frame_width) / 2)
        );
        assert!(placed.iter().all(|line| line.y != 0));
    }

    #[test]
    fn test_text_is_typed_one_letter_every_30_ms_inside_a_frame_of_final_width() {
        let start = lay_out(FOUND, Duration::ZERO, 80, 24);
        let after_three = lay_out(FOUND, Duration::from_millis(95), 80, 24);
        let typed = lay_out(FOUND, Duration::from_secs(10), 80, 24);

        assert_eq!(text(&start[1].line).trim_matches(['|', ' ']), "");
        assert_eq!(text(&after_three[1].line).trim_matches(['|', ' ']), "* O");
        for frame in [&start, &after_three] {
            assert_eq!(frame.len(), typed.len());
            for (line, final_line) in frame.iter().zip(&typed) {
                assert_eq!((line.x, line.y), (final_line.x, final_line.y));
                assert_eq!(width(&line.line), width(&final_line.line));
            }
        }
    }

    #[test]
    fn test_toast_stays_4_s_after_the_last_letter() {
        let typing = LETTER_DELAY * u32::try_from(FOUND.chars().count()).unwrap();

        assert!(!is_over(FOUND, typing, 80));
        assert!(!is_over(FOUND, typing + Duration::from_millis(3999), 80));
        assert!(is_over(FOUND, typing + Duration::from_secs(4), 80));
    }

    #[test]
    fn test_long_toast_wraps_inside_the_screen_at_70_columns() {
        let message = "* Une bouteille s'est échouée sur la berge : « Si tu lis ce message, \
                       c'est que la rivière a bien voulu me porter jusqu'à toi » *";
        let placed = lay_out(message, Duration::from_secs(60), 70, 20);

        assert!(placed.len() > 3);
        assert_eq!(placed.last().unwrap().y, 18);
        let frame_width = width(&placed[0].line);
        for line in &placed {
            assert_eq!(width(&line.line), frame_width);
            assert!(line.x >= 2);
            assert!(line.x as usize + frame_width <= 68);
        }
        let shown: Vec<String> = placed[1..placed.len() - 1]
            .iter()
            .map(|line| text(&line.line).trim_matches(['|', ' ']).to_string())
            .collect();
        assert_eq!(
            shown.join(" "),
            message
                .split_ascii_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        );
    }

    #[test]
    fn test_word_longer_than_the_frame_is_cut_rather_than_overflowing() {
        let placed = lay_out(&"a".repeat(100), Duration::from_secs(60), 20, 10);

        for line in &placed {
            assert!(line.x as usize + width(&line.line) <= 18);
        }
    }
}
