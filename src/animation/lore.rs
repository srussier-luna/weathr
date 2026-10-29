use crate::animation::cloud_bank::CloudBank;
use crate::animation::garden::{self, GardenLayout};
use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::prelude::*;
use std::collections::BTreeMap;
use std::io;

const BENCH: [&str; 2] = ["_____", "|   |"];
const WELL: [&str; 4] = [" _/^^^\\_", "/_______\\", " |  o  |", "(_______)"];
const ZOOM_HINT: &str = "‹ dézoom";
const ZOOM_HINT_LIFT: u16 = 4;
const TREEHOUSE: [&str; 2] = [" /\\ ", "|[]|"];
const CHEST: [&str; 2] = [" ___ ", "[_$_]"];
const BALLOON: [&str; 4] = [" ,-. ", "(   )", " \\ / ", "  #  "];
const BALLOON_SPEED: f32 = 0.06;
const LANTERN_WINDOW: (u16, u16) = (8, 8);
const FOOTPRINT_STEP: usize = 3;
const LIGHTS: [Color; 4] = [Color::Red, Color::Yellow, Color::Cyan, Color::Magenta];
const WOOD: Color = Color::Rgb {
    r: 150,
    g: 100,
    b: 60,
};
const MUD: Color = Color::Rgb {
    r: 120,
    g: 90,
    b: 60,
};
const ENVELOPE: Color = Color::Rgb {
    r: 220,
    g: 60,
    b: 60,
};

#[derive(Clone, Copy, Default)]
pub struct Choices {
    pub bow_blue: bool,
    pub well_green: bool,
    pub postman_bell: bool,
    pub owl_silent: bool,
    pub avoids_puddles: bool,
    pub treehouse_blue: bool,
    pub nuts: bool,
    pub frog_croak: bool,
    pub lampions: bool,
    pub white_kittens: bool,
    pub three_ducklings: bool,
}

impl Choices {
    pub fn from_answers(answers: &BTreeMap<String, u8>) -> Self {
        let chose = |id: &str, option: u8| answers.get(id) == Some(&option);
        Self {
            bow_blue: chose("bow", 2),
            well_green: chose("well", 2),
            postman_bell: chose("bell", 1),
            owl_silent: chose("owl", 2),
            avoids_puddles: chose("puddles", 2),
            treehouse_blue: chose("treehouse", 2),
            nuts: chose("nuts", 1),
            frog_croak: chose("song", 1),
            lampions: chose("party", 2),
            white_kittens: chose("kittens", 2),
            three_ducklings: chose("ducklings", 1),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Unlocks {
    pub bench: bool,
    pub cat_bow: bool,
    pub footprints: bool,
    pub well: bool,
    pub bone: bool,
    pub key: bool,
    pub treehouse: bool,
    pub owl_friend: bool,
    pub blue_lantern: bool,
    pub garland: bool,
    pub kittens: bool,
    pub x_mark: bool,
    pub balloon: bool,
    pub ducklings: bool,
    pub weather_vane: bool,
    pub bunting: bool,
    pub chest: bool,
    pub wet_footprints: bool,
    pub silhouette: bool,
    pub whispering_well: bool,
    pub staring: bool,
    pub found_objects: bool,
    pub bottles: bool,
    pub choices: Choices,
}

impl Unlocks {
    pub fn after_reading(
        letters_read: usize,
        strange_letters_read: usize,
        archives_read: usize,
        answers: &BTreeMap<String, u8>,
    ) -> Self {
        let read = |letter: usize| letters_read >= letter;
        let strange = |letter: usize| strange_letters_read >= letter;
        Self {
            bench: read(4),
            cat_bow: read(6),
            footprints: read(8),
            well: read(13),
            bone: read(15),
            key: read(17),
            treehouse: read(22),
            owl_friend: read(24),
            blue_lantern: read(26),
            garland: read(31),
            kittens: read(33),
            x_mark: read(35),
            balloon: read(40),
            ducklings: read(42),
            weather_vane: read(49),
            bunting: read(58),
            chest: read(59),
            wet_footprints: strange(3),
            silhouette: strange(5),
            whispering_well: strange(8),
            staring: strange(11),
            found_objects: read(45),
            bottles: archives_read >= 3,
            choices: Choices::from_answers(answers),
        }
    }
}

pub struct Lore {
    unlocks: Unlocks,
    balloon_x: Option<f32>,
    balloon_wait: u32,
    wind: f32,
    frame: u32,
}

impl Lore {
    pub fn new() -> Self {
        Self {
            unlocks: Unlocks::default(),
            balloon_x: None,
            balloon_wait: 600,
            wind: 0.0,
            frame: 0,
        }
    }

    pub fn unlocks(&self) -> Unlocks {
        self.unlocks
    }

    pub fn set_unlocks(&mut self, unlocks: Unlocks) {
        self.unlocks = unlocks;
    }

    pub fn set_wind(&mut self, direction_deg: f32) {
        self.wind = -direction_deg.to_radians().sin();
    }

    fn has_balloon_weather(conditions: &WeatherConditions) -> bool {
        conditions.is_day
            && !conditions.is_raining
            && !conditions.is_thunderstorm
            && !conditions.is_snowing
    }

    pub fn update(
        &mut self,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
        rng: &mut impl Rng,
    ) {
        self.frame = self.frame.wrapping_add(1);
        if !self.unlocks.balloon || !Self::has_balloon_weather(conditions) {
            self.balloon_x = None;
            return;
        }

        match self.balloon_x {
            Some(x) if x > layout.width as f32 => {
                self.balloon_x = None;
                self.balloon_wait = rng.random_range(1800..3600);
            }
            Some(x) => self.balloon_x = Some(x + BALLOON_SPEED),
            None if self.balloon_wait > 0 => self.balloon_wait -= 1,
            None => self.balloon_x = Some(-6.0),
        }
    }

    pub fn render_scenery(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
    ) -> io::Result<()> {
        let unlocks = &self.unlocks;
        let ground_y = layout.horizon_y;
        let house = &layout.house;

        if unlocks.bench {
            let x = (layout.house_x + house.door_x()).saturating_sub(15);
            garden::draw_solid(renderer, x, ground_y - 2, &BENCH[..1], WOOD)?;
            garden::draw(renderer, x, ground_y - 1, &BENCH[1..], WOOD)?;
        }

        if unlocks.well {
            match layout.well_x() {
                Some(x) => {
                    let roof = if unlocks.choices.well_green {
                        Color::DarkGreen
                    } else {
                        Color::DarkRed
                    };
                    garden::draw_solid(renderer, x, ground_y - 4, &WELL[..2], roof)?;
                    garden::draw_solid(renderer, x, ground_y - 2, &WELL[2..3], WOOD)?;
                    garden::draw_solid(renderer, x, ground_y - 1, &WELL[3..], Color::Grey)?;
                }
                None => {
                    let y = layout.plan.tree.map_or(ground_y / 2, |tree| {
                        tree.top_y.saturating_sub(ZOOM_HINT_LIFT)
                    });
                    garden::draw_solid(renderer, 0, y, &[ZOOM_HINT], Color::DarkGrey)?;
                }
            }
        }

        if unlocks.footprints {
            let start = layout
                .plan
                .tree
                .map_or(1, |tree| tree.trunk_x() + tree.trunk_width() + 4);
            let end = layout.house_x + house.door_x();
            for x in (start..end).step_by(FOOTPRINT_STEP) {
                renderer.render_char(x, ground_y, ':', MUD)?;
            }
        }

        if unlocks.key
            && let Some(mailbox_x) = layout.plan.mailbox_x
        {
            garden::draw(renderer, mailbox_x + 2, ground_y, &["o-m"], Color::Yellow)?;
        }

        if unlocks.choices.nuts
            && let Some(mailbox_x) = layout.plan.mailbox_x
        {
            garden::draw(renderer, mailbox_x + 5, ground_y, &["oo"], WOOD)?;
        }

        if let Some(tree) = layout.plan.tree {
            let spot_x = tree.trunk_x() + tree.trunk_width() + 2;
            if unlocks.chest {
                garden::draw_solid(renderer, spot_x - 1, ground_y - 1, &CHEST, WOOD)?;
                renderer.render_char(spot_x + 1, ground_y, '$', Color::Yellow)?;
            } else if unlocks.x_mark {
                renderer.render_char(spot_x, ground_y, 'X', Color::Red)?;
            }

            if unlocks.treehouse && tree.canopy_height >= 6 {
                let x = tree.center_x() + 3;
                let y = tree.top_y + tree.canopy_height / 4;
                let paint = if unlocks.choices.treehouse_blue {
                    Color::Rgb {
                        r: 60,
                        g: 100,
                        b: 200,
                    }
                } else {
                    Color::Rgb {
                        r: 190,
                        g: 60,
                        b: 50,
                    }
                };
                garden::draw_solid(renderer, x, y, &TREEHOUSE, paint)?;
                let ladder_x = tree.trunk_x() + tree.trunk_width();
                for ladder_y in tree.canopy_bottom_y()..ground_y {
                    renderer.render_char(ladder_x, ladder_y, 'H', WOOD)?;
                }
            }
        }

        if unlocks.bone {
            let x = layout.house_x + house.porch_x() + 1;
            garden::draw(renderer, x, ground_y - 1, &["o==o"], Color::White)?;
        }

        if unlocks.weather_vane {
            let (peak_x, peak_y) = house.peak();
            let x = layout.house_x + peak_x;
            let y = layout.house_y + peak_y;
            let arrow = if self.wind >= 0.0 { "-->" } else { "<--" };
            garden::draw(renderer, x - 1, y.saturating_sub(2), &[arrow], Color::Grey)?;
            renderer.render_char(x, y.saturating_sub(1), '|', Color::Grey)?;
        }

        let rail_y = layout.house_y + house.porch_rail_row();
        let porch_start = layout.house_x + house.porch_x();
        let porch_end = porch_start + house.porch_width();

        if unlocks.bunting {
            let decoration = if unlocks.choices.lampions { 'o' } else { 'v' };
            for (i, x) in (porch_start..porch_end).step_by(2).enumerate() {
                renderer.render_char(x, rail_y - 2, decoration, LIGHTS[i % LIGHTS.len()])?;
            }
        }

        if !conditions.is_day {
            if unlocks.garland {
                let shift = (self.frame / 15) as usize;
                for (i, x) in (porch_start..porch_end).step_by(3).enumerate() {
                    let color = LIGHTS[(i + shift) % LIGHTS.len()];
                    renderer.render_char(x, rail_y - 1, 'o', color)?;
                }
            }

            if unlocks.blue_lantern {
                let color = if (self.frame / 20) % 2 == 0 {
                    Color::Blue
                } else {
                    Color::Cyan
                };
                let x = layout.house_x + LANTERN_WINDOW.0;
                let y = layout.house_y + LANTERN_WINDOW.1;
                garden::draw(renderer, x, y, &["[]"], color)?;
            }
        }
        Ok(())
    }

    pub fn render_balloon(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        let Some(x) = self.balloon_x else {
            return Ok(());
        };
        let x = x.round() as i32;
        let y = CloudBank::bottom();
        garden::draw_clipped(renderer, x, y, &BALLOON[..3], ENVELOPE)?;
        garden::draw_clipped(renderer, x, y + 3, &BALLOON[3..], WOOD)
    }
}

impl Default for Lore {
    fn default() -> Self {
        Self::new()
    }
}
