use crate::animation::garden::GardenLayout;
use crate::animation::lore::Unlocks;
use crate::render::TerminalRenderer;
use crate::scene::river::River;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

const BOTTLE_SPEED: f32 = 1.0 / 75.0;
const FIRST_OBJECT_WAIT: u32 = 1800;
const FIRST_BOTTLE_WAIT: u32 = 1200;

pub struct Finds {
    object_x: Option<u16>,
    object_wait: u32,
    bottle: Option<f32>,
    bottle_wait: u32,
    has_objects_left: bool,
    has_bottles_left: bool,
    frame: u32,
}

impl Finds {
    pub fn new() -> Self {
        Self {
            object_x: None,
            object_wait: FIRST_OBJECT_WAIT,
            bottle: None,
            bottle_wait: FIRST_BOTTLE_WAIT,
            has_objects_left: true,
            has_bottles_left: true,
            frame: 0,
        }
    }

    pub fn set_remaining(&mut self, has_objects_left: bool, has_bottles_left: bool) {
        self.has_objects_left = has_objects_left;
        self.has_bottles_left = has_bottles_left;
    }

    pub fn has_object(&self) -> bool {
        self.object_x.is_some()
    }

    pub fn has_bottle(&self) -> bool {
        self.bottle.is_some()
    }

    pub fn take_object(&mut self, rng: &mut impl Rng) -> bool {
        let taken = self.object_x.take().is_some();
        if taken {
            self.object_wait = rng.random_range(18000..54000);
        }
        taken
    }

    pub fn take_bottle(&mut self, rng: &mut impl Rng) -> bool {
        let taken = self.bottle.take().is_some();
        if taken {
            self.bottle_wait = rng.random_range(27000..81000);
        }
        taken
    }

    pub fn update(
        &mut self,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
        unlocks: &Unlocks,
        river: &River,
        is_frozen: bool,
        rng: &mut impl Rng,
    ) {
        self.frame = self.frame.wrapping_add(1);
        let width = layout.width;

        if unlocks.found_objects && self.has_objects_left && conditions.is_day {
            if self.object_x.is_none() {
                if self.object_wait > 0 {
                    self.object_wait -= 1;
                } else if width > 6 {
                    self.object_x = Some(rng.random_range(2..width - 3));
                }
            }
        } else {
            self.object_x = None;
        }

        if !unlocks.bottles || !self.has_bottles_left || is_frozen {
            self.bottle = None;
            return;
        }
        match self.bottle {
            Some(position) if position >= river.stretch_count() as f32 => {
                self.bottle = None;
                self.bottle_wait = rng.random_range(27000..81000);
            }
            Some(position) => self.bottle = Some(position + BOTTLE_SPEED),
            None if self.bottle_wait > 0 => self.bottle_wait -= 1,
            None => self.bottle = Some(0.0),
        }
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        river: &River,
    ) -> io::Result<()> {
        if let Some(x) = self.object_x {
            let color = if (self.frame / 10) % 2 == 0 {
                Color::Yellow
            } else {
                Color::White
            };
            renderer.render_char(x, layout.horizon_y, '?', color)?;
        }

        if let Some(stretch) = self
            .bottle
            .and_then(|position| river.stretch(position as usize))
        {
            let sprite = match stretch.width {
                1 => "o",
                2 => "[]",
                _ => "=[]",
            };
            let start_x = stretch.x + (stretch.width - sprite.len() as u16) / 2;
            for (offset, ch) in sprite.chars().enumerate() {
                let x = start_x + offset as u16;
                if river.is_water(x, stretch.y) {
                    renderer.render_char(x, stretch.y, ch, Color::Cyan)?;
                }
            }
        }
        Ok(())
    }
}

impl Default for Finds {
    fn default() -> Self {
        Self::new()
    }
}
