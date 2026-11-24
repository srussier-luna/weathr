use crate::animation::dog::Dog;
use crate::animation::garden::{self, GardenLayout};
use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::collections::BTreeSet;
use std::io;

const GAP: u16 = 1;
const WIDTH: u16 = 3;
const GNOME: [&str; 3] = [" A ", "(\")", "/V\\"];
const STATUE: [&str; 4] = [" o ", "/|\\", "/ \\", "[=]"];
const HAT: Color = Color::Red;
const FACE: Color = Color::Rgb {
    r: 240,
    g: 200,
    b: 160,
};
const COAT: Color = Color::Blue;
const DAY_GOLD: Color = Color::Rgb {
    r: 255,
    g: 200,
    b: 40,
};
const NIGHT_GOLD: Color = Color::Rgb {
    r: 170,
    g: 130,
    b: 30,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Trophy {
    GardenGnome,
    Statue,
}

pub const TROPHIES: [Trophy; 2] = [Trophy::GardenGnome, Trophy::Statue];

impl Trophy {
    pub fn id(self) -> &'static str {
        match self {
            Self::GardenGnome => "garden_gnome",
            Self::Statue => "statue",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::GardenGnome => "Nain de jardin",
            Self::Statue => "Statue",
        }
    }

    pub fn badge(self) -> &'static str {
        match self {
            Self::GardenGnome => "streak_60",
            Self::Statue => "streak_100",
        }
    }

    pub fn places(self) -> [&'static str; 2] {
        match self {
            Self::GardenGnome => ["au jardin", "rangé"],
            Self::Statue => ["au jardin", "rangée"],
        }
    }

    pub fn won(badges: &BTreeSet<String>) -> impl Iterator<Item = Self> + '_ {
        TROPHIES
            .into_iter()
            .filter(|trophy| badges.contains(trophy.badge()))
    }

    pub fn shown(badges: &BTreeSet<String>, hidden: &BTreeSet<String>) -> Vec<Self> {
        Self::won(badges)
            .filter(|trophy| !hidden.contains(trophy.id()))
            .collect()
    }

    fn sprite(self) -> &'static [&'static str] {
        match self {
            Self::GardenGnome => &GNOME,
            Self::Statue => &STATUE,
        }
    }
}

#[derive(Default)]
pub struct Trophies {
    shown: Vec<Trophy>,
}

impl Trophies {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_shown(&mut self, shown: Vec<Trophy>) {
        self.shown = shown;
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        is_day: bool,
    ) -> io::Result<()> {
        for (trophy, x) in spots(&self.shown, layout) {
            let sprite = trophy.sprite();
            let y = layout.horizon_y.saturating_sub(sprite.len() as u16);
            match trophy {
                Trophy::GardenGnome => {
                    for (row, color) in [HAT, FACE, COAT].into_iter().enumerate() {
                        garden::draw_solid(renderer, x, y + row as u16, &sprite[row..=row], color)?;
                    }
                }
                Trophy::Statue => {
                    let gold = if is_day { DAY_GOLD } else { NIGHT_GOLD };
                    garden::draw_solid(renderer, x, y, &sprite[..3], gold)?;
                    garden::draw_solid(renderer, x, y + 3, &sprite[3..], Color::Grey)?;
                }
            }
        }
        Ok(())
    }
}

fn spots(shown: &[Trophy], layout: &GardenLayout) -> Vec<(Trophy, u16)> {
    let end = layout.fence_x().saturating_sub(GAP).min(layout.width);
    let mut x = Dog::seat_end(layout) + GAP;
    let mut spots = Vec::new();
    for &trophy in shown {
        if x + WIDTH > end {
            break;
        }
        spots.push((trophy, x));
        x += WIDTH + GAP;
    }
    spots
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::tree::Growth;

    fn badges(ids: &[&str]) -> BTreeSet<String> {
        ids.iter().map(|id| id.to_string()).collect()
    }

    #[test]
    fn test_trophies_are_won_with_their_badge_only() {
        assert!(Trophy::won(&badges(&["streak_30"])).next().is_none());
        assert_eq!(
            Trophy::won(&badges(&["streak_60"])).collect::<Vec<_>>(),
            [Trophy::GardenGnome]
        );
        assert_eq!(
            Trophy::won(&badges(&["streak_60", "streak_100"])).collect::<Vec<_>>(),
            TROPHIES
        );
    }

    #[test]
    fn test_put_away_trophy_is_not_shown() {
        let won = badges(&["streak_60", "streak_100"]);
        assert_eq!(Trophy::shown(&won, &BTreeSet::new()), TROPHIES);
        assert_eq!(
            Trophy::shown(&won, &badges(&["garden_gnome"])),
            [Trophy::Statue]
        );
        assert_eq!(
            Trophy::shown(&won, &badges(&["garden_gnome", "statue"])),
            []
        );
    }

    #[test]
    fn test_sprites_are_as_wide_as_a_spot() {
        for trophy in TROPHIES {
            assert!(
                trophy
                    .sprite()
                    .iter()
                    .all(|line| line.chars().count() == WIDTH as usize)
            );
        }
    }

    #[test]
    fn test_trophies_stand_side_by_side_right_of_biscuit_seat_before_the_fence() {
        for width in [80, 120, 200, 300] {
            let layout = GardenLayout::new(width, 40, Growth::Full);
            let spots = spots(&TROPHIES, &layout);
            assert_eq!(spots.len(), 2, "{width}");
            let (gnome, gnome_x) = spots[0];
            let (statue, statue_x) = spots[1];
            assert_eq!((gnome, statue), (Trophy::GardenGnome, Trophy::Statue));
            assert!(gnome_x > Dog::seat_end(&layout));
            assert!(gnome_x + WIDTH < statue_x);
            assert!(statue_x + WIDTH < layout.fence_x());
        }
    }

    #[test]
    fn test_lone_trophy_takes_the_first_spot() {
        let layout = GardenLayout::new(120, 40, Growth::Full);
        let first = spots(&TROPHIES, &layout)[0].1;
        assert_eq!(spots(&[Trophy::Statue], &layout), [(Trophy::Statue, first)]);
    }

    #[test]
    fn test_trophy_without_room_is_not_drawn() {
        let layout = GardenLayout::new(120, 40, Growth::Full);
        let first = spots(&TROPHIES, &layout)[0].1;
        let narrow = GardenLayout {
            width: first + 2,
            ..layout
        };
        assert!(spots(&TROPHIES, &narrow).is_empty());
    }
}
