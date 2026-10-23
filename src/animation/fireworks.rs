use crate::render::TerminalRenderer;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

const MAX_ROCKETS: usize = 3;
const MAX_SPARKS: usize = 400;
const ROCKET_SPEED: f32 = 0.45;
const GRAVITY: f32 = 0.012;
const DRAG: f32 = 0.97;
const SKY_TOP: f32 = 3.0;
const PALETTE: [Color; 7] = [
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::Cyan,
    Color::Magenta,
    Color::Blue,
    Color::White,
];

struct Rocket {
    x: f32,
    y: f32,
    burst_y: f32,
}

struct Spark {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: u16,
    max_age: u16,
    color: Color,
}

pub struct Fireworks {
    rockets: Vec<Rocket>,
    sparks: Vec<Spark>,
    cooldown: u16,
}

impl Fireworks {
    pub fn new() -> Self {
        Self {
            rockets: Vec::with_capacity(MAX_ROCKETS),
            sparks: Vec::with_capacity(MAX_SPARKS),
            cooldown: 0,
        }
    }

    pub fn update(&mut self, terminal_width: u16, horizon_y: u16, rng: &mut impl Rng) {
        for rocket in &mut self.rockets {
            rocket.y -= ROCKET_SPEED;
        }

        let (exploded, flying): (Vec<_>, Vec<_>) =
            self.rockets.drain(..).partition(|r| r.y <= r.burst_y);
        self.rockets = flying;
        for rocket in exploded {
            self.explode(rocket.x, rocket.y, rng);
        }

        for spark in &mut self.sparks {
            spark.x += spark.vx;
            spark.y += spark.vy;
            spark.vx *= DRAG;
            spark.vy += GRAVITY;
            spark.age += 1;
        }
        self.sparks.retain(|s| s.age < s.max_age && s.y >= 0.0);

        if self.cooldown > 0 {
            self.cooldown -= 1;
        } else if self.rockets.len() < MAX_ROCKETS && terminal_width > 10 {
            let lowest_burst = (horizon_y.saturating_sub(17) as f32).max(SKY_TOP + 2.0);
            self.rockets.push(Rocket {
                x: rng.random_range(4.0..(terminal_width - 4) as f32),
                y: horizon_y as f32,
                burst_y: rng.random_range(SKY_TOP..lowest_burst),
            });
            self.cooldown = rng.random_range(15..50);
        }
    }

    fn explode(&mut self, x: f32, y: f32, rng: &mut impl Rng) {
        let count = rng.random_range(18..28);
        for i in 0..count {
            if self.sparks.len() >= MAX_SPARKS {
                break;
            }
            let angle = i as f32 / count as f32 * std::f32::consts::TAU + rng.random::<f32>() * 0.2;
            let speed = rng.random_range(0.25..0.45);
            self.sparks.push(Spark {
                x,
                y,
                vx: angle.cos() * speed * 2.0,
                vy: angle.sin() * speed,
                age: 0,
                max_age: rng.random_range(18..30),
                color: PALETTE[rng.random_range(0..PALETTE.len())],
            });
        }
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        for rocket in &self.rockets {
            let x = rocket.x as u16;
            let y = rocket.y as u16;
            renderer.render_char(x, y, '|', Color::Yellow)?;
            renderer.render_char(x, y + 1, '.', Color::DarkYellow)?;
        }

        for spark in &self.sparks {
            if spark.x < 0.0 {
                continue;
            }
            let life = spark.age as f32 / spark.max_age as f32;
            let ch = if life < 0.3 {
                '*'
            } else if life < 0.6 {
                'x'
            } else {
                '\''
            };
            renderer.render_char(spark.x as u16, spark.y as u16, ch, spark.color)?;
        }
        Ok(())
    }
}

impl Default for Fireworks {
    fn default() -> Self {
        Self::new()
    }
}
