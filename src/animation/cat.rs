use crate::animation::garden::{self, Blink, GardenLayout};
use crate::animation::lore::Unlocks;
use crate::render::TerminalRenderer;
use crate::scene::house::House;
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
const KITTEN: &str = "=^.^=";
const RIGHT_KITTEN_OFFSET: u16 = 3;

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

        if Self::is_indoors(conditions) {
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
                for (kitten_x, kitten_y) in kitten_seats(&layout.house) {
                    garden::draw(
                        renderer,
                        layout.house_x + kitten_x,
                        layout.house_y + kitten_y,
                        &[KITTEN],
                        kittens,
                    )?;
                }
            }
        }
        Ok(())
    }

    pub fn head(&self, layout: &GardenLayout, conditions: &WeatherConditions) -> (u16, u16) {
        let (x, y) = if Self::is_indoors(conditions) {
            layout.house.big_window()
        } else {
            let (perch_x, perch_y) = layout.house.roof_perch();
            (perch_x + 2, perch_y)
        };
        (layout.house_x + x + 1, layout.house_y + y)
    }

    fn is_indoors(conditions: &WeatherConditions) -> bool {
        conditions.is_raining || conditions.is_thunderstorm || conditions.is_snowing
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

fn kitten_seats(house: &House) -> impl Iterator<Item = (u16, u16)> {
    let (perch_x, _) = house.roof_perch();
    let width = KITTEN.len() as u16;
    [perch_x - width, perch_x + RIGHT_KITTEN_OFFSET]
        .into_iter()
        .filter_map(move |x| Some((x, house.roof_seat(x..x + width)?)))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kittens_sit_on_the_roof_next_to_minouche() {
        for house in [House::for_terminal(80, 24), House::for_terminal(200, 40)] {
            let ascii = house.get_ascii();
            let (perch_x, perch_y) = house.roof_perch();
            assert_eq!(kitten_seats(&house).count(), 2);
            for (x, y) in kitten_seats(&house) {
                let below = ascii[y as usize + 1].as_bytes();
                for column in x..x + KITTEN.len() as u16 {
                    assert_ne!(below[column as usize], b' ');
                }
                let beside_minouche = x + KITTEN.len() as u16 <= perch_x || y > perch_y + 1;
                assert!(beside_minouche);
                assert!(x.abs_diff(perch_x) <= KITTEN.len() as u16);
            }
        }
    }
}
