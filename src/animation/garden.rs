use crate::render::TerminalRenderer;
use crate::scene::WorldScene;
use crate::scene::decorations::GardenPlan;
use crate::scene::house::House;
use crossterm::style::Color;
use std::io;

const WELL_WIDTH: u16 = 9;
const CHEST_WIDTH: u16 = 9;
const SLOT_GAP: u16 = 1;

#[derive(Clone, Copy)]
pub struct GardenLayout {
    pub width: u16,
    pub horizon_y: u16,
    pub house: House,
    pub house_x: u16,
    pub house_y: u16,
    pub plan: GardenPlan,
}

impl GardenLayout {
    pub fn new(term_width: u16, term_height: u16) -> Self {
        let horizon_y = term_height.saturating_sub(WorldScene::GROUND_HEIGHT);
        let house = House::for_terminal(term_width, term_height);
        let house_x = house.x_on(term_width, horizon_y);
        let house_y = horizon_y.saturating_sub(house.height());

        Self {
            width: term_width,
            horizon_y,
            house,
            house_x,
            house_y,
            plan: GardenPlan::new(house_x, horizon_y),
        }
    }

    pub fn well_x(&self) -> Option<u16> {
        let tree = self.plan.tree?;
        tree.x
            .checked_sub(SLOT_GAP + CHEST_WIDTH + SLOT_GAP + WELL_WIDTH)
    }
}

pub fn draw(
    renderer: &mut TerminalRenderer,
    x: u16,
    y: u16,
    lines: &[&str],
    color: Color,
) -> io::Result<()> {
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != ' ' {
                renderer.render_char(x + col as u16, y + row as u16, ch, color)?;
            }
        }
    }
    Ok(())
}

pub fn draw_solid(
    renderer: &mut TerminalRenderer,
    x: u16,
    y: u16,
    lines: &[&str],
    color: Color,
) -> io::Result<()> {
    for (row, line) in lines.iter().enumerate() {
        let offset = line.len() - line.trim_start().len();
        for (col, ch) in line.trim().chars().enumerate() {
            renderer.render_char(x + (offset + col) as u16, y + row as u16, ch, color)?;
        }
    }
    Ok(())
}

pub fn draw_clipped(
    renderer: &mut TerminalRenderer,
    x: i32,
    y: u16,
    lines: &[&str],
    color: Color,
) -> io::Result<()> {
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let column = x + col as i32;
            if ch != ' ' && column >= 0 {
                renderer.render_char(column as u16, y + row as u16, ch, color)?;
            }
        }
    }
    Ok(())
}

pub fn scatter(position: u16, seed: u32) -> f32 {
    let mixed = (position as u32 ^ seed.wrapping_mul(0x9E37_79B9))
        .wrapping_mul(0x85EB_CA6B)
        .rotate_left(13)
        .wrapping_mul(0xC2B2_AE35);
    (mixed % 1000) as f32 / 1000.0
}

pub struct Bubble {
    timer: u16,
}

impl Bubble {
    pub fn new() -> Self {
        Self { timer: 0 }
    }

    pub fn update(&mut self, chance: f32, duration: u16, roll: f32) {
        if self.timer > 0 {
            self.timer -= 1;
        } else if roll < chance {
            self.timer = duration;
        }
    }

    pub fn show(&mut self, duration: u16) {
        self.timer = duration;
    }

    pub fn tick(&mut self) {
        self.timer = self.timer.saturating_sub(1);
    }

    pub fn is_visible(&self) -> bool {
        self.timer > 0
    }

    pub fn hide(&mut self) {
        self.timer = 0;
    }
}

pub struct Blink {
    countdown: u16,
    closed_for: u16,
}

impl Blink {
    pub fn new() -> Self {
        Self {
            countdown: 90,
            closed_for: 0,
        }
    }

    pub fn update(&mut self, next_countdown: u16) {
        if self.closed_for > 0 {
            self.closed_for -= 1;
        } else if self.countdown > 0 {
            self.countdown -= 1;
        } else {
            self.closed_for = 5;
            self.countdown = next_countdown;
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed_for > 0
    }
}

impl Default for Bubble {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Blink {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_well_fits_left_of_tree_on_wide_terminal() {
        let layout = GardenLayout::new(200, 40);
        let tree = layout.plan.tree.unwrap();
        let well_x = layout.well_x().unwrap();
        assert!(well_x + WELL_WIDTH + SLOT_GAP + CHEST_WIDTH + SLOT_GAP <= tree.x);
    }

    #[test]
    fn test_well_hidden_on_narrow_terminal() {
        let layout = GardenLayout::new(120, 30);
        assert_eq!(layout.well_x(), None);
    }
}
