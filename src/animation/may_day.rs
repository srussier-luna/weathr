use crate::animation::garden;
use crate::animation::sky_sprite::{self, SkySprite};
use crate::holiday::Holiday;
use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

const MUGUET: [&str; 9] = [
    r"        _.---._",
    r"      .' |   |`.",
    r"     |  (_) (_) \",
    r"     |          (_)",
    r"  _  |  _",
    r" ( \ | / )",
    r"  \ \|/ /",
    r"   \ | /",
    r"    '-'",
];
const BELL: &str = "(_)";
const STALK_COLOR: Color = Color::Rgb {
    r: 60,
    g: 170,
    b: 60,
};
const BELL_COLOR: Color = Color::White;

pub struct GiantMuguet;

impl GiantMuguet {
    pub const HEIGHT: u16 = MUGUET.len() as u16;
    pub const WIDTH: u16 = 19;

    pub fn shows(holiday: &Holiday, horizon_y: u16) -> bool {
        holiday.is_may_day && sky_sprite::fits(Self::HEIGHT, horizon_y)
    }
}

impl SkySprite for GiantMuguet {
    fn size(&self) -> (u16, u16) {
        (Self::WIDTH, Self::HEIGHT)
    }

    fn update(&mut self) {}

    fn render(&self, renderer: &mut TerminalRenderer, x: u16, y: u16) -> io::Result<()> {
        garden::draw_solid(renderer, x, y, &MUGUET, STALK_COLOR)?;
        for (row, line) in (y..).zip(MUGUET) {
            for (col, bell) in line.match_indices(BELL) {
                garden::draw(renderer, x + col as u16, row, &[bell], BELL_COLOR)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::candlemas::GiantCrepe;
    use crate::animation::cloud_bank::CloudBank;
    use crate::animation::music::SkyNotes;
    use chrono::NaiveDate;

    fn holiday_on(month: u32, day: u32) -> Holiday {
        Holiday::on(NaiveDate::from_ymd_opt(2027, month, day).unwrap())
    }

    #[test]
    fn test_giant_muguet_is_a_nineteen_by_nine() {
        assert_eq!(GiantMuguet::WIDTH, 19);
        assert_eq!(GiantMuguet::HEIGHT, 9);
        assert_eq!(
            MUGUET.iter().map(|line| line.len()).max(),
            Some(GiantMuguet::WIDTH as usize)
        );
        for line in MUGUET {
            assert_eq!(line.trim_end(), line);
        }
    }

    #[test]
    fn test_white_bells_hang_above_the_green_leaves() {
        let bell_rows: Vec<usize> = MUGUET
            .iter()
            .enumerate()
            .flat_map(|(row, line)| line.match_indices(BELL).map(move |_| row))
            .collect();
        assert_eq!(bell_rows, [2, 2, 3]);
        assert_ne!(STALK_COLOR, BELL_COLOR);
    }

    #[test]
    fn test_giant_muguet_only_on_may_day() {
        assert!(GiantMuguet::shows(&holiday_on(5, 1), 17));
        assert!(GiantMuguet::shows(&Holiday::may_day(), 17));
        assert!(!GiantMuguet::shows(&holiday_on(4, 30), 17));
        assert!(!GiantMuguet::shows(&holiday_on(5, 2), 17));
        assert!(!GiantMuguet::shows(&Holiday::default(), 17));
        assert!(!GiantMuguet::shows(&Holiday::candlemas(), 17));
        assert!(!GiantCrepe::shows(&Holiday::may_day(), 17));
        assert!(!SkyNotes::shows(&Holiday::may_day(), 17));
    }

    #[test]
    fn test_short_sky_keeps_sun_and_moon() {
        let lowest_horizon = CloudBank::TOP + GiantMuguet::HEIGHT;
        assert!(GiantMuguet::shows(&Holiday::may_day(), lowest_horizon));
        assert!(!GiantMuguet::shows(&Holiday::may_day(), lowest_horizon - 1));
    }
}
