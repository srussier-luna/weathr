use crate::animation::garden::{self, GardenLayout};
use crate::render::TerminalRenderer;
use crate::scene::decorations::ARCANE_VIOLET;
use std::io;

const CARD: [&str; 2] = ["[*]", " R "];
const CARD_WIDTH: u16 = 3;

fn card_x(layout: &GardenLayout) -> u16 {
    (layout.house_x + layout.house.width() / 2).saturating_sub(CARD_WIDTH / 2)
}

pub fn render(renderer: &mut TerminalRenderer, layout: &GardenLayout) -> io::Result<()> {
    garden::draw(
        renderer,
        card_x(layout),
        layout.horizon_y,
        &CARD,
        ARCANE_VIOLET,
    )
}
