use crate::animation::garden::{self, Bubble, GardenLayout};
use crate::animation::lore::Unlocks;
use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

pub const PUDDLE_WIDTH: u16 = 19;
const RIGHT_MARGIN: u16 = 1;
const DUCK_WIDTH: u16 = 5;
const STEP_FRAMES: u16 = 20;
const DUCK_RIGHT: [&str; 2] = [" _(o>", "\\___)"];
const DUCK_LEFT: [&str; 2] = ["<o)_ ", "(___/"];
const BEAK_COLOR: Color = Color::Rgb {
    r: 255,
    g: 140,
    b: 0,
};

pub fn puddle_x(terminal_width: u16) -> u16 {
    terminal_width.saturating_sub(PUDDLE_WIDTH + RIGHT_MARGIN)
}

pub struct Duck {
    offset: u16,
    is_facing_right: bool,
    step_timer: u16,
    frame: u32,
    quack: Bubble,
}

impl Duck {
    pub fn new() -> Self {
        Self {
            offset: 4,
            is_facing_right: true,
            step_timer: 0,
            frame: 0,
            quack: Bubble::new(),
        }
    }

    fn is_visible(conditions: &WeatherConditions) -> bool {
        conditions.is_raining || conditions.is_thunderstorm
    }

    pub fn update(&mut self, conditions: &WeatherConditions, rng: &mut impl Rng) {
        if !Self::is_visible(conditions) {
            self.quack.hide();
            return;
        }

        self.frame = self.frame.wrapping_add(1);
        self.step_timer += 1;

        if self.step_timer >= STEP_FRAMES {
            self.step_timer = 0;
            if rng.random::<f32>() < 0.15 {
                self.is_facing_right = !self.is_facing_right;
            } else if self.is_facing_right {
                if self.offset + DUCK_WIDTH + 1 < PUDDLE_WIDTH {
                    self.offset += 1;
                } else {
                    self.is_facing_right = false;
                }
            } else if self.offset > 1 {
                self.offset -= 1;
            } else {
                self.is_facing_right = true;
            }
        }

        self.quack.update(0.003, 40, rng.random::<f32>());
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
        unlocks: &Unlocks,
    ) -> io::Result<()> {
        if !Self::is_visible(conditions) {
            return Ok(());
        }

        let puddle_x = puddle_x(layout.width);
        let y = layout.horizon_y;

        let body_color = if conditions.is_day {
            Color::White
        } else {
            Color::Grey
        };
        let duck_x = puddle_x + self.offset;
        let head_y = y.saturating_sub(1);

        if self.is_facing_right {
            garden::draw_solid(renderer, duck_x, head_y, &DUCK_RIGHT, body_color)?;
            renderer.render_char(duck_x + 4, head_y, '>', BEAK_COLOR)?;
        } else {
            garden::draw_solid(renderer, duck_x, head_y, &DUCK_LEFT, body_color)?;
            renderer.render_char(duck_x, head_y, '<', BEAK_COLOR)?;
        }

        if unlocks.ducklings {
            let count = if unlocks.choices.three_ducklings {
                3
            } else {
                2
            };
            let (step, start, duckling) = if self.is_facing_right {
                (-3, -3, "o>")
            } else {
                (3, 6, "<o")
            };
            for offset in (0..count).map(|i| start + i * step) {
                garden::draw_clipped(
                    renderer,
                    duck_x as i32 + offset,
                    y,
                    &[duckling],
                    Color::Yellow,
                )?;
            }
        }

        if self.quack.is_visible() {
            garden::draw_solid(
                renderer,
                duck_x,
                head_y.saturating_sub(1),
                &["Coin!"],
                Color::White,
            )?;
        }
        Ok(())
    }
}

impl Default for Duck {
    fn default() -> Self {
        Self::new()
    }
}
