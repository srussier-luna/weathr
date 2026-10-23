use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

const BANNER_ROW: u16 = 0;
const COLOR_STEP_FRAMES: u32 = 6;
const PALETTE: [Color; 6] = [
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::Cyan,
    Color::Magenta,
    Color::White,
];

pub struct NewYearBanner {
    frame: u32,
}

impl NewYearBanner {
    pub fn new() -> Self {
        Self { frame: 0 }
    }

    pub fn update(&mut self) {
        self.frame = self.frame.wrapping_add(1);
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        terminal_width: u16,
        year: i32,
    ) -> io::Result<()> {
        let text = format!("*  Bonne année {year}  *");
        let length = text.chars().count() as u16;
        let x = terminal_width.saturating_sub(length) / 2;
        let shift = (self.frame / COLOR_STEP_FRAMES) as usize;

        for (i, ch) in text.chars().enumerate() {
            if ch != ' ' {
                let color = PALETTE[(i + shift) % PALETTE.len()];
                renderer.render_char(x + i as u16, BANNER_ROW, ch, color)?;
            }
        }
        Ok(())
    }
}

impl Default for NewYearBanner {
    fn default() -> Self {
        Self::new()
    }
}
