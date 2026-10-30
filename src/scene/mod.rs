pub mod decorations;
pub mod ground;
pub mod house;
pub mod mountains;
pub mod river;
pub mod tree;

use crate::holiday::Holiday;
use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use std::io;

pub struct WorldScene {
    house: house::House,
    ground: ground::Ground,
    decorations: decorations::Decorations,
    tree_growth: tree::Growth,
    width: u16,
    height: u16,
}

impl WorldScene {
    pub const GROUND_HEIGHT: u16 = 7;

    pub fn new(width: u16, height: u16) -> Self {
        let house = house::House::for_terminal(width, height);
        let ground = ground::Ground;
        let decorations = decorations::Decorations::new();

        Self {
            house,
            ground,
            decorations,
            tree_growth: tree::Growth::default(),
            width,
            height,
        }
    }

    pub fn update_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.house = house::House::for_terminal(width, height);
    }

    pub fn set_tree_growth(&mut self, growth: tree::Growth) {
        self.tree_growth = growth;
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        conditions: &WeatherConditions,
        holiday: &Holiday,
    ) -> io::Result<()> {
        let horizon_y = self.height.saturating_sub(Self::GROUND_HEIGHT);

        // House position
        let house_width = self.house.width();
        let house_height = self.house.height();
        let house_x = self.house.x_on(self.width, horizon_y);
        let house_y = horizon_y.saturating_sub(house_height);

        // Door/Path alignment

        mountains::Mountains.render(
            renderer,
            self.width,
            horizon_y,
            conditions.is_day,
            conditions.is_snowing || holiday.is_winter,
        )?;

        // Render Ground
        self.ground.render(
            renderer,
            self.width,
            Self::GROUND_HEIGHT,
            horizon_y,
            conditions.is_day,
        )?;

        river::River::new(self.width, horizon_y).render(
            renderer,
            conditions.is_day,
            holiday.is_winter,
        )?;

        // Render House
        self.house
            .render(renderer, house_x, house_y, conditions.is_day)?;

        // Render Decorations
        self.decorations.render(
            renderer,
            &crate::scene::decorations::DecorationRenderConfig {
                horizon_y,
                house_x,
                house_width,
                width: self.width,
                is_day: conditions.is_day,
                is_snowing: conditions.is_snowing,
                holiday: *holiday,
                tree_growth: self.tree_growth,
            },
        )?;

        Ok(())
    }
}
