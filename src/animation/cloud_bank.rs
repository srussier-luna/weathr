use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use std::io;

const MIN_DRIFT: f32 = 0.03;
const NIGHT_DIMMING: f32 = 0.65;
const TOP_TILE: &str = "  .--.    .---.     ";
const MIDDLE_TILE: &str = "-(    )--(     )----";
const BOTTOM_TILE: &str = "`-.___.-'";

pub struct CloudBank {
    offset: f32,
    drift: f32,
}

impl CloudBank {
    pub const TOP: u16 = 2;
    pub const HEIGHT: u16 = 4;

    pub fn new() -> Self {
        Self {
            offset: 0.0,
            drift: MIN_DRIFT,
        }
    }

    pub fn bottom() -> u16 {
        Self::TOP + Self::HEIGHT
    }

    pub fn set_wind(&mut self, speed_kmh: f32, direction_deg: f32) {
        let drift = speed_kmh / 40.0 * -direction_deg.to_radians().sin() * 0.1;
        self.drift = if drift.abs() < MIN_DRIFT {
            MIN_DRIFT.copysign(drift)
        } else {
            drift
        };
    }

    pub fn update(&mut self) {
        self.offset += self.drift;
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        terminal_width: u16,
        conditions: &WeatherConditions,
    ) -> io::Result<()> {
        let color = Self::color(conditions);
        let last = terminal_width.saturating_sub(1);
        let top: Vec<char> = TOP_TILE.chars().collect();
        let middle: Vec<char> = MIDDLE_TILE.chars().collect();
        let bottom: Vec<char> = BOTTOM_TILE.chars().collect();
        let shift = self.offset.rem_euclid(bottom.len() as f32) as usize;

        for x in 0..terminal_width {
            let column = x as usize;
            let (middle_ch, body_ch) = if x == 0 {
                ('.', '(')
            } else if x == last {
                ('.', ')')
            } else {
                (middle[column % middle.len()], ' ')
            };

            let top_ch = top[column % top.len()];
            if top_ch != ' ' {
                renderer.render_char(x, Self::TOP, top_ch, color)?;
            }
            renderer.render_char(x, Self::TOP + 1, middle_ch, color)?;
            renderer.render_char(x, Self::TOP + 2, body_ch, color)?;
            renderer.render_char(
                x,
                Self::TOP + 3,
                bottom[(column + bottom.len() - shift) % bottom.len()],
                color,
            )?;
        }
        Ok(())
    }

    fn color(conditions: &WeatherConditions) -> Color {
        let level = if conditions.is_thunderstorm {
            110.0
        } else if conditions.is_snowing {
            225.0
        } else {
            170.0
        };
        let dimming = if conditions.is_day {
            1.0
        } else {
            NIGHT_DIMMING
        };
        let value = (level * dimming) as u8;
        Color::Rgb {
            r: value,
            g: value,
            b: value.saturating_add(10),
        }
    }
}

impl Default for CloudBank {
    fn default() -> Self {
        Self::new()
    }
}
