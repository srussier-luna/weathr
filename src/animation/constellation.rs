use crate::animation::garden::{self, GardenLayout};
use crate::holiday::Season;
use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

const SHOW_FRAMES: u16 = 180;
const SKY_ROW: u16 = 3;

struct Constellation {
    name: &'static str,
    season: Option<Season>,
    stars: &'static [(u16, u16)],
}

const CONSTELLATIONS: [Constellation; 6] = [
    Constellation {
        name: "la Grande Ourse",
        season: None,
        stars: &[(0, 2), (4, 1), (8, 1), (12, 2), (14, 4), (19, 4), (20, 2)],
    },
    Constellation {
        name: "Cassiopée",
        season: None,
        stars: &[(0, 0), (3, 2), (6, 1), (9, 2), (12, 0)],
    },
    Constellation {
        name: "Orion",
        season: Some(Season::Winter),
        stars: &[(0, 0), (8, 0), (3, 3), (4, 3), (5, 3), (0, 6), (8, 6)],
    },
    Constellation {
        name: "le Lion",
        season: Some(Season::Spring),
        stars: &[(0, 0), (2, 1), (1, 2), (3, 3), (8, 3), (13, 2), (11, 4)],
    },
    Constellation {
        name: "le Triangle d'été",
        season: Some(Season::Summer),
        stars: &[(0, 0), (10, 1), (4, 5)],
    },
    Constellation {
        name: "Pégase",
        season: Some(Season::Autumn),
        stars: &[(0, 0), (8, 0), (0, 4), (8, 4), (13, 6)],
    },
];

pub struct StarGazing {
    wait: u32,
    remaining: u16,
    index: usize,
}

impl StarGazing {
    pub fn new() -> Self {
        Self {
            wait: 900,
            remaining: 0,
            index: 0,
        }
    }

    fn is_clear_night(conditions: &WeatherConditions) -> bool {
        !conditions.is_day
            && !conditions.is_cloudy
            && !conditions.is_raining
            && !conditions.is_thunderstorm
            && !conditions.is_snowing
            && !conditions.is_foggy
    }

    pub fn update(
        &mut self,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
        season: Season,
        rng: &mut impl Rng,
    ) {
        if layout.plan.tree.is_none() || !Self::is_clear_night(conditions) {
            self.remaining = 0;
            return;
        }
        if self.remaining > 0 {
            self.remaining -= 1;
        } else if self.wait > 0 {
            self.wait -= 1;
        } else {
            let visible: Vec<usize> = CONSTELLATIONS
                .iter()
                .enumerate()
                .filter(|(_, constellation)| constellation.season.is_none_or(|only| only == season))
                .map(|(index, _)| index)
                .collect();
            self.index = visible[rng.random_range(0..visible.len())];
            self.remaining = SHOW_FRAMES;
            self.wait = rng.random_range(1800..5400);
        }
    }

    pub fn render(&self, renderer: &mut TerminalRenderer, layout: &GardenLayout) -> io::Result<()> {
        let Some(tree) = layout.plan.tree.filter(|_| self.remaining > 0) else {
            return Ok(());
        };

        let constellation = &CONSTELLATIONS[self.index];
        let anchor_x = layout.width * 11 / 20;
        for (dx, dy) in constellation.stars {
            renderer.render_char(anchor_x + dx, SKY_ROW + dy, '*', Color::White)?;
        }

        let owl_y = tree.top_y + (tree.canopy_height / 2).saturating_sub(1);
        let caption = format!("Hou ! Là-haut : {}.", constellation.name);
        garden::draw_solid(
            renderer,
            tree.center_x().saturating_sub(4),
            owl_y.saturating_sub(2),
            &[caption.as_str()],
            Color::White,
        )
    }
}

impl Default for StarGazing {
    fn default() -> Self {
        Self::new()
    }
}
