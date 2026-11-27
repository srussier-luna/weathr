use crate::animation::garden::{self, GardenLayout};
use crate::animation::trophies::Trophies;
use crate::render::TerminalRenderer;
use crate::scene::river::River;
use crossterm::style::Color;
use std::io;

const SPRITE: [&str; 3] = ["Y Y", "(o)", "/ \\"];
const HEIGHT: u16 = SPRITE.len() as u16;
const NOSE: (u16, u16) = (1, 1);
const DAY_FUR: Color = Color::Rgb {
    r: 150,
    g: 95,
    b: 50,
};
const NIGHT_FUR: Color = Color::Rgb {
    r: 90,
    g: 60,
    b: 35,
};

#[derive(Default)]
pub struct Reindeer {
    is_home: bool,
}

impl Reindeer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_home(&mut self, is_home: bool) {
        self.is_home = is_home;
    }

    pub fn spot(&self, layout: &GardenLayout, river: &River, trophies: &Trophies) -> Option<u16> {
        self.is_home
            .then(|| trophies.free_spot(HEIGHT, layout, river))
            .flatten()
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        river: &River,
        trophies: &Trophies,
        is_day: bool,
    ) -> io::Result<()> {
        let Some(x) = self.spot(layout, river, trophies) else {
            return Ok(());
        };
        let y = layout.horizon_y.saturating_sub(HEIGHT);
        let fur = if is_day { DAY_FUR } else { NIGHT_FUR };
        garden::draw(renderer, x, y, &SPRITE, fur)?;
        renderer.render_char(x + NOSE.0, y + NOSE.1, 'o', Color::Red)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::trophies::{TROPHIES, Trophy, WIDTH};
    use crate::scene::tree::Growth;

    fn spot(reindeer: &Reindeer, shown: Vec<Trophy>, layout: &GardenLayout) -> Option<u16> {
        let mut trophies = Trophies::new();
        trophies.set_shown(shown);
        reindeer.spot(
            layout,
            &River::new(layout.width, layout.horizon_y),
            &trophies,
        )
    }

    #[test]
    fn test_reindeer_is_as_wide_as_a_trophy_spot() {
        assert!(
            SPRITE
                .iter()
                .all(|line| line.chars().count() == WIDTH as usize)
        );
    }

    #[test]
    fn test_reindeer_is_drawn_once_home_only() {
        let layout = GardenLayout::new(120, 40, Growth::Full);
        let mut reindeer = Reindeer::new();
        assert_eq!(spot(&reindeer, Vec::new(), &layout), None);
        reindeer.set_home(true);
        assert!(spot(&reindeer, Vec::new(), &layout).is_some());
    }

    #[test]
    fn test_reindeer_never_covers_a_trophy() {
        let layout = GardenLayout::new(200, 40, Growth::Full);
        let mut reindeer = Reindeer::new();
        reindeer.set_home(true);
        let x = spot(&reindeer, TROPHIES.to_vec(), &layout).unwrap();
        let first = spot(&reindeer, Vec::new(), &layout).unwrap();
        assert!(x >= first + 2 * (WIDTH + 1));
    }

    #[test]
    fn test_reindeer_without_room_is_not_drawn() {
        let layout = GardenLayout::new(120, 40, Growth::Full);
        let mut reindeer = Reindeer::new();
        reindeer.set_home(true);
        assert_eq!(
            spot(
                &reindeer,
                vec![Trophy::GardenGnome, Trophy::Statue],
                &layout
            ),
            None
        );
    }
}
