use crate::holiday::Season;
use crate::render::TerminalRenderer;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

const STRONG_WIND: f32 = 10.0;
const MAX_LEAVES: usize = 40;
const LEAF_CHARS: [char; 4] = [',', '`', '\'', '~'];
const BAND_HEIGHT: u16 = 12;

struct Leaf {
    x: f32,
    y: f32,
    vx: f32,
    phase: f32,
    ch: char,
    color: Color,
}

pub struct Gust {
    leaves: Vec<Leaf>,
    wind_speed: f32,
    direction: f32,
}

impl Gust {
    pub fn new() -> Self {
        Self {
            leaves: Vec::with_capacity(MAX_LEAVES),
            wind_speed: 0.0,
            direction: 1.0,
        }
    }

    pub fn set_wind(&mut self, speed: f32, direction_deg: f32) {
        self.wind_speed = speed;
        self.direction = if -direction_deg.to_radians().sin() >= 0.0 {
            1.0
        } else {
            -1.0
        };
    }

    pub fn update(
        &mut self,
        terminal_width: u16,
        horizon_y: u16,
        season: Season,
        rng: &mut impl Rng,
    ) {
        let width = terminal_width as f32;
        for leaf in &mut self.leaves {
            leaf.x += leaf.vx;
            leaf.phase += 0.2;
            leaf.y += leaf.phase.sin() * 0.15;
        }
        self.leaves
            .retain(|leaf| leaf.x >= -1.0 && leaf.x <= width + 1.0);

        let strength = (self.wind_speed - STRONG_WIND) / 20.0;
        if strength <= 0.0 || self.leaves.len() >= MAX_LEAVES || rng.random::<f32>() > strength {
            return;
        }

        let top = horizon_y.saturating_sub(BAND_HEIGHT) as f32;
        let speed = (0.6 + rng.random::<f32>() * 0.6) * (self.wind_speed / 15.0).min(2.0);
        self.leaves.push(Leaf {
            x: if self.direction > 0.0 { 0.0 } else { width },
            y: rng.random_range(top..horizon_y as f32),
            vx: speed * self.direction,
            phase: rng.random::<f32>() * 6.0,
            ch: LEAF_CHARS[rng.random_range(0..LEAF_CHARS.len())],
            color: leaf_color(season, rng),
        });
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        for leaf in &self.leaves {
            if leaf.x >= 0.0 && leaf.y >= 0.0 {
                renderer.render_char(leaf.x as u16, leaf.y as u16, leaf.ch, leaf.color)?;
            }
        }
        Ok(())
    }
}

fn leaf_color(season: Season, rng: &mut impl Rng) -> Color {
    match season {
        Season::Autumn | Season::Winter => {
            let palette = [
                Color::Rgb {
                    r: 230,
                    g: 120,
                    b: 30,
                },
                Color::Rgb {
                    r: 200,
                    g: 60,
                    b: 30,
                },
                Color::Rgb {
                    r: 150,
                    g: 90,
                    b: 40,
                },
            ];
            palette[rng.random_range(0..palette.len())]
        }
        Season::Spring | Season::Summer => Color::Rgb {
            r: 90,
            g: 170,
            b: 60,
        },
    }
}

impl Default for Gust {
    fn default() -> Self {
        Self::new()
    }
}
