use crate::animation::duck::puddle_x;
use crate::animation::garden::{self, GardenLayout};
use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

const SPEED: f32 = 0.1;
const MIN_X: f32 = 3.0;
const POND_CLEARANCE: u16 = 4;
const JUMP_FRAMES: u16 = 12;
const SPLASH_FRAMES: u16 = 10;
const SKIN: Color = Color::Rgb {
    r: 255,
    g: 205,
    b: 148,
};
const SHIRT: Color = Color::Red;
const PANTS: Color = Color::Rgb {
    r: 80,
    g: 160,
    b: 255,
};
const BOOTS: Color = Color::Yellow;
const UMBRELLA: Color = Color::Rgb {
    r: 220,
    g: 50,
    b: 120,
};

pub struct Kid {
    x: f32,
    direction: f32,
    pause: u16,
    jump: u16,
    splash: u16,
    frame: u32,
    avoids_puddles: bool,
}

impl Kid {
    pub fn new() -> Self {
        Self {
            x: -1.0,
            direction: 1.0,
            pause: 0,
            jump: 0,
            splash: 0,
            frame: 0,
            avoids_puddles: false,
        }
    }

    pub fn set_avoids_puddles(&mut self, avoids_puddles: bool) {
        self.avoids_puddles = avoids_puddles;
    }

    fn is_visible(conditions: &WeatherConditions) -> bool {
        conditions.is_day && !conditions.is_thunderstorm
    }

    pub fn update(
        &mut self,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
        rng: &mut impl Rng,
    ) {
        let max_x = puddle_x(layout.width).saturating_sub(POND_CLEARANCE) as f32;
        if !Self::is_visible(conditions) || max_x <= MIN_X {
            return;
        }

        self.frame = self.frame.wrapping_add(1);
        if !(MIN_X..=max_x).contains(&self.x) {
            self.x = rng.random_range(MIN_X..max_x);
        }

        self.splash = self.splash.saturating_sub(1);

        if self.jump > 0 {
            self.jump -= 1;
            if self.jump == 0 && conditions.is_raining {
                self.splash = SPLASH_FRAMES;
            }
        } else if !(self.avoids_puddles && conditions.is_raining) && rng.random::<f32>() < 0.006 {
            self.jump = JUMP_FRAMES;
        }

        if self.pause > 0 {
            self.pause -= 1;
            return;
        }

        self.x += self.direction * SPEED;
        if self.x <= MIN_X {
            self.x = MIN_X;
            self.direction = 1.0;
        } else if self.x >= max_x {
            self.x = max_x;
            self.direction = -1.0;
        }

        let roll = rng.random::<f32>();
        if roll < 0.004 {
            self.pause = rng.random_range(30..90);
        } else if roll < 0.008 {
            self.direction = -self.direction;
        }
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
    ) -> io::Result<()> {
        if !Self::is_visible(conditions) || self.x < MIN_X {
            return Ok(());
        }

        let x = self.x.round() as u16;
        let ground_y = layout.horizon_y;
        let feet_y = if self.jump > 0 {
            ground_y.saturating_sub(1)
        } else {
            ground_y
        };
        let head_y = feet_y.saturating_sub(2);
        let body_y = feet_y.saturating_sub(1);
        let is_mid_step = self.pause == 0 && (self.frame / 6) % 2 == 1;

        if conditions.is_raining {
            let canopy_y = feet_y.saturating_sub(4);
            garden::draw(renderer, x - 1, canopy_y, &[".-^-."], UMBRELLA)?;
            garden::draw(renderer, x - 2, canopy_y + 1, &["'--|--'"], UMBRELLA)?;
            renderer.render_char(x + 1, canopy_y + 1, '|', Color::DarkGrey)?;
            renderer.render_char(x + 1, head_y, '|', Color::DarkGrey)?;
            renderer.render_char(x, head_y, 'o', SKIN)?;
            garden::draw(renderer, x - 1, body_y, &["/|/"], SHIRT)?;
            let boots = if is_mid_step { " b " } else { "d b" };
            garden::draw(renderer, x - 1, feet_y, &[boots], BOOTS)?;
        } else {
            renderer.render_char(x, head_y, 'o', SKIN)?;
            if self.jump > 0 {
                renderer.render_char(x - 1, head_y, '\\', SHIRT)?;
                renderer.render_char(x + 1, head_y, '/', SHIRT)?;
                renderer.render_char(x, body_y, '|', SHIRT)?;
            } else {
                garden::draw(renderer, x - 1, body_y, &["/|\\"], SHIRT)?;
            }
            let legs = if is_mid_step { " | " } else { "/ \\" };
            garden::draw(renderer, x - 1, feet_y, &[legs], PANTS)?;
        }

        if self.splash > 0 {
            let splash_y = ground_y.saturating_sub(1);
            renderer.render_char(x - 2, splash_y, '\'', Color::Cyan)?;
            renderer.render_char(x + 2, splash_y, '\'', Color::Cyan)?;
            renderer.render_char(x - 3, ground_y, '.', Color::Cyan)?;
            renderer.render_char(x + 3, ground_y, '.', Color::Cyan)?;
        }
        Ok(())
    }
}

impl Default for Kid {
    fn default() -> Self {
        Self::new()
    }
}
