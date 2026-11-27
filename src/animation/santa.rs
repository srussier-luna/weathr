use crate::animation::cloud_bank::CloudBank;
use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

const SLEIGH: [&str; 3] = [
    "  _o_       Y Y   Y Y",
    " [_#_]======(o)===(o)",
    "  ' '       / \\   / \\",
];
const WIDTH: u16 = 21;
const HEIGHT: u16 = SLEIGH.len() as u16;
const SPEED: f32 = 0.5;
const ALTITUDE: u16 = CloudBank::TOP + 1;
const FUR: Color = Color::Rgb {
    r: 150,
    g: 95,
    b: 50,
};
const WOOD: Color = Color::Rgb {
    r: 170,
    g: 40,
    b: 30,
};

#[derive(Default)]
pub struct Sleigh {
    x: Option<f32>,
}

impl Sleigh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn take_off(&mut self) {
        self.x = Some(-f32::from(WIDTH));
    }

    pub fn is_flying(&self) -> bool {
        self.x.is_some()
    }

    pub fn update(&mut self, term_width: u16) {
        self.x = self
            .x
            .map(|x| x + SPEED)
            .filter(|&x| x < f32::from(term_width));
    }

    pub fn render(&self, renderer: &mut TerminalRenderer, horizon_y: u16) -> io::Result<()> {
        let Some(x) = self.x else {
            return Ok(());
        };
        if horizon_y < ALTITUDE + HEIGHT {
            return Ok(());
        }
        for (row, line) in SLEIGH.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let column = x as i32 + col as i32;
                if ch != ' ' && column >= 0 {
                    renderer.render_char(
                        column as u16,
                        ALTITUDE + row as u16,
                        ch,
                        color(row, ch),
                    )?;
                }
            }
        }
        Ok(())
    }
}

fn color(row: usize, ch: char) -> Color {
    match (row, ch) {
        (_, '#') | (1, 'o') => Color::Red,
        (0, 'o') => Color::White,
        (_, '=') => Color::Grey,
        (_, 'Y' | '(' | ')' | '/' | '\\') => FUR,
        _ => WOOD,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleigh_rows_are_as_wide_as_the_sleigh() {
        assert!(
            SLEIGH
                .iter()
                .all(|line| line.chars().count() == WIDTH as usize)
        );
    }

    #[test]
    fn test_sleigh_crosses_the_sky_once() {
        let mut sleigh = Sleigh::new();
        assert!(!sleigh.is_flying());
        sleigh.take_off();
        let frames = (0..1000)
            .take_while(|_| {
                sleigh.update(80);
                sleigh.is_flying()
            })
            .count();
        assert_eq!(frames as f32, (80.0 + f32::from(WIDTH)) / SPEED - 1.0);
        sleigh.update(80);
        assert!(!sleigh.is_flying());
    }
}
