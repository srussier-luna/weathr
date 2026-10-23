use crate::animation::garden::scatter;
use crate::holiday::{Holiday, Season};
use crate::render::TerminalRenderer;
use chrono::Local;
use crossterm::style::Color;
use std::io;

const TOP_MARGIN: u16 = 7;
const MIN_HEIGHT: u16 = 5;
const MAX_HEIGHT: u16 = 20;
const MAX_WIDTH: u16 = 24;
const HOUSE_GAP: u16 = 2;
const BLOSSOM_DENSITY: f32 = 0.12;
const NIGHT_DIMMING: f32 = 0.45;
const FIR_TIERS: u16 = 3;
const FIR_NOTCH: u16 = 2;
const BAUBLE_DENSITY: f32 = 0.12;
const SNOWY_ROW_STEP: u16 = 4;
const BLINK_MILLIS: i64 = 500;
const LIGHTS: [Color; 4] = [Color::Red, Color::Yellow, Color::Cyan, Color::Magenta];
const FIR_GREEN: Color = Color::Rgb {
    r: 20,
    g: 120,
    b: 40,
};
const AUTUMN_LEAVES: [(u8, u8, u8); 4] =
    [(230, 120, 30), (200, 60, 30), (220, 180, 40), (150, 90, 40)];
const TRUNK_COLOR: Color = Color::Rgb {
    r: 110,
    g: 75,
    b: 40,
};

#[derive(Clone, Copy)]
pub struct Tree {
    pub x: u16,
    pub top_y: u16,
    pub width: u16,
    pub canopy_height: u16,
    pub trunk_height: u16,
}

impl Tree {
    pub fn fit(house_x: u16, horizon_y: u16) -> Option<Self> {
        let max_height = horizon_y.saturating_sub(TOP_MARGIN).min(MAX_HEIGHT);
        (MIN_HEIGHT..=max_height)
            .rev()
            .find_map(|height| Self::with_height(height, house_x, horizon_y))
    }

    fn with_height(height: u16, house_x: u16, horizon_y: u16) -> Option<Self> {
        let trunk_height = (height / 4).max(1);
        let canopy_height = height - trunk_height;
        let width = (canopy_height * 3 / 2 + 6).min(MAX_WIDTH);
        let x = house_x.checked_sub(width + HOUSE_GAP).filter(|x| *x > 0)?;

        Some(Self {
            x,
            top_y: horizon_y - height,
            width,
            canopy_height,
            trunk_height,
        })
    }

    pub fn center_x(&self) -> u16 {
        self.x + self.width / 2
    }

    pub fn canopy_bottom_y(&self) -> u16 {
        self.top_y + self.canopy_height
    }

    pub fn trunk_width(&self) -> u16 {
        if self.canopy_height >= 8 { 4 } else { 2 }
    }

    pub fn trunk_x(&self) -> u16 {
        self.center_x() - self.trunk_width() / 2
    }

    pub fn canopy_span(&self, row: u16) -> (u16, u16) {
        let t = (2 * row + 1) as f32 / self.canopy_height as f32 - 1.0;
        let half = ((self.width / 2) as f32 * (1.0 - t * t).sqrt())
            .round()
            .max(1.0) as u16;
        (self.center_x() - half, self.center_x() + half)
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        is_day: bool,
        holiday: &Holiday,
        is_snowing: bool,
    ) -> io::Result<()> {
        let is_fir = holiday.is_december || holiday.season == Season::Winter;
        if is_fir {
            self.render_fir(renderer, holiday.is_december, is_day, is_snowing)?;
        } else {
            for row in 0..self.canopy_height {
                let (start, end) = self.canopy_span(row);
                for x in start..end {
                    let cell = scatter(x.wrapping_mul(31).wrapping_add(row), 41);
                    if let Some((ch, color)) = self.foliage(cell, holiday.season) {
                        renderer.render_char(x, self.top_y + row, ch, dim(color, is_day))?;
                    }
                }
            }
        }

        let bark = "|".repeat(self.trunk_width() as usize);
        let trunk_x = self.trunk_x();
        let roots_y = self.canopy_bottom_y() + self.trunk_height - 1;
        for y in self.canopy_bottom_y()..roots_y {
            renderer.render_line_colored(trunk_x, y, &bark, TRUNK_COLOR)?;
        }
        renderer.render_line_colored(
            trunk_x.saturating_sub(1),
            roots_y,
            &format!("_{bark}_"),
            TRUNK_COLOR,
        )?;

        if is_fir && holiday.is_christmas {
            self.render_presents(renderer, roots_y)?;
        }
        Ok(())
    }

    fn fir_half_width(&self, row: u16) -> u16 {
        let tier_rows = (self.canopy_height / FIR_TIERS).max(2);
        let full =
            ((row + 1) as f32 / self.canopy_height as f32 * (self.width / 2) as f32).round() as u16;
        let notch = if row > 0 && row % tier_rows == 0 {
            FIR_NOTCH
        } else {
            0
        };
        if row == 0 {
            0
        } else {
            full.saturating_sub(notch).max(1)
        }
    }

    fn render_fir(
        &self,
        renderer: &mut TerminalRenderer,
        is_decorated: bool,
        is_day: bool,
        is_snowing: bool,
    ) -> io::Result<()> {
        let tick = (Local::now().timestamp_millis() / BLINK_MILLIS) as usize;
        let center = self.center_x();

        for row in 0..self.canopy_height {
            let half = self.fir_half_width(row);
            let y = self.top_y + row;
            for x in center - half..=center + half {
                let is_edge = x == center - half || x == center + half;
                let cell = scatter(x.wrapping_mul(31).wrapping_add(row), 43);
                let (ch, color) = if is_decorated && row == 0 {
                    (if tick % 2 == 0 { '*' } else { '+' }, Color::Yellow)
                } else if is_decorated && !is_edge && cell < BAUBLE_DENSITY {
                    (
                        'o',
                        LIGHTS[(x as usize + row as usize + tick) % LIGHTS.len()],
                    )
                } else if is_snowing && (is_edge || row % SNOWY_ROW_STEP == 0) {
                    ('*', Color::White)
                } else {
                    ('*', dim(FIR_GREEN, is_day))
                };
                renderer.render_char(x, y, ch, color)?;
            }
        }
        Ok(())
    }

    fn render_presents(&self, renderer: &mut TerminalRenderer, y: u16) -> io::Result<()> {
        let left = self.trunk_x().saturating_sub(5);
        let right = self.trunk_x() + self.trunk_width() + 2;
        renderer.render_line_colored(left, y, "[+]", Color::Red)?;
        renderer.render_char(left + 1, y, '+', Color::Yellow)?;
        renderer.render_line_colored(right, y, "[+]", Color::Blue)?;
        renderer.render_char(right + 1, y, '+', Color::White)
    }

    fn foliage(&self, cell: f32, season: Season) -> Option<(char, Color)> {
        match season {
            Season::Spring if cell < BLOSSOM_DENSITY => Some((
                '*',
                Color::Rgb {
                    r: 255,
                    g: 170,
                    b: 200,
                },
            )),
            Season::Spring => Some((
                '#',
                Color::Rgb {
                    r: 110,
                    g: 190,
                    b: 80,
                },
            )),
            Season::Summer => Some(('#', Color::Rgb { r: 0, g: 130, b: 0 })),
            Season::Autumn => {
                let (r, g, b) = AUTUMN_LEAVES[(cell * AUTUMN_LEAVES.len() as f32) as usize];
                Some(('#', Color::Rgb { r, g, b }))
            }
            Season::Winter => None,
        }
    }
}

fn dim(color: Color, is_day: bool) -> Color {
    match color {
        Color::Rgb { r, g, b } if !is_day => Color::Rgb {
            r: (r as f32 * NIGHT_DIMMING) as u8,
            g: (g as f32 * NIGHT_DIMMING) as u8,
            b: (b as f32 * NIGHT_DIMMING) as u8,
        },
        _ => color,
    }
}
