use crate::animation::garden::{self, GardenLayout};
use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

const SPEED: f32 = 0.4;
const SPRITE_WIDTH: f32 = 9.0;
const DOOR_CENTER: u16 = 2;
const FACE_CENTER: u16 = 5;
const STEP_COLUMNS: i32 = 2;
const HAT: [&str; 2] = ["     *", "   /███\\"];
const FACE: &str = "  <(°◡°)>";
const ARMS: &str = "  /     \\";
const TRAY_COLUMN: i32 = 3;
const FULL_TRAY: &str = "[###]";
const EMPTY_TRAY: &str = "[   ]";
const FEET: [&str; 2] = ["   d   b", "    d b"];
const SPRITE_ROWS: u16 = 5;
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

    pub fn visit(&mut self) {
        self.x = -SPRITE_WIDTH;
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
        f32::from(
            (layout.house_x + layout.house.door_x() + DOOR_CENTER).saturating_sub(FACE_CENTER),
        )
    }

    pub fn update(&mut self, layout: &GardenLayout) {
        let stop_x = Self::stop_x(layout);
        self.trip = match self.trip {
            Trip::Arriving => {
                self.x += SPEED;
                if self.x >= stop_x {
                    self.x = stop_x;
                    Trip::Staying
                } else {
                    Trip::Arriving
                }
            }
            Trip::Leaving => {
                self.x -= SPEED;
                if self.x < -SPRITE_WIDTH {
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
        let y = layout.horizon_y.saturating_sub(SPRITE_ROWS - 1);
        let feet = match self.trip {
            Trip::Staying => FEET[0],
            _ => FEET[(x / STEP_COLUMNS).rem_euclid(2) as usize],
        };
        let tray = if self.is_visiting() {
            FULL_TRAY
        } else {
            EMPTY_TRAY
        };
        garden::draw_clipped(renderer, x, y, &[HAT[0]], Color::White)?;
        garden::draw_clipped(renderer, x, y + 1, &[HAT[1]], ELF_RED)?;
        garden::draw_clipped(renderer, x, y + 2, &[FACE], Color::White)?;
        garden::draw_clipped(renderer, x, y + 3, &[ARMS], ELF_GREEN)?;
        garden::draw_clipped(renderer, x + TRAY_COLUMN, y + 3, &[tray], CHOCOLATE_BROWN)?;
        garden::draw_clipped(renderer, x, y + 4, &[feet], ELF_GREEN)
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

        chocolutin.visit();
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

        chocolutin.visit();
        walk(&mut chocolutin, &layout);
        chocolutin.leave();
        assert!(!chocolutin.is_visiting());
        walk(&mut chocolutin, &layout);
        assert_eq!(chocolutin.trip, Trip::Waiting);
    }

    #[test]
    fn test_chocolutin_stops_in_front_of_the_door_clear_of_the_postman() {
        for (width, height) in [(120, 40), (200, 60)] {
            let layout = GardenLayout::new(width, height, Growth::Full);
            let stop = Chocolutin::stop_x(&layout) as u16;
            assert_eq!(
                stop + FACE_CENTER,
                layout.house_x + layout.house.door_x() + DOOR_CENTER
            );
            let mailbox_x = layout.plan.mailbox_x.unwrap();
            assert!(stop > mailbox_x + SPRITE_WIDTH as u16);
        }
    }

    #[test]
    fn test_sprite_fits_its_width() {
        let rows = [HAT[0], HAT[1], FACE, ARMS, FEET[0], FEET[1]];
        assert!(
            rows.iter()
                .all(|row| row.chars().count() <= SPRITE_WIDTH as usize)
        );
        assert!(TRAY_COLUMN as usize + FULL_TRAY.len() <= ARMS.len());
    }
}
