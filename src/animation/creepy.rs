use crate::animation::garden::{self, GardenLayout};
use crate::animation::lore::Unlocks;
use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

const SHOW_FRAMES: u16 = 90;
const STARE_FRAMES: u16 = 150;
const WHISPERS: [&str; 6] = ["...", "ici", "viens", "treize", "chut", "derrière toi"];
const SHADOW: Color = Color::Rgb {
    r: 40,
    g: 40,
    b: 50,
};
const WET: Color = Color::Rgb {
    r: 150,
    g: 160,
    b: 200,
};

struct Apparition {
    wait: u32,
    remaining: u16,
}

impl Apparition {
    fn new(wait: u32) -> Self {
        Self { wait, remaining: 0 }
    }

    fn update(&mut self, is_allowed: bool, duration: u16, next_wait: u32) -> bool {
        if !is_allowed {
            self.remaining = 0;
            return false;
        }
        if self.remaining > 0 {
            self.remaining -= 1;
        } else if self.wait > 0 {
            self.wait -= 1;
        } else {
            self.remaining = duration;
            self.wait = next_wait;
            return true;
        }
        false
    }

    fn is_visible(&self) -> bool {
        self.remaining > 0
    }
}

pub struct Creepy {
    silhouette: Apparition,
    whisper: Apparition,
    stare: Apparition,
    whisper_index: usize,
}

impl Creepy {
    pub fn new() -> Self {
        Self {
            silhouette: Apparition::new(1200),
            whisper: Apparition::new(600),
            stare: Apparition::new(1800),
            whisper_index: 0,
        }
    }

    pub fn is_staring(&self) -> bool {
        self.stare.is_visible()
    }

    pub fn update(
        &mut self,
        conditions: &WeatherConditions,
        unlocks: &Unlocks,
        rng: &mut impl Rng,
    ) {
        let is_night = !conditions.is_day;
        let is_dry =
            !conditions.is_raining && !conditions.is_thunderstorm && !conditions.is_snowing;

        self.silhouette.update(
            unlocks.silhouette && is_night && is_dry,
            SHOW_FRAMES,
            rng.random_range(1200..3600),
        );
        if self.whisper.update(
            unlocks.whispering_well && unlocks.well && is_night,
            SHOW_FRAMES,
            rng.random_range(600..1800),
        ) {
            self.whisper_index = rng.random_range(0..WHISPERS.len());
        }
        self.stare
            .update(unlocks.staring, STARE_FRAMES, rng.random_range(1800..5400));
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
        unlocks: &Unlocks,
    ) -> io::Result<()> {
        let house = &layout.house;
        let well_x = layout.well_x().filter(|_| unlocks.well);

        if unlocks.wet_footprints && !conditions.is_day {
            let door_x = layout.house_x + house.door_x() + 2;
            let end_x = well_x.unwrap_or(0);
            for x in (end_x..door_x).rev().step_by(3) {
                renderer.render_char(x, layout.horizon_y, '.', WET)?;
            }
        }

        if self.silhouette.is_visible() {
            let (window_x, window_y) = house.big_window();
            let x = layout.house_x + window_x;
            let y = layout.house_y + window_y + 1;
            garden::draw_solid(renderer, x, y, &["/o\\"], SHADOW)?;
        }

        if self.whisper.is_visible()
            && let Some(well_x) = well_x
        {
            let y = layout.horizon_y.saturating_sub(5);
            garden::draw(
                renderer,
                well_x,
                y,
                &[WHISPERS[self.whisper_index]],
                Color::DarkGrey,
            )?;
        }
        Ok(())
    }
}

impl Default for Creepy {
    fn default() -> Self {
        Self::new()
    }
}
