use crate::animation::duck::puddle_x;
use crate::animation::garden::{self, GardenLayout};
use crate::animation::{kid, postman};
use crate::render::TerminalRenderer;
use crate::scene::decorations::{PINE_OFFSET, PINE_WIDTH};
use crate::scene::river::River;
use crossterm::style::Color;
use std::io;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Neighbour {
    Rose,
    Paulo,
    Odile,
    Leo,
    Gaston,
}

impl Neighbour {
    pub const ALL: [Self; 5] = [
        Self::Rose,
        Self::Paulo,
        Self::Odile,
        Self::Leo,
        Self::Gaston,
    ];
}

const ROWS: u16 = 4;
const WIDTH: u16 = 29;
const RIVER_GAP: u16 = 2;
const TABLE_X: u16 = 12;
const SEATS: [(Neighbour, u16); 5] = [
    (Neighbour::Rose, 0),
    (Neighbour::Paulo, 4),
    (Neighbour::Odile, 8),
    (Neighbour::Leo, 22),
    (Neighbour::Gaston, 26),
];
const BENCHES: [(u16, &str); 2] = [(0, "==========="), (22, "=======")];
const FOOD: &str = "U (@) U";
const CLOTH: &str = "[=======]";
const LEGS: &str = " |     | ";
const BODY: &str = "/|\\";
const WOOD: Color = Color::Rgb {
    r: 150,
    g: 100,
    b: 50,
};
const CAKE: Color = Color::Rgb {
    r: 255,
    g: 170,
    b: 200,
};
const ROSE_PINK: Color = Color::Rgb {
    r: 240,
    g: 110,
    b: 160,
};
const NAVY: Color = Color::Rgb {
    r: 40,
    g: 60,
    b: 140,
};

struct Look {
    hat: Option<(char, Color)>,
    body: [Color; 3],
}

fn look(neighbour: Neighbour) -> Look {
    match neighbour {
        Neighbour::Rose => Look {
            hat: Some(('n', Color::White)),
            body: [ROSE_PINK; 3],
        },
        Neighbour::Paulo => Look {
            hat: Some(('=', NAVY)),
            body: [Color::Blue, Color::White, Color::Blue],
        },
        Neighbour::Odile => Look {
            hat: None,
            body: [Color::Blue, Color::White, Color::Red],
        },
        Neighbour::Leo => Look {
            hat: None,
            body: [kid::SHIRT; 3],
        },
        Neighbour::Gaston => Look {
            hat: Some(('_', postman::POSTAL_YELLOW)),
            body: [postman::POSTAL_YELLOW; 3],
        },
    }
}

fn party_x(layout: &GardenLayout, river: &River) -> Option<u16> {
    let top = layout.horizon_y.saturating_sub(ROWS - 1);
    let river_x = (0..river.stretch_count())
        .filter_map(|index| river.stretch(index))
        .filter(|stretch| stretch.y >= top)
        .map(|stretch| stretch.x)
        .chain([puddle_x(layout.width)])
        .min()?;
    let latest = river_x.checked_sub(RIVER_GAP + WIDTH)?;
    let earliest = layout.house_x + layout.house.width() + PINE_OFFSET + PINE_WIDTH + 1;
    let x = if latest >= earliest {
        earliest + (latest - earliest) / 2
    } else {
        latest
    };
    (x >= layout.house_x).then_some(x)
}

pub fn render(
    renderer: &mut TerminalRenderer,
    layout: &GardenLayout,
    river: &River,
    guests: &[Neighbour],
) -> io::Result<()> {
    let Some(x) = party_x(layout, river) else {
        return Ok(());
    };
    let ground_y = layout.horizon_y;

    for row in 1..ROWS - 1 {
        for column in x..x + WIDTH {
            renderer.render_char(column, ground_y.saturating_sub(row), ' ', Color::Reset)?;
        }
    }
    garden::draw(
        renderer,
        x + TABLE_X + 1,
        ground_y.saturating_sub(2),
        &[FOOD],
        CAKE,
    )?;
    garden::draw(
        renderer,
        x + TABLE_X,
        ground_y.saturating_sub(1),
        &[CLOTH],
        Color::White,
    )?;
    garden::draw(renderer, x + TABLE_X, ground_y, &[LEGS], WOOD)?;
    for (offset, bench) in BENCHES {
        garden::draw(renderer, x + offset, ground_y, &[bench], WOOD)?;
    }

    for (neighbour, offset) in SEATS {
        if guests.contains(&neighbour) {
            render_guest(renderer, x + offset, ground_y, &look(neighbour))?;
        }
    }
    Ok(())
}

fn render_guest(
    renderer: &mut TerminalRenderer,
    x: u16,
    ground_y: u16,
    look: &Look,
) -> io::Result<()> {
    if let Some((hat, color)) = look.hat {
        renderer.render_char(x + 1, ground_y.saturating_sub(3), hat, color)?;
    }
    renderer.render_char(x + 1, ground_y.saturating_sub(2), 'o', kid::SKIN)?;
    for ((column, part), color) in BODY.chars().enumerate().zip(look.body) {
        renderer.render_char(x + column as u16, ground_y.saturating_sub(1), part, color)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn placed(width: u16, height: u16) -> (GardenLayout, River, u16) {
        let layout = GardenLayout::new(width, height);
        let river = River::new(width, layout.horizon_y);
        let x = party_x(&layout, &river).unwrap();
        (layout, river, x)
    }

    #[test]
    fn test_party_sits_in_front_of_the_house_before_the_river() {
        for (width, height) in [(100, 30), (121, 40), (160, 40), (200, 50), (300, 60)] {
            let (layout, river, x) = placed(width, height);
            let top = layout.horizon_y - (ROWS - 1);
            assert!(x >= layout.house_x, "{width}x{height}");
            assert!(x + WIDTH < puddle_x(width), "{width}x{height}");
            assert!(
                (0..river.stretch_count())
                    .filter_map(|index| river.stretch(index))
                    .filter(|stretch| stretch.y >= top)
                    .all(|stretch| x + WIDTH < stretch.x),
                "{width}x{height}"
            );
        }
    }

    #[test]
    fn test_party_keeps_clear_of_the_mailbox() {
        let (layout, _, x) = placed(121, 40);
        let mailbox_x = layout.plan.mailbox_x.unwrap();
        assert!(mailbox_x < x);
    }

    #[test]
    fn test_party_moves_past_the_pine_on_wide_terminals() {
        let (layout, _, x) = placed(260, 50);
        assert!(x > layout.house_x + layout.house.width() + PINE_OFFSET + PINE_WIDTH);
    }

    #[test]
    fn test_every_neighbour_has_a_seat() {
        for neighbour in Neighbour::ALL {
            assert!(SEATS.iter().any(|(seated, _)| *seated == neighbour));
        }
        assert!(SEATS.iter().all(|(_, offset)| offset + 3 <= WIDTH));
    }
}
