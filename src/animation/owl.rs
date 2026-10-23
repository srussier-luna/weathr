use crate::animation::garden::{self, Blink, Bubble, GardenLayout};
use crate::animation::lore::Unlocks;
use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

const FEATHERS: Color = Color::Rgb {
    r: 160,
    g: 120,
    b: 80,
};
const BRANCH: Color = Color::Rgb {
    r: 110,
    g: 75,
    b: 40,
};

pub struct Owl {
    blink: Blink,
    hoot: Bubble,
    is_staring: bool,
}

impl Owl {
    pub fn new() -> Self {
        Self {
            blink: Blink::new(),
            hoot: Bubble::new(),
            is_staring: false,
        }
    }

    pub fn set_staring(&mut self, is_staring: bool) {
        self.is_staring = is_staring;
    }

    pub fn update(&mut self, conditions: &WeatherConditions, rng: &mut impl Rng) {
        if conditions.is_day {
            self.hoot.hide();
            return;
        }

        self.blink.update(rng.random_range(90..240));
        self.hoot.update(0.002, 45, rng.random::<f32>());
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
        unlocks: &Unlocks,
    ) -> io::Result<()> {
        let Some(tree) = layout.plan.tree else {
            return Ok(());
        };
        if conditions.is_day {
            return Ok(());
        }

        let x = tree.center_x() - 2;
        let y = tree.top_y + (tree.canopy_height / 2).saturating_sub(1);
        let is_closed = self.blink.is_closed() && !self.is_staring;
        let (head, eye) = if self.is_staring {
            ("(O,O)", 'O')
        } else if is_closed {
            ("(-,-)", '-')
        } else {
            ("(o,o)", 'o')
        };

        let owls = if unlocks.owl_friend {
            vec![x, x + 6]
        } else {
            vec![x]
        };
        for owl_x in owls {
            garden::draw_solid(renderer, owl_x, y, &[head, "( : )"], FEATHERS)?;
            garden::draw_solid(renderer, owl_x, y + 2, &["-\"-\"-"], BRANCH)?;
            if !is_closed {
                renderer.render_char(owl_x + 1, y, eye, Color::Yellow)?;
                renderer.render_char(owl_x + 3, y, eye, Color::Yellow)?;
            }
        }

        if self.hoot.is_visible() && !unlocks.choices.owl_silent {
            garden::draw_solid(renderer, x, y.saturating_sub(2), &["Hou!"], Color::White)?;
        }
        Ok(())
    }
}

impl Default for Owl {
    fn default() -> Self {
        Self::new()
    }
}
