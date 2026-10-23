use crate::animation::garden::{self, GardenLayout, scatter};
use crate::holiday::Holiday;
use crate::render::TerminalRenderer;
use crate::scene::river::River;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

const FLOWER_DENSITY: f32 = 0.18;
const FLOWERS: [char; 4] = ['@', '*', 'o', '%'];
const EGG_COUNT: u16 = 6;
const LILY_ROWS: [u16; 3] = [1, 2, 3];
const LILY: [char; 3] = ['(', '*', ')'];
const LILY_PAD: Color = Color::Rgb {
    r: 60,
    g: 170,
    b: 60,
};
const LILY_FLOWER: Color = Color::Rgb {
    r: 255,
    g: 120,
    b: 180,
};
const BUTTERFLY_COUNT: usize = 4;
const BUTTERFLY_SPEED: f32 = 0.25;
const PALETTE: [Color; 6] = [
    Color::Red,
    Color::Magenta,
    Color::Yellow,
    Color::White,
    Color::Cyan,
    Color::Rgb {
        r: 255,
        g: 150,
        b: 200,
    },
];

struct Butterfly {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
}

pub struct Spring {
    butterflies: Vec<Butterfly>,
    frame: u32,
}

impl Spring {
    pub fn new() -> Self {
        Self {
            butterflies: Vec::with_capacity(BUTTERFLY_COUNT),
            frame: 0,
        }
    }

    fn has_butterflies(holiday: &Holiday, conditions: &WeatherConditions) -> bool {
        holiday.is_spring
            && conditions.is_day
            && !conditions.is_raining
            && !conditions.is_thunderstorm
            && !conditions.is_snowing
    }

    pub fn update(
        &mut self,
        layout: &GardenLayout,
        holiday: &Holiday,
        conditions: &WeatherConditions,
        rng: &mut impl Rng,
    ) {
        self.frame = self.frame.wrapping_add(1);
        if !Self::has_butterflies(holiday, conditions) {
            self.butterflies.clear();
            return;
        }

        let width = layout.width.saturating_sub(2) as f32;
        let top = layout.horizon_y.saturating_sub(10) as f32;
        let bottom = layout.horizon_y.saturating_sub(2) as f32;
        while self.butterflies.len() < BUTTERFLY_COUNT {
            self.butterflies.push(Butterfly {
                x: rng.random_range(1.0..width.max(2.0)),
                y: rng.random_range(top..bottom.max(top + 1.0)),
                vx: 0.0,
                vy: 0.0,
                color: PALETTE[rng.random_range(0..PALETTE.len())],
            });
        }
        for butterfly in &mut self.butterflies {
            if rng.random::<f32>() < 0.08 {
                butterfly.vx = rng.random_range(-BUTTERFLY_SPEED..BUTTERFLY_SPEED);
                butterfly.vy = rng.random_range(-0.15..0.15);
            }
            butterfly.x = (butterfly.x + butterfly.vx).clamp(1.0, width.max(2.0));
            butterfly.y = (butterfly.y + butterfly.vy).clamp(top, bottom.max(top + 1.0));
        }
    }

    pub fn render_ground(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        holiday: &Holiday,
    ) -> io::Result<()> {
        if holiday.is_spring {
            let y = layout.horizon_y.saturating_sub(1);
            for x in 0..layout.width {
                let is_free = matches!(renderer.char_at(x, y), Some(' ' | '^'));
                if is_free && scatter(x, 7) < FLOWER_DENSITY {
                    let flower = FLOWERS[(scatter(x, 11) * FLOWERS.len() as f32) as usize];
                    let color = PALETTE[(scatter(x, 13) * PALETTE.len() as f32) as usize];
                    renderer.render_char(x, y, flower, color)?;
                }
            }
        }

        if holiday.is_easter {
            let span = layout.width.saturating_sub(4) as f32;
            for egg in 0..EGG_COUNT {
                let x = 2 + (scatter(egg, 23) * span) as u16;
                let color = PALETTE[egg as usize % PALETTE.len()];
                renderer.render_char(x, layout.horizon_y, 'O', color)?;
            }
        }
        Ok(())
    }

    pub fn render_lilies(
        &self,
        renderer: &mut TerminalRenderer,
        river: &River,
        holiday: &Holiday,
    ) -> io::Result<()> {
        if !holiday.is_spring {
            return Ok(());
        }
        for (index, depth) in LILY_ROWS.into_iter().enumerate() {
            let (y, span) = river.pond_row(depth);
            let room = span.len().saturating_sub(LILY.len() + 2) as f32;
            if room <= 0.0 {
                continue;
            }
            let x = span.start + 1 + (scatter(index as u16, 41) * room) as u16;
            for (offset, ch) in LILY.into_iter().enumerate() {
                let color = if ch == '*' { LILY_FLOWER } else { LILY_PAD };
                renderer.render_char(x + offset as u16, y, ch, color)?;
            }
        }
        Ok(())
    }

    pub fn render_butterflies(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        let wings = if (self.frame / 5) % 2 == 0 {
            "}{"
        } else {
            ")("
        };
        for butterfly in &self.butterflies {
            garden::draw(
                renderer,
                butterfly.x as u16,
                butterfly.y as u16,
                &[wings],
                butterfly.color,
            )?;
        }
        Ok(())
    }
}

impl Default for Spring {
    fn default() -> Self {
        Self::new()
    }
}
