use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

const PLUME_HEIGHT: u16 = 6;
const STEP_FRAMES: u32 = 12;
const LEFT_PUFF_OFFSET: u16 = 2;

pub struct ChimneySmoke {
    chimney_x: u16,
    chimney_y: u16,
    ceiling: u16,
    frame: u32,
    wind_shift: f32,
}

impl ChimneySmoke {
    pub fn new() -> Self {
        Self {
            chimney_x: 0,
            chimney_y: 0,
            ceiling: 0,
            frame: 0,
            wind_shift: 0.0,
        }
    }

    pub fn set_wind(&mut self, speed_kmh: f32, direction_deg: f32) {
        let speed_factor = speed_kmh / 40.0;
        let x_component = -direction_deg.to_radians().sin();
        self.wind_shift = speed_factor * x_component;
    }

    pub fn update(&mut self, chimney_x: u16, chimney_y: u16, ceiling: u16) {
        self.chimney_x = chimney_x;
        self.chimney_y = chimney_y;
        self.ceiling = ceiling;
        self.frame = self.frame.wrapping_add(1);
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        let step = self.frame / STEP_FRAMES;

        for height in 0..PLUME_HEIGHT {
            let puff = step.wrapping_sub(height as u32);
            if puff % 3 == 2 {
                continue;
            }

            let Some(y) = self
                .chimney_y
                .checked_sub(height)
                .filter(|y| *y >= self.ceiling)
            else {
                continue;
            };
            let is_left = puff % 2 == 0;
            let base_x = if is_left {
                self.chimney_x as f32 - LEFT_PUFF_OFFSET as f32
            } else {
                self.chimney_x as f32
            };
            let x = (base_x + self.wind_shift * height as f32).round();
            if x < 0.0 {
                continue;
            }

            let ch = if is_left { '(' } else { ')' };
            let color = if height < 3 {
                Color::Grey
            } else {
                Color::DarkGrey
            };
            renderer.render_char(x as u16, y, ch, color)?;
        }
        Ok(())
    }
}

impl Default for ChimneySmoke {
    fn default() -> Self {
        Self::new()
    }
}
