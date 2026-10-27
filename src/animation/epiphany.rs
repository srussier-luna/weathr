use crate::animation::garden::{self, GardenLayout};
use crate::render::TerminalRenderer;
use crate::scene::house::House;
use crossterm::style::Color;
use std::io;

const CROWN: &str = "\\/\\/";
const CROWN_GOLD: Color = Color::Rgb {
    r: 255,
    g: 200,
    b: 40,
};

pub fn crown_offset(house: &House) -> (u16, u16) {
    let (peak_x, peak_y) = house.peak();
    let half = CROWN.len() as u16 / 2;
    (peak_x.saturating_sub(half), peak_y.saturating_sub(1))
}

pub fn render_crown(renderer: &mut TerminalRenderer, layout: &GardenLayout) -> io::Result<()> {
    let (dx, dy) = crown_offset(&layout.house);
    garden::draw(
        renderer,
        layout.house_x + dx,
        layout.house_y + dy,
        &[CROWN],
        CROWN_GOLD,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crown_sits_on_small_house_peak() {
        let house = House::for_terminal(80, 24);
        assert_eq!(house.peak(), (17, 3));
        assert_eq!(crown_offset(&house), (15, 2));
    }

    #[test]
    fn test_crown_sits_on_large_house_peak() {
        let house = House::for_terminal(200, 40);
        assert_eq!(house.peak(), (26, 1));
        assert_eq!(crown_offset(&house), (24, 0));
    }
}
