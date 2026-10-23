use crate::animation::garden::{self, GardenLayout};
use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use std::io;

const WIDTH: u16 = 7;
const RIGHT_MARGIN: u16 = 3;
const HAT: &str = " _[_]_ ";
const BODY: [&str; 3] = [" (o>o) ", "\\( : )/", "(     )"];
const CARROT: Color = Color::Rgb {
    r: 255,
    g: 140,
    b: 0,
};
const STICK: Color = Color::Rgb {
    r: 110,
    g: 75,
    b: 40,
};

#[derive(Default)]
pub struct Snowman;

impl Snowman {
    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
        has_snow_cover: bool,
    ) -> io::Result<()> {
        if !conditions.is_snowing && !has_snow_cover {
            return Ok(());
        }

        let x = layout.width.saturating_sub(WIDTH + RIGHT_MARGIN);
        let y = layout.horizon_y.saturating_sub(3);
        let snow = if conditions.is_day {
            Color::White
        } else {
            Color::Grey
        };

        garden::draw_solid(renderer, x, y, &[HAT], Color::DarkGrey)?;
        garden::draw_solid(renderer, x, y + 1, &BODY, snow)?;
        renderer.render_char(x + 2, y + 1, 'o', Color::DarkGrey)?;
        renderer.render_char(x + 3, y + 1, '>', CARROT)?;
        renderer.render_char(x + 4, y + 1, 'o', Color::DarkGrey)?;
        renderer.render_char(x, y + 2, '\\', STICK)?;
        renderer.render_char(x + 3, y + 2, ':', Color::DarkGrey)?;
        renderer.render_char(x + 6, y + 2, '/', STICK)?;
        Ok(())
    }
}
