pub mod airplanes;
pub mod birds;
pub mod cat;
pub mod chimney;
pub mod cloud_bank;
pub mod clouds;
pub mod constellation;
pub mod creepy;
pub mod dog;
pub mod duck;
pub mod epiphany;
pub mod finds;
pub mod fireflies;
pub mod fireworks;
pub mod fog;
pub mod frog;
pub mod garden;
pub mod gust;
pub mod halloween;
pub mod kid;
pub mod leaves;
pub mod lore;
pub mod moon;
pub mod music;
pub mod new_year;
pub mod owl;
pub mod postman;
pub mod rainbow;
pub mod raindrops;
pub mod snail;
pub mod snow;
pub mod snow_cover;
pub mod snowman;
pub mod spring;
pub mod squirrel;
pub mod stars;
pub mod sun_path;
pub mod sunny;
pub mod thunderstorm;

use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

pub trait Animation {
    fn get_frame(&self, frame_number: usize) -> &[String];
    fn frame_count(&self) -> usize;

    fn get_color(&self) -> Color {
        Color::Reset
    }
}

pub struct AnimationController {
    current_frame: usize,
}

impl AnimationController {
    pub fn new() -> Self {
        Self { current_frame: 0 }
    }

    pub fn next_frame<A: Animation>(&mut self, animation: &A) -> usize {
        self.current_frame = (self.current_frame + 1) % animation.frame_count();
        self.current_frame
    }

    pub fn current<'a, A: Animation>(&self, animation: &'a A) -> &'a [String] {
        animation.get_frame(self.current_frame)
    }

    pub fn render_frame<A: Animation>(
        &self,
        renderer: &mut TerminalRenderer,
        animation: &A,
        y_offset: u16,
    ) -> io::Result<()> {
        let frame = animation.get_frame(self.current_frame);
        let color = animation.get_color();
        renderer.render_centered_colored(frame, y_offset, color)
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.current_frame = 0;
    }
}

impl Default for AnimationController {
    fn default() -> Self {
        Self::new()
    }
}
