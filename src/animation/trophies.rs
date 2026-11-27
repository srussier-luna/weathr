use crate::animation::dog::Dog;
use crate::animation::garden::{self, GardenLayout};
use crate::render::TerminalRenderer;
use crate::scene::river::River;
use crossterm::style::Color;
use std::collections::BTreeSet;
use std::io;

const GAP: u16 = 1;
pub const WIDTH: u16 = 3;
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
    Sundial,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Spot {
    BesideDoghouse,
    InFrontOfFence,
}

pub const TROPHIES: [Trophy; 3] = [Trophy::GardenGnome, Trophy::Statue, Trophy::Sundial];

impl Trophy {
    pub fn id(self) -> &'static str {
        match self {
            Self::GardenGnome => "garden_gnome",
            Self::Statue => "statue",
            Self::Sundial => "sundial",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::GardenGnome => "Nain de jardin",
            Self::Statue => "Statue",
            Self::Sundial => "Cadran solaire",
        }
    }

    pub fn badge(self) -> &'static str {
        match self {
            Self::GardenGnome => "streak_60",
            Self::Statue => "streak_100",
            Self::Sundial => "streak_365",
        }
    }

    pub fn places(self) -> [&'static str; 2] {
        match self {
            Self::GardenGnome | Self::Sundial => ["au jardin", "rangé"],
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

    fn spot(self) -> Spot {
        match self {
            Self::GardenGnome | Self::Statue => Spot::BesideDoghouse,
            Self::Sundial => Spot::InFrontOfFence,
        }
    }

    fn sprite(self, is_day: bool) -> Vec<(&'static str, Color)> {
        match self {
            Self::GardenGnome => vec![(" A ", HAT), ("(\")", FACE), ("/V\\", COAT)],
            Self::Statue => {
                let gold = if is_day { DAY_GOLD } else { NIGHT_GOLD };
                vec![
                    (" o ", gold),
                    ("/|\\", gold),
                    ("/ \\", gold),
                    ("[=]", Color::Grey),
                ]
            }
            Self::Sundial => vec![
                (" |\\", Color::DarkYellow),
                ("===", Color::White),
                ("[_]", Color::Grey),
            ],
        }
    }

    fn height(self) -> u16 {
        self.sprite(true).len() as u16
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

    pub fn free_spot(&self, height: u16, layout: &GardenLayout, river: &River) -> Option<u16> {
        free_spot(&self.shown, height, layout, river)
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        river: &River,
        is_day: bool,
    ) -> io::Result<()> {
        for (trophy, x) in spots(&self.shown, layout, river) {
            let y = layout.horizon_y.saturating_sub(trophy.height());
            for (row, (line, color)) in trophy.sprite(is_day).into_iter().enumerate() {
                garden::draw_solid(renderer, x, y + row as u16, &[line], color)?;
            }
        }
        Ok(())
    }
}

fn beside_doghouse_end(layout: &GardenLayout) -> u16 {
    layout.fence_x().saturating_sub(GAP).min(layout.width)
}

fn spots(shown: &[Trophy], layout: &GardenLayout, river: &River) -> Vec<(Trophy, u16)> {
    let end = beside_doghouse_end(layout);
    let mut x = Dog::seat_end(layout) + GAP;
    let mut spots = Vec::new();
    for &trophy in shown {
        let spot = match trophy.spot() {
            Spot::BesideDoghouse => {
                let spot = x;
                x += WIDTH + GAP;
                (spot + WIDTH <= end).then_some(spot)
            }
            Spot::InFrontOfFence => {
                let spot = layout.fence_x() + GAP;
                (spot + WIDTH <= layout.width).then_some(spot)
            }
        };
        if let Some(spot) =
            spot.filter(|&spot| clear_of_river(trophy.height(), spot, layout, river))
        {
            spots.push((trophy, spot));
        }
    }
    spots
}

fn free_spot(shown: &[Trophy], height: u16, layout: &GardenLayout, river: &River) -> Option<u16> {
    let taken = shown
        .iter()
        .filter(|trophy| trophy.spot() == Spot::BesideDoghouse)
        .count() as u16;
    let x = Dog::seat_end(layout) + GAP + taken * (WIDTH + GAP);
    (x + WIDTH <= beside_doghouse_end(layout) && clear_of_river(height, x, layout, river))
        .then_some(x)
}

fn clear_of_river(height: u16, x: u16, layout: &GardenLayout, river: &River) -> bool {
    let top = layout.horizon_y.saturating_sub(height);
    (0..river.stretch_count())
        .filter_map(|index| river.stretch(index))
        .filter(|stretch| (top..layout.horizon_y).contains(&stretch.y))
        .all(|stretch| stretch.x + stretch.width <= x || x + WIDTH <= stretch.x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::tree::Growth;

    fn placed(shown: &[Trophy], layout: &GardenLayout) -> Vec<(Trophy, u16)> {
        spots(shown, layout, &River::new(layout.width, layout.horizon_y))
    }

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
            [Trophy::GardenGnome, Trophy::Statue]
        );
        assert_eq!(
            Trophy::won(&badges(&["streak_365"])).collect::<Vec<_>>(),
            [Trophy::Sundial]
        );
        assert_eq!(
            Trophy::won(&badges(&["streak_60", "streak_100", "streak_365"])).collect::<Vec<_>>(),
            TROPHIES
        );
    }

    #[test]
    fn test_put_away_trophy_is_not_shown() {
        let won = badges(&["streak_60", "streak_100", "streak_365"]);
        assert_eq!(Trophy::shown(&won, &BTreeSet::new()), TROPHIES);
        assert_eq!(
            Trophy::shown(&won, &badges(&["garden_gnome"])),
            [Trophy::Statue, Trophy::Sundial]
        );
        assert_eq!(
            Trophy::shown(&won, &badges(&["garden_gnome", "statue"])),
            [Trophy::Sundial]
        );
        assert_eq!(
            Trophy::shown(&won, &badges(&["garden_gnome", "statue", "sundial"])),
            []
        );
    }

    #[test]
    fn test_sprites_are_as_wide_as_a_spot() {
        for trophy in TROPHIES {
            assert!([true, false].into_iter().all(|is_day| {
                trophy
                    .sprite(is_day)
                    .iter()
                    .all(|(line, _)| line.chars().count() == WIDTH as usize)
            }));
        }
    }

    #[test]
    fn test_trophies_stand_side_by_side_right_of_biscuit_seat_before_the_fence() {
        for width in [100, 120, 200, 300] {
            let layout = GardenLayout::new(width, 40, Growth::Full);
            let spots = placed(&TROPHIES, &layout);
            assert!(spots.len() >= 2, "{width}");
            let (gnome, gnome_x) = spots[0];
            let (statue, statue_x) = spots[1];
            assert_eq!((gnome, statue), (Trophy::GardenGnome, Trophy::Statue));
            assert!(gnome_x > Dog::seat_end(&layout));
            assert!(gnome_x + WIDTH < statue_x);
            assert!(statue_x + WIDTH < layout.fence_x());
        }
    }

    #[test]
    fn test_sundial_stands_in_front_of_the_fence_with_both_houses() {
        let mut house_widths = BTreeSet::new();
        for width in 80..400 {
            for height in [40, 60] {
                let layout = GardenLayout::new(width, height, Growth::Full);
                let spots = placed(&TROPHIES, &layout);
                let [_, (_, statue_x), (sundial, sundial_x)] = spots[..] else {
                    continue;
                };
                assert_eq!(sundial, Trophy::Sundial);
                assert_eq!(sundial_x, layout.fence_x() + GAP);
                assert!(statue_x + WIDTH < sundial_x);
                assert!(sundial_x + WIDTH <= layout.width);
                house_widths.insert(layout.house.width());
            }
        }
        assert_eq!(house_widths, BTreeSet::from([64, 86]));
    }

    #[test]
    fn test_sundial_without_room_in_front_of_the_fence_is_not_drawn() {
        let layout = GardenLayout::new(120, 40, Growth::Full);
        let river = River::new(layout.width, layout.horizon_y);
        let narrow = GardenLayout {
            width: layout.fence_x() + GAP + WIDTH - 1,
            ..layout
        };
        assert_eq!(
            spots(&TROPHIES, &narrow, &river)
                .iter()
                .map(|(trophy, _)| *trophy)
                .collect::<Vec<_>>(),
            [Trophy::GardenGnome, Trophy::Statue]
        );
    }

    #[test]
    fn test_lone_trophy_takes_the_first_spot() {
        let layout = GardenLayout::new(120, 40, Growth::Full);
        let first = placed(&TROPHIES, &layout)[0].1;
        assert_eq!(
            placed(&[Trophy::Statue], &layout),
            [(Trophy::Statue, first)]
        );
    }

    #[test]
    fn test_trophy_without_room_is_not_drawn() {
        let layout = GardenLayout::new(120, 40, Growth::Full);
        let first = placed(&TROPHIES, &layout)[0].1;
        let narrow = GardenLayout {
            width: first + 2,
            ..layout
        };
        assert!(placed(&TROPHIES, &narrow).is_empty());
    }

    fn free(shown: &[Trophy], layout: &GardenLayout) -> Option<u16> {
        free_spot(
            shown,
            3,
            layout,
            &River::new(layout.width, layout.horizon_y),
        )
    }

    #[test]
    fn test_free_spot_without_trophies_is_the_first_spot() {
        let layout = GardenLayout::new(120, 40, Growth::Full);
        assert_eq!(free(&[], &layout), Some(placed(&TROPHIES, &layout)[0].1));
    }

    #[test]
    fn test_free_spot_stands_after_the_trophies_beside_the_doghouse() {
        let layout = GardenLayout::new(200, 40, Growth::Full);
        let spots = placed(&TROPHIES, &layout);
        let x = free(&TROPHIES, &layout).unwrap();
        assert!(x > Dog::seat_end(&layout));
        assert!(
            spots
                .iter()
                .all(|&(_, spot)| spot + WIDTH < x || x + WIDTH < spot)
        );
        assert!(x + WIDTH < layout.fence_x());
    }

    #[test]
    fn test_free_spot_skips_the_sundial_in_front_of_the_fence() {
        let layout = GardenLayout::new(120, 40, Growth::Full);
        assert_eq!(free(&[Trophy::Sundial], &layout), free(&[], &layout));
    }

    #[test]
    fn test_no_free_spot_without_room() {
        let layout = GardenLayout::new(120, 40, Growth::Full);
        assert_eq!(free(&[Trophy::GardenGnome, Trophy::Statue], &layout), None);
        let narrow = GardenLayout {
            width: Dog::seat_end(&layout) + 2,
            ..layout
        };
        assert_eq!(free(&[], &narrow), None);
    }

    #[test]
    fn test_trophies_never_cover_the_river_above_the_horizon() {
        let layout = GardenLayout::new(80, 40, Growth::Full);
        let river = River::new(layout.width, layout.horizon_y);
        assert_eq!(
            (0..river.stretch_count())
                .filter_map(|index| river.stretch(index))
                .filter(|stretch| stretch.y + 4 >= layout.horizon_y && stretch.y < layout.horizon_y)
                .map(|stretch| (stretch.x, stretch.width))
                .collect::<Vec<_>>(),
            [(70, 1), (71, 1), (71, 2), (70, 2)]
        );
        assert_eq!(
            spots(&TROPHIES, &layout, &river),
            [(Trophy::GardenGnome, 66), (Trophy::Sundial, 75)]
        );
    }
}
