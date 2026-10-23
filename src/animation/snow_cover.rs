use crate::animation::garden::{GardenLayout, scatter};
use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

const BUILD_RATE: f32 = 1.0 / 900.0;
const MELT_RATE: f32 = 1.0 / 1800.0;
const GROUND_SNOW: [char; 4] = ['_', '-', '~', '.'];

pub struct SnowCover {
    level: f32,
}

impl SnowCover {
    pub fn new() -> Self {
        Self { level: 0.0 }
    }

    pub fn update(&mut self, is_snowing: bool) {
        self.level = if is_snowing {
            (self.level + BUILD_RATE).min(1.0)
        } else {
            (self.level - MELT_RATE).max(0.0)
        };
    }

    pub fn is_thick(&self) -> bool {
        self.level > 0.5
    }

    pub fn render(&self, renderer: &mut TerminalRenderer, layout: &GardenLayout) -> io::Result<()> {
        if self.level <= 0.0 {
            return Ok(());
        }

        for x in 0..layout.width {
            if scatter(x, 3) < self.level {
                let ch = GROUND_SNOW[(scatter(x, 5) * GROUND_SNOW.len() as f32) as usize];
                renderer.render_char(x, layout.horizon_y, ch, Color::White)?;
            }
        }

        let ascii = layout.house.get_ascii();
        let house_width = ascii.iter().map(|line| line.len()).max().unwrap_or(0);
        for column in 0..house_width {
            let x = layout.house_x + column as u16;
            let top = ascii.iter().enumerate().find_map(|(row, line)| {
                line.chars()
                    .nth(column)
                    .filter(|ch| !matches!(ch, ' ' | '(' | ')'))
                    .map(|ch| (row, ch))
            });
            if let Some((row, ch)) = top
                && scatter(x, 9) < self.level
            {
                renderer.render_char(x, layout.house_y + row as u16, ch, Color::White)?;
            }
        }

        if let Some(tree) = layout.plan.tree {
            for x in tree.x..tree.x + tree.width {
                let top = (0..tree.canopy_height).find(|row| {
                    let (start, end) = tree.canopy_span(*row);
                    (start..end).contains(&x)
                });
                if let Some(row) = top
                    && scatter(x, 17) < self.level
                {
                    let y = tree.top_y + row;
                    let ch = renderer
                        .char_at(x, y)
                        .filter(|ch| *ch != ' ')
                        .unwrap_or('_');
                    renderer.render_char(x, y, ch, Color::White)?;
                }
            }
        }
        Ok(())
    }
}

impl Default for SnowCover {
    fn default() -> Self {
        Self::new()
    }
}
