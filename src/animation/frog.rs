use crate::animation::duck::puddle_x;
use crate::animation::garden::{self, Bubble, GardenLayout};
use crate::animation::lore::Unlocks;
use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

const JUMP_FRAMES: u16 = 14;
const BANK_OFFSET: u16 = 8;
const POND_OFFSET: u16 = 5;
const SPRITE: [&str; 2] = [" @..@", "(----)"];

pub struct Frog {
    is_in_pond: bool,
    wait: u16,
    jump: u16,
    splash: Bubble,
}

impl Frog {
    pub fn new() -> Self {
        Self {
            is_in_pond: false,
            wait: 60,
            jump: 0,
            splash: Bubble::new(),
        }
    }

    fn is_visible(conditions: &WeatherConditions) -> bool {
        conditions.is_raining || conditions.is_thunderstorm
    }

    pub fn update(&mut self, conditions: &WeatherConditions, rng: &mut impl Rng) {
        if !Self::is_visible(conditions) {
            self.splash.hide();
            return;
        }

        self.splash.tick();
        if self.jump > 0 {
            self.jump -= 1;
            if self.jump == 0 {
                self.is_in_pond = !self.is_in_pond;
                self.wait = rng.random_range(60..150);
                if self.is_in_pond {
                    self.splash.show(30);
                }
            }
        } else if self.wait > 0 {
            self.wait -= 1;
        } else {
            self.jump = JUMP_FRAMES;
        }
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

        let pond = puddle_x(layout.width);
        let bank_x = pond.saturating_sub(BANK_OFFSET) as f32;
        let pond_x = (pond + POND_OFFSET) as f32;
        let (from, to) = if self.is_in_pond {
            (pond_x, bank_x)
        } else {
            (bank_x, pond_x)
        };

        let progress = if self.jump > 0 {
            1.0 - self.jump as f32 / JUMP_FRAMES as f32
        } else {
            0.0
        };
        let x = from + (to - from) * progress;
        let lift = (progress * std::f32::consts::PI).sin() * 2.0;
        let y = layout.horizon_y.saturating_sub(1 + lift.round() as u16);

        garden::draw(renderer, x as u16, y, &SPRITE, Color::Green)?;
        renderer.render_char(x as u16 + 1, y, '@', Color::Yellow)?;
        renderer.render_char(x as u16 + 4, y, '@', Color::Yellow)?;

        if self.splash.is_visible() {
            garden::draw_solid(
                renderer,
                x as u16,
                y.saturating_sub(1),
                &[if unlocks.choices.frog_croak {
                    "Croâ !"
                } else {
                    "Plouf!"
                }],
                Color::White,
            )?;
        }
        Ok(())
    }
}

impl Default for Frog {
    fn default() -> Self {
        Self::new()
    }
}
