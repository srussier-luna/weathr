use crate::animation::garden::{self, GardenLayout};
use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

const SPEED: f32 = 0.4;
const DOOR_GAP: u16 = 1;
const STEP_COLUMNS: i32 = 2;
const HAT: &str = "   /\\*";
const GIFT: &str = "[]";
const FACE: &str = "(oo)";
const FACE_COLUMN: i32 = 2;
const LEGS: [&str; 2] = ["/##\\", "|##|"];
const ELF_RED: Color = Color::Rgb {
    r: 210,
    g: 40,
    b: 50,
};
const ELF_GREEN: Color = Color::Rgb {
    r: 40,
    g: 160,
    b: 70,
};
const CHOCOLATE_BROWN: Color = Color::Rgb {
    r: 150,
    g: 90,
    b: 50,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Trip {
    Waiting,
    Arriving,
    Staying,
    Leaving,
}

pub struct Chocolutin {
    trip: Trip,
    x: f32,
}

impl Chocolutin {
    pub fn new() -> Self {
        Self {
            trip: Trip::Waiting,
            x: 0.0,
        }
    }

    pub fn visit(&mut self, layout: &GardenLayout) {
        self.x = f32::from(layout.width);
        self.trip = Trip::Arriving;
    }

    pub fn leave(&mut self) {
        if matches!(self.trip, Trip::Arriving | Trip::Staying) {
            self.trip = Trip::Leaving;
        }
    }

    pub fn is_visiting(&self) -> bool {
        matches!(self.trip, Trip::Arriving | Trip::Staying)
    }

    pub fn has_arrived(&self) -> bool {
        matches!(self.trip, Trip::Staying)
    }

    fn stop_x(layout: &GardenLayout) -> f32 {
        f32::from(layout.house_x + layout.house.width() + DOOR_GAP)
    }

    pub fn update(&mut self, layout: &GardenLayout) {
        let stop_x = Self::stop_x(layout);
        self.trip = match self.trip {
            Trip::Arriving => {
                self.x -= SPEED;
                if self.x <= stop_x {
                    self.x = stop_x;
                    Trip::Staying
                } else {
                    Trip::Arriving
                }
            }
            Trip::Leaving => {
                self.x += SPEED;
                if self.x > f32::from(layout.width) {
                    Trip::Waiting
                } else {
                    Trip::Leaving
                }
            }
            trip => trip,
        };
    }

    pub fn render(&self, renderer: &mut TerminalRenderer, layout: &GardenLayout) -> io::Result<()> {
        if matches!(self.trip, Trip::Waiting) {
            return Ok(());
        }
        let x = self.x.round() as i32;
        let y = layout.horizon_y.saturating_sub(2);
        let legs = match self.trip {
            Trip::Staying => LEGS[0],
            _ => LEGS[(x / STEP_COLUMNS).rem_euclid(2) as usize],
        };
        garden::draw_clipped(renderer, x, y, &[HAT], ELF_RED)?;
        if self.is_visiting() {
            garden::draw_clipped(renderer, x, y + 1, &[GIFT], CHOCOLATE_BROWN)?;
        }
        garden::draw_clipped(renderer, x + FACE_COLUMN, y + 1, &[FACE], Color::White)?;
        garden::draw_clipped(renderer, x + FACE_COLUMN, y + 2, &[legs], ELF_GREEN)
    }
}

impl Default for Chocolutin {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::tree::Growth;

    fn walk(chocolutin: &mut Chocolutin, layout: &GardenLayout) {
        for _ in 0..=(layout.width as usize * 3) {
            chocolutin.update(layout);
        }
    }

    #[test]
    fn test_chocolutin_walks_to_the_house_and_waits_there() {
        let layout = GardenLayout::new(160, 40, Growth::Full);
        let mut chocolutin = Chocolutin::new();
        assert!(!chocolutin.is_visiting());

        chocolutin.visit(&layout);
        assert!(chocolutin.is_visiting());
        assert!(!chocolutin.has_arrived());
        chocolutin.update(&layout);
        assert!(!chocolutin.has_arrived());

        walk(&mut chocolutin, &layout);
        assert!(chocolutin.has_arrived());
        assert_eq!(chocolutin.x, Chocolutin::stop_x(&layout));
    }

    #[test]
    fn test_chocolutin_leaves_the_scene_once_told_to() {
        let layout = GardenLayout::new(160, 40, Growth::Full);
        let mut chocolutin = Chocolutin::new();
        chocolutin.leave();
        assert_eq!(chocolutin.trip, Trip::Waiting);

        chocolutin.visit(&layout);
        walk(&mut chocolutin, &layout);
        chocolutin.leave();
        assert!(!chocolutin.is_visiting());
        walk(&mut chocolutin, &layout);
        assert_eq!(chocolutin.trip, Trip::Waiting);
    }

    #[test]
    fn test_narrow_scene_lets_chocolutin_arrive_at_once() {
        let layout = GardenLayout::new(40, 20, Growth::Full);
        let mut chocolutin = Chocolutin::new();
        chocolutin.visit(&layout);
        chocolutin.update(&layout);
        assert!(chocolutin.has_arrived());
    }
}
