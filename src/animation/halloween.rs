use crate::animation::cloud_bank::CloudBank;
use crate::animation::garden::{self, Bubble, GardenLayout};
use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

const BAT_COUNT: usize = 3;
const BAT_SPEED: f32 = 0.35;
const GHOST_SPEED: f32 = 0.08;
const GHOST: [&str; 4] = [" .-. ", "(o o)", "| O |", "'~~~'"];
const PUMPKIN_TOP: &str = " _|_ ";
const PUMPKIN_FACE: &str = "(^v^)";
const PUMPKIN_OFFSETS: [i32; 2] = [-8, 6];
const PUMPKIN: Color = Color::Rgb {
    r: 255,
    g: 120,
    b: 0,
};
const BAT: Color = Color::Rgb {
    r: 150,
    g: 100,
    b: 190,
};

struct Bat {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

pub struct Halloween {
    bats: Vec<Bat>,
    ghost_x: Option<f32>,
    ghost_direction: f32,
    boo: Bubble,
    frame: u32,
}

impl Halloween {
    pub fn new() -> Self {
        Self {
            bats: Vec::with_capacity(BAT_COUNT),
            ghost_x: None,
            ghost_direction: 1.0,
            boo: Bubble::new(),
            frame: 0,
        }
    }

    pub fn update(
        &mut self,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
        rng: &mut impl Rng,
    ) {
        self.frame = self.frame.wrapping_add(1);
        if conditions.is_day {
            self.bats.clear();
            self.ghost_x = None;
            self.boo.hide();
            return;
        }

        let width = layout.width as f32;
        let top = (CloudBank::bottom() + 1) as f32;
        let bottom = (layout.house_y + 3) as f32;
        while self.bats.len() < BAT_COUNT {
            self.bats.push(Bat {
                x: rng.random_range(0.0..width),
                y: rng.random_range(top..bottom.max(top + 1.0)),
                vx: BAT_SPEED,
                vy: 0.0,
            });
        }
        for bat in &mut self.bats {
            if rng.random::<f32>() < 0.05 {
                bat.vx = rng.random_range(-BAT_SPEED..BAT_SPEED);
                bat.vy = rng.random_range(-0.15..0.15);
            }
            bat.x = (bat.x + bat.vx).rem_euclid(width);
            bat.y = (bat.y + bat.vy).clamp(top, bottom.max(top + 1.0));
        }

        let max_x = layout.width.saturating_sub(7) as f32;
        let x = *self
            .ghost_x
            .get_or_insert(rng.random_range(2.0..max_x.max(3.0)));
        if rng.random::<f32>() < 0.004 || x <= 2.0 || x >= max_x {
            self.ghost_direction = if x <= 2.0 {
                1.0
            } else if x >= max_x {
                -1.0
            } else {
                -self.ghost_direction
            };
        }
        self.ghost_x = Some(x + GHOST_SPEED * self.ghost_direction);
        self.boo.update(0.002, 45, rng.random::<f32>());
    }

    pub fn render_spirits(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
    ) -> io::Result<()> {
        let wings = if (self.frame / 4) % 2 == 0 {
            "^v^"
        } else {
            "-v-"
        };
        for bat in &self.bats {
            garden::draw(renderer, bat.x as u16, bat.y as u16, &[wings], BAT)?;
        }

        if let Some(x) = self.ghost_x {
            let bob = ((self.frame as f32 * 0.08).sin() * 1.2).round() as i32;
            let y = (layout.horizon_y as i32 - 8 + bob).max(0) as u16;
            garden::draw_solid(renderer, x as u16, y, &GHOST, Color::White)?;
            if self.boo.is_visible() {
                garden::draw_solid(
                    renderer,
                    x as u16,
                    y.saturating_sub(1),
                    &["Bouh!"],
                    Color::White,
                )?;
            }
        }
        Ok(())
    }

    pub fn render_pumpkins(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        is_day: bool,
    ) -> io::Result<()> {
        let face = if is_day {
            Color::Rgb {
                r: 140,
                g: 70,
                b: 0,
            }
        } else if (self.frame / 7) % 3 == 0 {
            Color::Rgb {
                r: 255,
                g: 200,
                b: 0,
            }
        } else {
            Color::Yellow
        };
        let door_x = (layout.house_x + layout.house.door_x()) as i32;
        let y = layout.horizon_y.saturating_sub(2);

        for offset in PUMPKIN_OFFSETS {
            let x = (door_x + offset).max(0) as u16;
            garden::draw_solid(renderer, x, y, &[PUMPKIN_TOP, PUMPKIN_FACE], PUMPKIN)?;
            renderer.render_char(x + 2, y, '|', Color::Green)?;
            for (i, ch) in PUMPKIN_FACE.chars().enumerate() {
                if matches!(ch, '^' | 'v') {
                    renderer.render_char(x + i as u16, y + 1, ch, face)?;
                }
            }
        }
        Ok(())
    }
}

impl Default for Halloween {
    fn default() -> Self {
        Self::new()
    }
}
