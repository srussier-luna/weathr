use crate::animation::duck::puddle_x;
use crate::animation::garden::{self, GardenLayout};
use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use std::io;

const SPEED: f32 = 0.02;
const START_GAP: u16 = 12;
const BODY: Color = Color::Rgb {
    r: 200,
    g: 180,
    b: 140,
};
const SHELL: Color = Color::Rgb {
    r: 170,
    g: 110,
    b: 60,
};

pub struct Snail {
    x: Option<f32>,
}

impl Snail {
    pub fn new() -> Self {
        Self { x: None }
    }

    fn is_visible(conditions: &WeatherConditions) -> bool {
        conditions.is_raining || conditions.is_thunderstorm
    }

    pub fn update(&mut self, layout: &GardenLayout, conditions: &WeatherConditions) {
        if !Self::is_visible(conditions) {
            self.x = None;
            return;
        }

        let start = puddle_x(layout.width).saturating_sub(START_GAP) as f32;
        let x = self.x.unwrap_or(start) - SPEED;
        self.x = Some(if x < 1.0 { start } else { x });
    }

    pub fn render(&self, renderer: &mut TerminalRenderer, layout: &GardenLayout) -> io::Result<()> {
        let Some(x) = self.x else {
            return Ok(());
        };

        let x = x as u16;
        let y = layout.horizon_y.saturating_sub(1);
        renderer.render_char(x, y, '\\', BODY)?;
        garden::draw(renderer, x + 2, y, &[".-."], SHELL)?;
        renderer.render_char(x + 1, y + 1, '\\', BODY)?;
        garden::draw(renderer, x + 2, y + 1, &["(@)"], SHELL)?;
        renderer.render_char(x + 5, y + 1, '_', BODY)
    }
}

impl Default for Snail {
    fn default() -> Self {
        Self::new()
    }
}
