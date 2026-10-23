use crate::animation::garden::scatter;
use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

const SKY_RESERVED_ROWS: u16 = 9;
const MIN_PEAK_HEIGHT: u16 = 3;
const MAX_PEAK_HEIGHT: u16 = 16;
const MIN_SPACING: f32 = 10.0;
const SPACING_RANGE: f32 = 10.0;
const DAY_ROCK: Color = Color::Rgb {
    r: 120,
    g: 130,
    b: 150,
};
const NIGHT_ROCK: Color = Color::Rgb {
    r: 60,
    g: 65,
    b: 85,
};

#[derive(Default)]
pub struct Mountains;

impl Mountains {
    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        terminal_width: u16,
        horizon_y: u16,
        is_day: bool,
        is_snowy: bool,
    ) -> io::Result<()> {
        match Self::max_height(horizon_y) {
            Some(max_height) => self.render_peaks(
                renderer,
                terminal_width,
                horizon_y,
                max_height,
                is_day,
                is_snowy,
            ),
            None => Ok(()),
        }
    }

    pub fn ridge_tops(terminal_width: u16, horizon_y: u16) -> Vec<Option<u16>> {
        match Self::max_height(horizon_y) {
            Some(max_height) => Self::ridge(terminal_width, horizon_y, max_height)
                .into_iter()
                .map(|cell| cell.map(|(y, _)| y))
                .collect(),
            None => vec![None; terminal_width as usize],
        }
    }

    fn max_height(horizon_y: u16) -> Option<u16> {
        let max_height = horizon_y
            .saturating_sub(SKY_RESERVED_ROWS)
            .min(MAX_PEAK_HEIGHT);
        (max_height >= MIN_PEAK_HEIGHT).then_some(max_height)
    }

    fn render_peaks(
        &self,
        renderer: &mut TerminalRenderer,
        terminal_width: u16,
        horizon_y: u16,
        max_height: u16,
        is_day: bool,
        is_snowy: bool,
    ) -> io::Result<()> {
        let ridge = Self::ridge(terminal_width, horizon_y, max_height);
        let (rock, snow) = if is_day {
            (DAY_ROCK, Color::White)
        } else {
            (NIGHT_ROCK, Color::Grey)
        };
        let snow_line = horizon_y - max_height * 2 / 3;

        for (column, cell) in ridge.iter().enumerate() {
            let Some((y, ch)) = *cell else {
                continue;
            };
            let x = column as u16;
            renderer.render_char(x, y, ch, rock)?;
            for fill_y in y + 1..horizon_y {
                renderer.render_char(x, fill_y, ' ', rock)?;
            }
            if is_snowy {
                for snow_y in y + 1..=snow_line.min(horizon_y - 1) {
                    renderer.render_char(x, snow_y, '*', snow)?;
                }
            }
        }
        Ok(())
    }

    fn ridge(terminal_width: u16, horizon_y: u16, max_height: u16) -> Vec<Option<(u16, char)>> {
        let mut ridge = vec![None; terminal_width as usize];
        let mut peak_x = -4.0;
        let mut index = 0;

        while peak_x < terminal_width as f32 + 8.0 {
            let min_height = (max_height / 2).max(MIN_PEAK_HEIGHT);
            let range = (max_height - min_height + 1) as f32;
            let height = (min_height + (scatter(index, 31) * range) as u16).min(max_height);
            let top = horizon_y - height;
            for step in 0..height {
                let y = top + step;
                Self::plot(&mut ridge, peak_x as i32 - step as i32, y, '/');
                Self::plot(&mut ridge, peak_x as i32 + 1 + step as i32, y, '\\');
            }
            peak_x += MIN_SPACING + scatter(index, 37) * SPACING_RANGE;
            index += 1;
        }
        ridge
    }

    fn plot(ridge: &mut [Option<(u16, char)>], x: i32, y: u16, ch: char) {
        let Some(cell) = usize::try_from(x).ok().and_then(|x| ridge.get_mut(x)) else {
            return;
        };
        if cell.is_none_or(|(current_y, _)| y < current_y) {
            *cell = Some((y, ch));
        }
    }
}
