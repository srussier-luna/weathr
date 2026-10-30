use crate::animation::garden::GardenLayout;
use crate::render::TerminalRenderer;
use crate::scene::house::House;
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
const MAX_NOTES: usize = 10;
const SPAWN_CHANCE: f32 = 0.15;
const RISE_SPEED: f32 = 0.06;
const WOBBLE_AMPLITUDE: f32 = 1.5;
const WOBBLE_SPEED: f32 = 0.18;

struct Note {
    origin_x: f32,
    y: f32,
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

    pub fn update(&mut self, house: &House, rng: &mut impl Rng) {
        let roof_line = house.eave_row() as f32 + 1.0;
        for note in &mut self.notes {
            note.y -= RISE_SPEED;
            note.phase = (note.phase + WOBBLE_SPEED) % TAU;
        }
        self.notes.retain(|note| note.y >= roof_line);

        if self.notes.len() < MAX_NOTES && rng.random::<f32>() < SPAWN_CHANCE {
            let windows = house.windows();
            if let Some(&(x, y)) = windows.choose(rng) {
                self.notes.push(Note {
                    origin_x: x as f32,
                    y: y as f32,
                    phase: 0.0,
                    glyph: *GLYPHS.choose(rng).unwrap_or(&GLYPHS[0]),
                    color: *PALETTE.choose(rng).unwrap_or(&PALETTE[0]),
                });
            }
        }
    }

    pub fn render(&self, renderer: &mut TerminalRenderer, layout: &GardenLayout) -> io::Result<()> {
        for note in &self.notes {
            let x = (layout.house_x as f32 + note.x()).round();
            if x < 0.0 {
                continue;
            }
            renderer.render_char(
                x as u16,
                layout.house_y + note.y as u16,
                note.glyph,
                note.color,
            )?;
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

    fn play(house: &House, frames: usize) -> MusicNotes {
        let mut rng = StdRng::seed_from_u64(21);
        let mut music = MusicNotes::new();
        for _ in 0..frames {
            music.update(house, &mut rng);
        }
        music
    }

    #[test]
    fn test_several_notes_play_at_once() {
        let house = House::for_terminal(80, 24);
        let mut rng = StdRng::seed_from_u64(21);
        let mut music = MusicNotes::new();
        let mut most = 0;
        for _ in 0..300 {
            music.update(&house, &mut rng);
            most = most.max(music.notes.len());
        }
        assert!(most >= 3);
    }

    #[test]
    fn test_notes_rise_from_windows_and_stay_under_the_roof() {
        for house in [House::for_terminal(80, 24), House::for_terminal(200, 40)] {
            let window_xs: Vec<f32> = house.windows().iter().map(|(x, _)| *x as f32).collect();
            let lowest_window = house.windows().iter().map(|(_, y)| *y).max().unwrap();
            for frames in 1..300 {
                for note in &play(&house, frames).notes {
                    assert!(window_xs.contains(&note.origin_x));
                    assert!((note.x() - note.origin_x).abs() <= WOBBLE_AMPLITUDE);
                    assert!(note.y as u16 > house.eave_row());
                    assert!(note.y <= lowest_window as f32);
                }
            }
        }
    }
}
