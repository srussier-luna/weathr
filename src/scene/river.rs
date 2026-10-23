use crate::animation::duck::{PUDDLE_WIDTH, puddle_x};
use crate::render::TerminalRenderer;
use crate::scene::WorldScene;
use crate::scene::mountains::Mountains;
use chrono::Local;
use crossterm::style::Color;
use std::io;
use std::ops::Range;

const SOURCE_SEARCH: u16 = 24;
const MAX_DRIFT_PER_ROW: f32 = 0.5;
const POND_DEPTH: f32 = 5.5;
const WIGGLE_AMPLITUDE: f32 = 1.5;
const WIGGLE_FREQUENCY: f32 = 0.7;
const WIGGLE_FADE_ROWS: f32 = 3.0;
const WIDE_ROWS_ABOVE_HORIZON: u16 = 3;
const FLOW_STEP_MILLIS: i64 = 300;
const FLOWING: [char; 6] = ['~', '-', '~', '~', '-', '~'];
const FROZEN: [char; 6] = ['=', '-', '=', '=', '-', '='];
const DAY_WATER: Color = Color::Rgb {
    r: 70,
    g: 130,
    b: 220,
};
const NIGHT_WATER: Color = Color::Rgb {
    r: 40,
    g: 70,
    b: 130,
};
const ICE: Color = Color::Rgb {
    r: 200,
    g: 220,
    b: 255,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stretch {
    pub x: u16,
    pub y: u16,
    pub width: u16,
}

pub struct River {
    stretches: Vec<Stretch>,
    ridge: Vec<Option<u16>>,
    horizon_y: u16,
    pond_x: u16,
}

impl River {
    pub fn new(terminal_width: u16, horizon_y: u16) -> Self {
        let ridge = Mountains::ridge_tops(terminal_width, horizon_y);
        let pond_x = puddle_x(terminal_width);
        let target = pond_x + PUDDLE_WIDTH / 2;
        let top_at = |x: u16| ridge.get(x as usize).copied().flatten();
        let (source_x, source_y) = Self::summit_source(&ridge, target, horizon_y)
            .unwrap_or_else(|| (target, top_at(target).map_or(horizon_y, |top| top + 1)));
        let descent = horizon_y.saturating_sub(source_y) as f32;
        let reach = descent * MAX_DRIFT_PER_ROW;
        let mouth_x = (target as f32)
            .clamp(source_x as f32 - reach, source_x as f32 + reach)
            .clamp(pond_x as f32, (pond_x + PUDDLE_WIDTH - 2) as f32);
        let last_column = terminal_width.saturating_sub(2);

        let stretches = (source_y..=horizon_y)
            .map(|y| {
                let travelled = (y - source_y) as f32;
                let remaining = (horizon_y - y) as f32;
                let along = if descent > 0.0 {
                    travelled / descent
                } else {
                    1.0
                };
                let fade = (travelled.min(remaining) / WIGGLE_FADE_ROWS).min(1.0);
                let wiggle = WIGGLE_AMPLITUDE * (travelled * WIGGLE_FREQUENCY).sin() * fade;
                let x = source_x as f32 + (mouth_x - source_x as f32) * along + wiggle;
                let width = if horizon_y - y < WIDE_ROWS_ABOVE_HORIZON {
                    2
                } else {
                    1
                };
                Stretch {
                    x: (x.round().max(0.0) as u16).min(last_column),
                    y,
                    width,
                }
            })
            .collect();

        Self {
            stretches,
            ridge,
            horizon_y,
            pond_x,
        }
    }

    fn summit_source(ridge: &[Option<u16>], target: u16, horizon_y: u16) -> Option<(u16, u16)> {
        let top_at = |x: usize| ridge.get(x).copied().flatten();
        let is_summit = |x: usize| {
            top_at(x).is_some_and(|top| {
                top_at(x + 1) == Some(top)
                    && x.checked_sub(1)
                        .and_then(top_at)
                        .is_none_or(|left| left > top)
                    && top_at(x + 2).is_none_or(|right| right > top)
            })
        };
        let target = target as usize;
        (0..=SOURCE_SEARCH as usize)
            .flat_map(|distance| [target.checked_sub(distance), Some(target + distance)])
            .flatten()
            .filter(|x| is_summit(*x))
            .find_map(|x| {
                let source_y = top_at(x)? + 1;
                let reach = horizon_y.saturating_sub(source_y) as f32 * MAX_DRIFT_PER_ROW;
                let gap = (x as f32 - target as f32).abs();
                (gap <= reach + (PUDDLE_WIDTH / 2) as f32 - 1.0).then_some((x as u16, source_y))
            })
    }

    fn pond_span(&self, depth: u16) -> Range<u16> {
        let radius = PUDDLE_WIDTH as f32 / 2.0;
        let ratio = depth as f32 / POND_DEPTH;
        if ratio >= 1.0 {
            return self.pond_x..self.pond_x;
        }
        let half_width = radius * (1.0 - ratio * ratio).sqrt();
        let center = self.pond_x as f32 + radius;
        (center - half_width).round() as u16..(center + half_width).round() as u16
    }

    pub fn pond_row(&self, depth: u16) -> (u16, Range<u16>) {
        (self.horizon_y + depth, self.pond_span(depth))
    }

    pub fn stretch_count(&self) -> usize {
        self.stretches.len()
    }

    pub fn stretch(&self, index: usize) -> Option<Stretch> {
        self.stretches.get(index).copied()
    }

    pub fn is_water(&self, x: u16, y: u16) -> bool {
        y >= self.horizon_y
            || self
                .ridge
                .get(x as usize)
                .copied()
                .flatten()
                .is_some_and(|top| y > top)
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        is_day: bool,
        is_frozen: bool,
    ) -> io::Result<()> {
        let (pattern, color, shift) = match (is_frozen, is_day) {
            (true, _) => (FROZEN, ICE, 0),
            (false, true) => (FLOWING, DAY_WATER, Self::flow_shift()),
            (false, false) => (FLOWING, NIGHT_WATER, Self::flow_shift()),
        };
        let (width, _) = renderer.get_size();

        for depth in 0..WorldScene::GROUND_HEIGHT {
            let y = self.horizon_y + depth;
            for x in self.pond_span(depth).filter(|x| *x < width) {
                let index = (x as usize + depth as usize + shift) % pattern.len();
                renderer.render_char(x, y, pattern[index], color)?;
            }
        }

        for (row, stretch) in self.stretches.iter().enumerate() {
            for offset in 0..stretch.width {
                let x = stretch.x + offset;
                if x >= width || !self.is_water(x, stretch.y) {
                    continue;
                }
                let index = (row + offset as usize + pattern.len() - shift) % pattern.len();
                renderer.render_char(x, stretch.y, pattern[index], color)?;
            }
        }
        Ok(())
    }

    fn flow_shift() -> usize {
        (Local::now().timestamp_millis() / FLOW_STEP_MILLIS) as usize % FLOWING.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_river_flows_from_mountain_into_pond() {
        let horizon_y = 33;
        let river = River::new(160, horizon_y);
        let first = river.stretch(0).unwrap();
        assert!(first.y < horizon_y);

        let last = river.stretch(river.stretch_count() - 1).unwrap();
        let pond = puddle_x(160)..puddle_x(160) + PUDDLE_WIDTH;
        assert_eq!(last.y, horizon_y);
        assert!(pond.contains(&last.x) && pond.contains(&(last.x + last.width - 1)));
    }

    #[test]
    fn test_pond_is_a_half_round() {
        let river = River::new(160, 33);
        let pond_x = puddle_x(160);
        let widths: Vec<u16> = (0..WorldScene::GROUND_HEIGHT)
            .map(|depth| river.pond_span(depth).len() as u16)
            .collect();
        assert_eq!(river.pond_span(0), pond_x..pond_x + PUDDLE_WIDTH);
        assert_eq!(widths, [19, 19, 17, 15, 13, 7, 0]);
    }

    #[test]
    fn test_river_reduces_to_pond_without_mountains() {
        let river = River::new(80, 6);
        assert_eq!(river.stretch_count(), 1);
        assert_eq!(river.stretch(0).map(|stretch| stretch.y), Some(6));
    }
}
