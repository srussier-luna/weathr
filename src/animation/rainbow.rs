use crate::animation::cloud_bank::CloudBank;
use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::f32::consts::PI;
use std::io;

const MAX_RADIUS: f32 = 10.0;
const WIDTH_RATIO: f32 = 2.2;
const STEPS: u16 = 240;
const BANDS: [Color; 6] = [
    Color::Red,
    Color::Rgb {
        r: 255,
        g: 140,
        b: 0,
    },
    Color::Yellow,
    Color::Green,
    Color::Blue,
    Color::Magenta,
];

#[derive(Default)]
pub struct Rainbow;

impl Rainbow {
    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        terminal_width: u16,
        horizon_y: u16,
    ) -> io::Result<()> {
        let center_x = terminal_width as f32 * 0.75;
        let base_y = horizon_y as f32;
        let height = (horizon_y.saturating_sub(CloudBank::bottom() + 2) as f32).min(MAX_RADIUS);

        for (band, color) in BANDS.iter().enumerate() {
            let radius_y = height - band as f32;
            if radius_y < 2.0 {
                break;
            }
            let radius_x = radius_y * WIDTH_RATIO;

            for step in 0..=STEPS {
                let angle = PI * step as f32 / STEPS as f32;
                let x = (center_x + radius_x * angle.cos()).round();
                let y = (base_y - radius_y * angle.sin()).round();
                if x < 0.0 || y < 0.0 {
                    continue;
                }
                let rise = radius_y * angle.cos().abs();
                let run = radius_x * angle.sin() * 0.5;
                let ch = if rise < run {
                    '-'
                } else if angle > PI / 2.0 {
                    '/'
                } else {
                    '\\'
                };
                renderer.render_char(x as u16, y as u16, ch, *color)?;
            }
        }
        Ok(())
    }
}
