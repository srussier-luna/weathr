use crate::animation::garden::{self, GardenLayout};
use crate::render::TerminalRenderer;
use crate::scene::tree::Tree;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

const CLIMB_SPEED: f32 = 0.12;
const RUN_SPEED: f32 = 0.2;
const SPRITE_RIGHT: [&str; 2] = ["@  ,", "(_/o>"];
const SPRITE_LEFT: [&str; 2] = [" ,  @", "<o\\_)"];
const FUR: Color = Color::Rgb {
    r: 190,
    g: 110,
    b: 50,
};

enum Phase {
    Resting(u16),
    Running(f32),
    ClimbingUp,
    Hiding(u16),
    ClimbingDown,
}

pub struct Squirrel {
    phase: Phase,
    x: f32,
    y: f32,
    is_facing_right: bool,
    frame: u32,
    is_placed: bool,
}

impl Squirrel {
    pub fn new() -> Self {
        Self {
            phase: Phase::Resting(60),
            x: 0.0,
            y: 0.0,
            is_facing_right: true,
            frame: 0,
            is_placed: false,
        }
    }

    fn is_visible(conditions: &WeatherConditions) -> bool {
        conditions.is_day && !conditions.is_raining && !conditions.is_thunderstorm
    }

    fn ground_y(layout: &GardenLayout) -> f32 {
        layout.horizon_y.saturating_sub(2) as f32
    }

    fn climb_x(tree: &Tree) -> f32 {
        tree.trunk_x().saturating_sub(2) as f32
    }

    pub fn update(
        &mut self,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
        rng: &mut impl Rng,
    ) {
        let Some(tree) = layout.plan.tree.filter(|_| Self::is_visible(conditions)) else {
            self.is_placed = false;
            return;
        };

        self.frame = self.frame.wrapping_add(1);
        let ground_y = Self::ground_y(layout);
        let climb_x = Self::climb_x(&tree);
        if !self.is_placed {
            self.x = climb_x;
            self.y = ground_y;
            self.phase = Phase::Resting(60);
            self.is_placed = true;
        }

        self.phase = match self.phase {
            Phase::Resting(remaining) if remaining > 0 => Phase::Resting(remaining - 1),
            Phase::Resting(_) => {
                if (self.x - climb_x).abs() < RUN_SPEED && rng.random::<f32>() < 0.6 {
                    Phase::ClimbingUp
                } else {
                    let min_x = tree.x.saturating_sub(4).max(1) as f32;
                    let max_x = (tree.x + tree.width + 2) as f32;
                    let target = if rng.random::<f32>() < 0.5 {
                        climb_x
                    } else {
                        rng.random_range(min_x..max_x)
                    };
                    self.is_facing_right = target > self.x;
                    Phase::Running(target)
                }
            }
            Phase::Running(target) => {
                if (target - self.x).abs() <= RUN_SPEED {
                    self.x = target;
                    Phase::Resting(rng.random_range(30..120))
                } else {
                    self.x += RUN_SPEED.copysign(target - self.x);
                    Phase::Running(target)
                }
            }
            Phase::ClimbingUp => {
                self.y -= CLIMB_SPEED;
                if self.y <= tree.canopy_bottom_y() as f32 - 1.0 {
                    Phase::Hiding(rng.random_range(90..300))
                } else {
                    Phase::ClimbingUp
                }
            }
            Phase::Hiding(remaining) if remaining > 0 => Phase::Hiding(remaining - 1),
            Phase::Hiding(_) => Phase::ClimbingDown,
            Phase::ClimbingDown => {
                self.y += CLIMB_SPEED;
                if self.y >= ground_y {
                    self.y = ground_y;
                    Phase::Resting(rng.random_range(30..90))
                } else {
                    Phase::ClimbingDown
                }
            }
        };
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        if !self.is_placed || matches!(self.phase, Phase::Hiding(_)) {
            return Ok(());
        }

        let hop = matches!(self.phase, Phase::Running(_)) && (self.frame / 4) % 2 == 0;
        let y = (self.y as u16).saturating_sub(u16::from(hop));
        let sprite = if self.is_facing_right {
            &SPRITE_RIGHT
        } else {
            &SPRITE_LEFT
        };
        garden::draw(renderer, self.x as u16, y, sprite, FUR)
    }
}

impl Default for Squirrel {
    fn default() -> Self {
        Self::new()
    }
}
