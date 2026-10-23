use crate::animation::garden::{self, Blink, GardenLayout};
use crate::animation::lore::Unlocks;
use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

const DAY_FUR: Color = Color::Rgb {
    r: 230,
    g: 140,
    b: 40,
};
const NIGHT_FUR: Color = Color::Rgb {
    r: 120,
    g: 120,
    b: 120,
};
const DAY_EYES: Color = Color::Rgb {
    r: 120,
    g: 200,
    b: 80,
};

pub struct Cat {
    blink: Blink,
    frame: u32,
    is_staring: bool,
}

impl Cat {
    pub fn new() -> Self {
        Self {
            blink: Blink::new(),
            frame: 0,
            is_staring: false,
        }
    }

    pub fn set_staring(&mut self, is_staring: bool) {
        self.is_staring = is_staring;
    }

    pub fn update(&mut self, rng: &mut impl Rng) {
        self.frame = self.frame.wrapping_add(1);
        self.blink.update(rng.random_range(60..200));
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
        unlocks: &Unlocks,
    ) -> io::Result<()> {
        let (fur, eyes_color) = if conditions.is_day {
            (DAY_FUR, DAY_EYES)
        } else {
            (NIGHT_FUR, Color::Yellow)
        };

        if conditions.is_raining || conditions.is_thunderstorm || conditions.is_snowing {
            self.render_in_window(renderer, layout, fur, eyes_color)?;
            if unlocks.cat_bow {
                let (window_x, window_y) = layout.house.big_window();
                let x = layout.house_x + window_x + 1;
                renderer.render_char(x, layout.house_y + window_y, '8', bow_color(unlocks))?;
            }
        } else {
            let is_sunny = conditions.is_day
                && !conditions.is_cloudy
                && !conditions.is_foggy
                && !self.is_staring;
            self.render_on_roof(renderer, layout, fur, eyes_color, is_sunny)?;
            let (perch_x, perch_y) = layout.house.roof_perch();
            let x = layout.house_x + perch_x;
            let y = layout.house_y + perch_y;
            if unlocks.cat_bow {
                renderer.render_char(x + 6, y, '8', bow_color(unlocks))?;
            }
            if unlocks.kittens {
                let kittens = if unlocks.choices.white_kittens {
                    Color::White
                } else {
                    Color::DarkGrey
                };
                garden::draw(renderer, x + 8, y + 1, &["=^.^= =^.^="], kittens)?;
            }
        }
        Ok(())
    }

    fn render_in_window(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        fur: Color,
        eyes_color: Color,
    ) -> io::Result<()> {
        let (window_x, window_y) = layout.house.big_window();
        let x = layout.house_x + window_x;
        let y = layout.house_y + window_y;
        let face = if self.is_staring {
            "O.O"
        } else if self.blink.is_closed() {
            "-.-"
        } else {
            "o.o"
        };

        garden::draw_solid(renderer, x, y, &["^ ^"], fur)?;
        garden::draw_solid(renderer, x, y + 1, &[face], fur)?;
        if !self.blink.is_closed() {
            renderer.render_char(x, y + 1, 'o', eyes_color)?;
            renderer.render_char(x + 2, y + 1, 'o', eyes_color)?;
        }
        Ok(())
    }

    fn render_on_roof(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        fur: Color,
        eyes_color: Color,
        is_sleeping: bool,
    ) -> io::Result<()> {
        let (perch_x, perch_y) = layout.house.roof_perch();
        let x = layout.house_x + perch_x;
        let y = layout.house_y + perch_y;
        let eyes_closed = !self.is_staring && (is_sleeping || self.blink.is_closed());
        let (face, eye) = if self.is_staring {
            ("( O.O )", 'O')
        } else if eyes_closed {
            ("( -.- )", '-')
        } else {
            ("( o.o )", 'o')
        };

        garden::draw(renderer, x, y, &[" /\\_/\\ ", face], fur)?;

        if !eyes_closed {
            renderer.render_char(x + 2, y + 1, eye, eyes_color)?;
            renderer.render_char(x + 4, y + 1, eye, eyes_color)?;
        }

        if is_sleeping {
            let phase = (self.frame / 20) % 3;
            if phase >= 1 {
                renderer.render_char(x + 7, y.saturating_sub(1), 'z', Color::White)?;
            }
            if phase >= 2 {
                renderer.render_char(x + 8, y.saturating_sub(2), 'Z', Color::White)?;
            }
        }
        Ok(())
    }
}

fn bow_color(unlocks: &Unlocks) -> Color {
    if unlocks.choices.bow_blue {
        Color::Blue
    } else {
        Color::Red
    }
}

impl Default for Cat {
    fn default() -> Self {
        Self::new()
    }
}
