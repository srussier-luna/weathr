use crate::animation::garden::GardenLayout;
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
const HEAD_RISE: f32 = 4.0;
const RISE_SPEED: f32 = 0.06;
const WOBBLE_AMPLITUDE: f32 = 1.5;
const WOBBLE_SPEED: f32 = 0.18;

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

        for note in &mut self.notes {
            note.y -= RISE_SPEED;
            note.phase = (note.phase + WOBBLE_SPEED) % TAU;
        }
        self.notes.retain(|note| note.y >= note.top);

        if rng.random::<f32>() < SPAWN_CHANCE {
            let roof_line = layout.house_y + layout.house.eave_row() + 1;
            if let Some(&(x, y)) = layout.house.windows().choose(rng) {
                self.play(
                    layout.house_x + x,
                    layout.house_y + y,
                    roof_line as f32,
                    rng,
                );
            }
        }

        for &(x, head_y) in singers {
            if head_y > 0 && rng.random::<f32>() < SING_CHANCE {
                let y = head_y - 1;
                self.play(x, y, (y as f32 - HEAD_RISE + 1.0).max(0.0), rng);
            }
        }
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
            glyph: *GLYPHS.choose(rng).unwrap_or(&GLYPHS[0]),
            color: *PALETTE.choose(rng).unwrap_or(&PALETTE[0]),
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

impl Default for MusicNotes {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_notes_rise_from_windows_and_stay_under_the_roof() {
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
                    assert!(note.y as u16 > layout.house_y + house.eave_row());
                    assert!(note.y <= (layout.house_y + lowest_window) as f32);
                }
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
            for note in music.notes.iter().filter(|note| note.origin_x == 70.0) {
                assert!(note.y < SINGER.1 as f32);
                assert!(note.y > (SINGER.1 - 1) as f32 - HEAD_RISE);
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
}
