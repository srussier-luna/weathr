use crate::animation::garden;
use crate::animation::sky_sprite::{self, SkySprite};
use crate::holiday::Holiday;
use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

const GOLDEN: [&str; 7] = [
    r"     _.-----._",
    r"  .-'  .   o  '-.",
    r" /  o    .    .  \",
    r"|  .   o    .   o |",
    r" \   .    o   .  /",
    r"  '-.  o   .  .-'",
    r"     '-------'",
];
const PALE: [&str; 7] = [
    r"     _.-----._",
    r"  .-'         '-.",
    r" /               \",
    r"|                 |",
    r" \               /",
    r"  '-.         .-'",
    r"     '-------'",
];
const GOLDEN_HALF: [&str; 5] = [
    r"    _.-------._",
    r" .-'  o    .   '-.",
    r"|  .    o    .  o |",
    r" '-.   .    o  .-'",
    r"    '---------'",
];
const PALE_HALF: [&str; 5] = [
    r"    _.-------._",
    r" .-'           '-.",
    r"|                 |",
    r" '-.           .-'",
    r"    '---------'",
];
const GOLDEN_THIN: [&str; 3] = [
    r"  _.-----------._",
    r"(  .   o   .   o  )",
    r"  '-------------'",
];
const PALE_THIN: [&str; 3] = [
    r"  _.-----------._",
    r"(                 )",
    r"  '-------------'",
];
const EDGE: [&str; 1] = [r"(=================)"];
const GOLDEN_COLOR: Color = Color::Rgb {
    r: 225,
    g: 155,
    b: 50,
};
const PALE_COLOR: Color = Color::Rgb {
    r: 250,
    g: 235,
    b: 195,
};
const REST_FRAMES: u32 = 150;
const FLIP_STEP_FRAMES: u32 = 3;
const FLIP_STEPS: u32 = 5;
const CYCLE_FRAMES: u32 = REST_FRAMES + FLIP_STEPS * FLIP_STEP_FRAMES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Side {
    Golden,
    Pale,
}

impl Side {
    fn flipped(self) -> Self {
        match self {
            Self::Golden => Self::Pale,
            Self::Pale => Self::Golden,
        }
    }

    fn color(self) -> Color {
        match self {
            Self::Golden => GOLDEN_COLOR,
            Self::Pale => PALE_COLOR,
        }
    }

    fn full(self) -> &'static [&'static str] {
        match self {
            Self::Golden => &GOLDEN,
            Self::Pale => &PALE,
        }
    }

    fn half(self) -> &'static [&'static str] {
        match self {
            Self::Golden => &GOLDEN_HALF,
            Self::Pale => &PALE_HALF,
        }
    }

    fn thin(self) -> &'static [&'static str] {
        match self {
            Self::Golden => &GOLDEN_THIN,
            Self::Pale => &PALE_THIN,
        }
    }
}

pub struct GiantCrepe {
    frame: u32,
}

impl GiantCrepe {
    pub const HEIGHT: u16 = GOLDEN.len() as u16;
    pub const WIDTH: u16 = EDGE[0].len() as u16;

    pub fn new() -> Self {
        Self { frame: 0 }
    }

    pub fn shows(holiday: &Holiday, horizon_y: u16) -> bool {
        holiday.is_candlemas && sky_sprite::fits(Self::HEIGHT, horizon_y)
    }

    fn pose(&self) -> (Side, &'static [&'static str]) {
        let up = if (self.frame / CYCLE_FRAMES) % 2 == 0 {
            Side::Golden
        } else {
            Side::Pale
        };
        let Some(flipping) = (self.frame % CYCLE_FRAMES).checked_sub(REST_FRAMES) else {
            return (up, up.full());
        };
        let down = up.flipped();
        match flipping / FLIP_STEP_FRAMES {
            0 => (up, up.half()),
            1 => (up, up.thin()),
            2 => (up, &EDGE),
            3 => (down, down.thin()),
            _ => (down, down.half()),
        }
    }
}

impl SkySprite for GiantCrepe {
    fn size(&self) -> (u16, u16) {
        (Self::WIDTH, Self::HEIGHT)
    }

    fn update(&mut self) {
        self.frame = self.frame.wrapping_add(1);
    }

    fn render(&self, renderer: &mut TerminalRenderer, x: u16, y: u16) -> io::Result<()> {
        let (side, lines) = self.pose();
        let top = y + (Self::HEIGHT - lines.len() as u16) / 2;
        garden::draw_solid(renderer, x, top, lines, side.color())
    }
}

impl Default for GiantCrepe {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::cloud_bank::CloudBank;
    use crate::animation::music::SkyNotes;
    use chrono::NaiveDate;

    const POSES: [&[&str]; 7] = [
        &GOLDEN,
        &PALE,
        &GOLDEN_HALF,
        &PALE_HALF,
        &GOLDEN_THIN,
        &PALE_THIN,
        &EDGE,
    ];

    fn holiday_on(month: u32, day: u32) -> Holiday {
        Holiday::on(NaiveDate::from_ymd_opt(2027, month, day).unwrap())
    }

    fn after(frames: u32) -> GiantCrepe {
        let mut crepe = GiantCrepe::new();
        for _ in 0..frames {
            crepe.update();
        }
        crepe
    }

    #[test]
    fn test_giant_crepe_is_a_round_nineteen_by_seven() {
        assert_eq!(GiantCrepe::WIDTH, 19);
        assert_eq!(GiantCrepe::HEIGHT, 7);
        for lines in POSES {
            assert!(lines.len() as u16 <= GiantCrepe::HEIGHT);
            assert_eq!(lines.len() % 2, 1);
            for line in lines {
                let indent = line.len() - line.trim_start().len();
                assert_eq!(line.trim_end(), *line);
                assert_eq!(indent + line.len(), GiantCrepe::WIDTH as usize, "{line}");
            }
        }
    }

    #[test]
    fn test_golden_side_has_spots_and_pale_side_none() {
        for lines in [&GOLDEN[..], &GOLDEN_HALF, &GOLDEN_THIN] {
            assert!(lines.iter().any(|line| line.contains('o')));
        }
        for lines in [&PALE[..], &PALE_HALF, &PALE_THIN] {
            assert!(lines.iter().all(|line| !line.contains('o')));
        }
        assert_ne!(Side::Golden.color(), Side::Pale.color());
    }

    #[test]
    fn test_giant_crepe_only_on_candlemas() {
        assert!(GiantCrepe::shows(&holiday_on(2, 2), 17));
        assert!(GiantCrepe::shows(&Holiday::candlemas(), 17));
        assert!(!GiantCrepe::shows(&holiday_on(2, 1), 17));
        assert!(!GiantCrepe::shows(&holiday_on(2, 3), 17));
        assert!(!GiantCrepe::shows(&Holiday::default(), 17));
        assert!(!GiantCrepe::shows(&Holiday::music_day(), 17));
        assert!(!SkyNotes::shows(&Holiday::candlemas(), 17));
    }

    #[test]
    fn test_short_sky_keeps_sun_and_moon() {
        let lowest_horizon = CloudBank::TOP + GiantCrepe::HEIGHT;
        assert!(GiantCrepe::shows(&Holiday::candlemas(), lowest_horizon));
        assert!(!GiantCrepe::shows(
            &Holiday::candlemas(),
            lowest_horizon - 1
        ));
    }

    #[test]
    fn test_giant_crepe_rests_golden_side_up_at_first() {
        for frames in [0, REST_FRAMES - 1] {
            assert_eq!(after(frames).pose(), (Side::Golden, &GOLDEN[..]));
        }
    }

    #[test]
    fn test_giant_crepe_flips_through_its_edge() {
        let flip: Vec<_> = (REST_FRAMES..CYCLE_FRAMES)
            .map(|frame| after(frame).pose().1)
            .collect();
        let expected = [
            &GOLDEN_HALF[..],
            &GOLDEN_THIN,
            &EDGE,
            &PALE_THIN,
            &PALE_HALF,
        ];
        for (step, poses) in flip.chunks(FLIP_STEP_FRAMES as usize).enumerate() {
            assert!(poses.iter().all(|pose| *pose == expected[step]), "{step}");
        }
        assert_eq!(flip.len() as u32, FLIP_STEPS * FLIP_STEP_FRAMES);
    }

    #[test]
    fn test_giant_crepe_flips_now_and_then() {
        assert_eq!(after(CYCLE_FRAMES).pose(), (Side::Pale, &PALE[..]));
        assert_eq!(
            after(CYCLE_FRAMES + REST_FRAMES - 1).pose(),
            (Side::Pale, &PALE[..])
        );
        assert_eq!(after(2 * CYCLE_FRAMES).pose(), (Side::Golden, &GOLDEN[..]));
    }
}
