use crossterm::queue;
use crossterm::style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor};
use std::io::{self, IsTerminal, Write};

type Rgb = [u8; 3];

const LOGO: [&str; 6] = [
    "██╗    ██╗███████╗ █████╗ ████████╗██╗  ██╗██████╗",
    "██║    ██║██╔════╝██╔══██╗╚══██╔══╝██║  ██║██╔══██╗",
    "██║ █╗ ██║█████╗  ███████║   ██║   ███████║██████╔╝",
    "██║███╗██║██╔══╝  ██╔══██║   ██║   ██╔══██║██╔══██╗",
    "╚███╔███╔╝███████╗██║  ██║   ██║   ██║  ██║██║  ██║",
    " ╚══╝╚══╝ ╚══════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝",
];
const OUTLINE: &str = "╗╔╝╚═║";
const OUTLINE_SHADE: f64 = 0.45;
const SUNSET: [Rgb; 4] = [
    [255, 200, 60],
    [255, 110, 80],
    [220, 60, 140],
    [140, 70, 200],
];
const YELLOW: Rgb = [255, 210, 70];
const GREY: Rgb = [150, 150, 165];
const GREEN: Rgb = [110, 200, 110];
const POSTMAN: [(&str, Rgb); 4] = [
    ("     __o       .--.", YELLOW),
    ("   _`\\<,_     |____|>", YELLOW),
    ("  (*)/ (*)      ||", GREY),
    ("~~~~~~~~~~~~~~~~~~~~~", GREEN),
];
const MESSAGE_POSITION: f64 = 0.3;

pub fn show(message: Option<&str>) {
    if !io::stdout().is_terminal() {
        return;
    }
    let _ = draw(&mut io::stdout(), message);
}

fn draw(out: &mut impl Write, message: Option<&str>) -> io::Result<()> {
    let logo_width = width(LOGO);
    for row in LOGO {
        queue!(out, SetAttribute(Attribute::Bold))?;
        for (x, character) in row.chars().enumerate() {
            let position = x as f64 / (logo_width - 1) as f64;
            queue!(
                out,
                SetForegroundColor(logo_color(character, position)),
                Print(character)
            )?;
        }
        queue!(out, SetAttribute(Attribute::Reset))?;
        writeln!(out)?;
    }
    writeln!(out)?;

    let postman_width = width(POSTMAN.map(|(row, _)| row));
    let indent = " ".repeat(logo_width.saturating_sub(postman_width) / 2);
    for (row, color) in POSTMAN {
        queue!(
            out,
            Print(&indent),
            SetForegroundColor(rgb(color)),
            Print(row),
            ResetColor
        )?;
        writeln!(out)?;
    }
    writeln!(out)?;

    if let Some(message) = message {
        queue!(
            out,
            SetForegroundColor(rgb(gradient(MESSAGE_POSITION))),
            Print(message),
            ResetColor
        )?;
        writeln!(out)?;
    }
    out.flush()
}

fn width<'a>(rows: impl IntoIterator<Item = &'a str>) -> usize {
    rows.into_iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or_default()
}

fn logo_color(character: char, position: f64) -> Color {
    let color = gradient(position);
    if OUTLINE.contains(character) {
        rgb(darker(color))
    } else {
        rgb(color)
    }
}

fn gradient(position: f64) -> Rgb {
    let scaled = position.clamp(0.0, 1.0) * (SUNSET.len() - 1) as f64;
    let index = (scaled as usize).min(SUNSET.len() - 2);
    let fraction = scaled - index as f64;
    let (from, to) = (SUNSET[index], SUNSET[index + 1]);
    std::array::from_fn(|channel| {
        let (from, to) = (f64::from(from[channel]), f64::from(to[channel]));
        (from + (to - from) * fraction).round_ties_even() as u8
    })
}

fn darker(color: Rgb) -> Rgb {
    color.map(|channel| (f64::from(channel) * OUTLINE_SHADE).round_ties_even() as u8)
}

fn rgb([r, g, b]: Rgb) -> Color {
    Color::Rgb { r, g, b }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_runs_from_first_to_last_sunset_stop() {
        assert_eq!(gradient(0.0), SUNSET[0]);
        assert_eq!(gradient(1.0), SUNSET[3]);
    }

    #[test]
    fn test_gradient_blends_neighbour_stops() {
        assert_eq!(gradient(MESSAGE_POSITION), [255, 119, 78]);
        assert_eq!(gradient(0.5), [238, 85, 110]);
    }

    #[test]
    fn test_outline_uses_darker_gradient_colour() {
        assert_eq!(logo_color('█', 0.0), rgb(SUNSET[0]));
        assert_eq!(logo_color('╗', 0.0), rgb([115, 90, 27]));
    }
}
