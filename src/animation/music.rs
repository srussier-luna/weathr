use crate::animation::cloud_bank::CloudBank;
use crate::animation::garden::{self, GardenLayout};
use crate::holiday::Holiday;
use crate::render::TerminalRenderer;
use crossterm::style::Color;
use rand::prelude::*;
use std::f32::consts::TAU;
use std::io;

const GLYPHS: [char; 2] = ['♪', '♫'];
const PALETTE: [Color; 4] = [
    Color::Rgb {
        r: 255,
        g: 105,
        b: 180,
    },
    Color::Rgb {
        r: 255,
        g: 215,
        b: 0,
    },
    Color::Rgb {
        r: 0,
        g: 220,
        b: 255,
    },
    Color::Rgb {
        r: 150,
        g: 255,
        b: 120,
    },
];
const MAX_NOTES: usize = 16;
const SPAWN_CHANCE: f32 = 0.15;
const SING_CHANCE: f32 = 0.01;
const RISE_ROWS: f32 = 4.0;
const RISE_SPEED: f32 = 0.06;
const WOBBLE_AMPLITUDE: f32 = 1.5;
const WOBBLE_SPEED: f32 = 0.18;
const SKY_NOTES: [&str; 12] = [
    r"        ###################",
    r"        ###################",
    r"        |        |        |",
    r"        |        ##########",
    r"        |        ##########",
    r"        |        |        |",
    r"        |        |        |",
    r"        |        |        |",
    r"    _.-'|    _.-'|    _.-'|",
    r"  .'    |  .'    |  .'    |",
    r" (     .' (     .' (     .'",
    r"  `-.-'    `-.-'    `-.-'",
];
const COLOR_STEP_FRAMES: u32 = 90;

struct Note {
    origin_x: f32,
    y: f32,
    top: f32,
    phase: f32,
    glyph: char,
    color: Color,
}

impl Note {
    fn x(&self) -> f32 {
        self.origin_x + self.phase.sin() * WOBBLE_AMPLITUDE
    }
}

pub struct MusicNotes {
    notes: Vec<Note>,
}

impl MusicNotes {
    pub fn new() -> Self {
        Self {
            notes: Vec::with_capacity(MAX_NOTES),
        }
    }

    pub fn update(
        &mut self,
        holiday: &Holiday,
        layout: &GardenLayout,
        singers: &[(u16, u16)],
        rng: &mut impl Rng,
    ) {
        if !holiday.is_music_day {
            self.notes.clear();
            return;
        }

        self.rise();

        if rng.random::<f32>() < SPAWN_CHANCE {
            if let Some(&window) = layout.house.windows().choose(rng) {
                self.play_window(layout, window, rng);
            }
        }

        for &(x, head_y) in singers {
            if head_y > 0 && rng.random::<f32>() < SING_CHANCE {
                let y = head_y - 1;
                self.play(x, y, rise_top(y), rng);
            }
        }
    }

    fn rise(&mut self) {
        for note in &mut self.notes {
            note.y -= RISE_SPEED;
            note.phase = (note.phase + WOBBLE_SPEED) % TAU;
        }
        self.notes.retain(|note| note.y >= note.top);
    }

    fn play_window(&mut self, layout: &GardenLayout, (x, y): (u16, u16), rng: &mut impl Rng) {
        let roof_line = layout.house_y + layout.house.eave_row() + 1;
        let y = layout.house_y + y;
        let top = if y > roof_line {
            roof_line as f32
        } else {
            rise_top(y)
        };
        self.play(layout.house_x + x, y, top, rng);
    }

    fn play(&mut self, x: u16, y: u16, top: f32, rng: &mut impl Rng) {
        if self.notes.len() >= MAX_NOTES {
            return;
        }
        self.notes.push(Note {
            origin_x: x as f32,
            y: y as f32,
            top,
            phase: 0.0,
            glyph: GLYPHS[rng.random_range(0..GLYPHS.len())],
            color: PALETTE[rng.random_range(0..PALETTE.len())],
        });
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        for note in &self.notes {
            let x = note.x().round();
            if x < 0.0 {
                continue;
            }
            renderer.render_char(x as u16, note.y as u16, note.glyph, note.color)?;
        }
        Ok(())
    }
}

fn rise_top(y: u16) -> f32 {
    (y as f32 - RISE_ROWS + 1.0).max(0.0)
}

impl Default for MusicNotes {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SkyNotes {
    frame: u32,
}

impl SkyNotes {
    pub const HEIGHT: u16 = SKY_NOTES.len() as u16;
    pub const WIDTH: u16 = SKY_NOTES[0].len() as u16;

    pub fn new() -> Self {
        Self { frame: 0 }
    }

    pub fn shows(holiday: &Holiday, horizon_y: u16) -> bool {
        holiday.is_music_day && horizon_y >= CloudBank::TOP + Self::HEIGHT
    }

    pub fn update(&mut self) {
        self.frame = self.frame.wrapping_add(1);
    }

    fn color(&self) -> Color {
        PALETTE[(self.frame / COLOR_STEP_FRAMES) as usize % PALETTE.len()]
    }

    pub fn render(&self, renderer: &mut TerminalRenderer, x: u16, y: u16) -> io::Result<()> {
        garden::draw(renderer, x, y, &SKY_NOTES, self.color())
    }
}

impl Default for SkyNotes {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::WorldScene;
    use rand::rngs::StdRng;

    const SINGER: (u16, u16) = (70, 20);

    fn play(
        music: &mut MusicNotes,
        holiday: &Holiday,
        layout: &GardenLayout,
        singers: &[(u16, u16)],
        frames: usize,
        rng: &mut StdRng,
    ) -> usize {
        let mut most = 0;
        for _ in 0..frames {
            music.update(holiday, layout, singers, rng);
            most = most.max(music.notes.len());
        }
        most
    }

    fn sings(music: &MusicNotes) -> bool {
        music
            .notes
            .iter()
            .any(|note| note.origin_x == SINGER.0 as f32)
    }

    #[test]
    fn test_several_notes_play_at_once() {
        let layout = GardenLayout::new(80, 24);
        let mut rng = StdRng::seed_from_u64(21);
        let mut music = MusicNotes::new();
        let most = play(
            &mut music,
            &Holiday::music_day(),
            &layout,
            &[],
            300,
            &mut rng,
        );
        assert!(most >= 3);
    }

    #[test]
    fn test_notes_rise_from_windows_up_to_a_few_rows_above_the_eave() {
        for layout in [GardenLayout::new(80, 24), GardenLayout::new(200, 40)] {
            let house = layout.house;
            let window_xs: Vec<f32> = house
                .windows()
                .iter()
                .map(|(x, _)| (layout.house_x + x) as f32)
                .collect();
            let lowest_window = house.windows().iter().map(|(_, y)| *y).max().unwrap();
            let mut rng = StdRng::seed_from_u64(21);
            let mut music = MusicNotes::new();
            for _ in 0..300 {
                music.update(&Holiday::music_day(), &layout, &[], &mut rng);
                for note in &music.notes {
                    assert!(window_xs.contains(&note.origin_x));
                    assert!((note.x() - note.origin_x).abs() <= WOBBLE_AMPLITUDE);
                    assert!(note.y > (layout.house_y + house.eave_row()) as f32 - RISE_ROWS);
                    assert!(note.y <= (layout.house_y + lowest_window) as f32);
                }
            }
        }
    }

    #[test]
    fn test_every_window_note_stays_on_screen_for_a_while() {
        for layout in [GardenLayout::new(80, 24), GardenLayout::new(200, 40)] {
            let mut rng = StdRng::seed_from_u64(21);
            for window in layout.house.windows() {
                let mut music = MusicNotes::new();
                music.play_window(&layout, window, &mut rng);
                for _ in 0..12 {
                    music.rise();
                }
                assert_eq!(music.notes.len(), 1, "window {window:?}");
            }
        }
    }

    #[test]
    fn test_singers_only_sing_on_music_day() {
        let layout = GardenLayout::new(80, 24);
        let mut rng = StdRng::seed_from_u64(21);
        let mut music = MusicNotes::new();

        let most = play(
            &mut music,
            &Holiday::default(),
            &layout,
            &[SINGER],
            600,
            &mut rng,
        );
        assert_eq!(most, 0);

        let mut has_sung = false;
        for _ in 0..600 {
            music.update(&Holiday::music_day(), &layout, &[SINGER], &mut rng);
            has_sung |= sings(&music);
        }
        assert!(has_sung);
    }

    #[test]
    fn test_notes_rise_a_few_rows_above_the_head() {
        let layout = GardenLayout::new(80, 24);
        let mut rng = StdRng::seed_from_u64(21);
        let mut music = MusicNotes::new();
        for _ in 0..600 {
            music.update(&Holiday::music_day(), &layout, &[SINGER], &mut rng);
            for note in music
                .notes
                .iter()
                .filter(|note| note.origin_x == SINGER.0 as f32)
            {
                assert!(note.y < SINGER.1 as f32);
                assert!(note.y > (SINGER.1 - 1) as f32 - RISE_ROWS);
            }
        }
    }

    #[test]
    fn test_hidden_singer_sings_nothing() {
        let layout = GardenLayout::new(80, 24);
        let mut rng = StdRng::seed_from_u64(21);
        let mut music = MusicNotes::new();
        play(
            &mut music,
            &Holiday::music_day(),
            &layout,
            &[SINGER],
            600,
            &mut rng,
        );

        for frame in 0..600 {
            music.update(&Holiday::music_day(), &layout, &[], &mut rng);
            if frame > 100 {
                assert!(!sings(&music));
            }
        }
    }

    #[test]
    fn test_sky_notes_are_twelve_lines_tall() {
        assert_eq!(SkyNotes::HEIGHT, 12);
        assert_eq!(SkyNotes::WIDTH, 27);
        assert!(
            SKY_NOTES
                .iter()
                .all(|line| line.len() <= SkyNotes::WIDTH as usize)
        );
        assert!(SKY_NOTES.iter().all(|line| line.trim_end() == *line));
    }

    #[test]
    fn test_sky_notes_only_on_music_day() {
        let date = |day| chrono::NaiveDate::from_ymd_opt(2027, 6, day).unwrap();
        assert!(SkyNotes::shows(&Holiday::on(date(21)), 17));
        assert!(SkyNotes::shows(&Holiday::music_day(), 17));
        assert!(!SkyNotes::shows(&Holiday::on(date(20)), 17));
        assert!(!SkyNotes::shows(&Holiday::on(date(22)), 17));
        assert!(!SkyNotes::shows(&Holiday::default(), 17));
    }

    #[test]
    fn test_short_sky_keeps_sun_and_moon() {
        let lowest_horizon = CloudBank::TOP + SkyNotes::HEIGHT;
        assert!(SkyNotes::shows(&Holiday::music_day(), lowest_horizon));
        assert!(!SkyNotes::shows(&Holiday::music_day(), lowest_horizon - 1));
        assert!(!SkyNotes::shows(
            &Holiday::music_day(),
            20 - WorldScene::GROUND_HEIGHT
        ));
        assert!(SkyNotes::shows(
            &Holiday::music_day(),
            21 - WorldScene::GROUND_HEIGHT
        ));
    }

    #[test]
    fn test_sky_notes_change_color_from_time_to_time() {
        let mut sky = SkyNotes::new();
        let mut colors = vec![sky.color()];
        for frame in 1..=COLOR_STEP_FRAMES * PALETTE.len() as u32 {
            let before = sky.color();
            sky.update();
            if frame % COLOR_STEP_FRAMES == 0 {
                assert_ne!(sky.color(), before);
                colors.push(sky.color());
            } else {
                assert_eq!(sky.color(), before);
            }
        }
        assert!(PALETTE.iter().all(|color| colors.contains(color)));
        assert_eq!(colors.last(), colors.first());
    }
}
