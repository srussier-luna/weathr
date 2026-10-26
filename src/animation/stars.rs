use crate::render::TerminalRenderer;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

#[derive(Clone, Copy)]
struct Star {
    x: u16,
    y: u16,
    brightness: f32,
    phase: f32,
}

struct ShootingStar {
    x: f32,
    y: f32,
    speed_x: f32,
    speed_y: f32,
    length: usize,
    active: bool,
}

pub struct StarSystem {
    stars: Vec<Star>,
    shooting_star: Option<ShootingStar>,
    is_meteor_shower: bool,
    is_sky_clear: bool,
    terminal_width: u16,
    terminal_height: u16,
}

impl StarSystem {
    const MIN_DISTANCE: f32 = 3.0; // Minimum distance between stars
    const SHOOTING_STAR_CHANCE: f32 = 0.005;
    const METEOR_SHOWER_SHOOTING_STAR_CHANCE: f32 = 0.05;

    pub fn new(terminal_width: u16, terminal_height: u16) -> Self {
        let stars = Self::create_stars(terminal_width, terminal_height, &[]);

        Self {
            stars,
            shooting_star: None,
            is_meteor_shower: false,
            is_sky_clear: true,
            terminal_width,
            terminal_height,
        }
    }

    fn create_stars(terminal_width: u16, terminal_height: u16, inital_stars: &[Star]) -> Vec<Star> {
        let mut rng = rand::rng();
        let count = (terminal_width as usize * terminal_height as usize) / 80; // Density

        if count < inital_stars.len() {
            return inital_stars.to_vec();
        }

        let mut stars = Vec::with_capacity(count);

        stars.extend(inital_stars.iter().cloned());

        for _ in 0..count {
            let mut attempts = 0;
            let max_attempts = 50;

            loop {
                let x = rng.random::<u16>() % terminal_width;
                let y = rng.random::<u16>() % (terminal_height / 2); // Upper half

                // Check if this position is far enough from existing stars
                let too_close = stars.iter().any(|star: &Star| {
                    let dx = (star.x as f32 - x as f32).abs();
                    let dy = (star.y as f32 - y as f32).abs();
                    let distance = (dx * dx + dy * dy).sqrt();
                    distance < Self::MIN_DISTANCE
                });

                if !too_close || attempts >= max_attempts {
                    stars.push(Star {
                        x,
                        y,
                        brightness: rng.random::<f32>(),
                        phase: rng.random::<f32>() * std::f32::consts::TAU,
                    });
                    break;
                }

                attempts += 1;
            }
        }

        stars
    }

    pub fn set_meteor_shower(&mut self, is_meteor_shower: bool) {
        self.is_meteor_shower = is_meteor_shower;
    }

    pub fn set_sky_clear(&mut self, is_sky_clear: bool) {
        self.is_sky_clear = is_sky_clear;
        if !is_sky_clear {
            self.shooting_star = None;
        }
    }

    fn shooting_star_chance(&self) -> f32 {
        if !self.is_sky_clear {
            0.0
        } else if self.is_meteor_shower {
            Self::METEOR_SHOWER_SHOOTING_STAR_CHANCE
        } else {
            Self::SHOOTING_STAR_CHANCE
        }
    }

    pub fn update(&mut self, terminal_width: u16, terminal_height: u16, rng: &mut impl Rng) {
        if terminal_width != self.terminal_width || terminal_height != self.terminal_height {
            // Fix stars not resizing
            self.stars = Self::create_stars(terminal_width, terminal_height, &self.stars);

            self.terminal_width = terminal_width;
            self.terminal_height = terminal_height;

            return;
        }

        // Twinkle
        for star in &mut self.stars {
            star.phase += 0.05;
            star.brightness = (star.phase.sin() + 1.0) / 2.0; // 0.0 to 1.0
        }

        // Shooting Star Logic
        if let Some(ref mut star) = self.shooting_star {
            star.x += star.speed_x;
            star.y += star.speed_y;

            if star.x < 0.0 || star.y as u16 >= terminal_height || star.length == 0 {
                self.shooting_star = None;
            }
        } else if rng.random::<f32>() < self.shooting_star_chance() {
            let start_x = (rng.random::<u16>() % (terminal_width / 2)) + (terminal_width / 4);
            let start_y = rng.random::<u16>() % (terminal_height / 4);

            self.shooting_star = Some(ShootingStar {
                x: start_x as f32,
                y: start_y as f32,
                speed_x: if rng.random::<bool>() { 0.9 } else { -0.9 },
                speed_y: 0.3 + (rng.random::<f32>() * 0.2),
                length: 9,
                active: true,
            });
        }
    }

    pub fn has_shooting_star(&self) -> bool {
        self.shooting_star.as_ref().is_some_and(|star| star.active)
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        for star in &self.stars {
            let ch = if star.brightness > 0.8 {
                '*'
            } else if star.brightness > 0.4 {
                '+'
            } else {
                '.'
            };
            let color = if star.brightness > 0.6 {
                Color::White
            } else {
                Color::DarkGrey
            };

            renderer.render_char(star.x, star.y, ch, color)?;
        }

        if let Some(ref star) = self.shooting_star
            && star.active
        {
            let head_x = star.x as i16;
            let head_y = star.y as i16;

            if head_x >= 0
                && head_x < self.terminal_width as i16
                && head_y >= 0
                && head_y < self.terminal_height as i16
            {
                renderer.render_char(head_x as u16, head_y as u16, '*', Color::White)?;
            }

            for i in 1..star.length {
                let trail_x = (star.x - (star.speed_x * i as f32)) as i16;
                let trail_y = (star.y - (star.speed_y * i as f32)) as i16;

                if trail_x >= 0
                    && trail_x < self.terminal_width as i16
                    && trail_y >= 0
                    && trail_y < self.terminal_height as i16
                {
                    let (ch, color) = match i {
                        1 => ('+', Color::White),
                        2..=4 => ('-', Color::Grey),
                        _ => ('.', Color::DarkGrey),
                    };
                    renderer.render_char(trail_x as u16, trail_y as u16, ch, color)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIDTH: u16 = 120;
    const HEIGHT: u16 = 45;
    const FRAMES: u32 = 300_000;

    fn count_shooting_stars(is_meteor_shower: bool, is_sky_clear: bool) -> u32 {
        let mut rng = StdRng::seed_from_u64(15);
        let mut stars = StarSystem::new(WIDTH, HEIGHT);
        stars.set_meteor_shower(is_meteor_shower);
        stars.set_sky_clear(is_sky_clear);
        let mut count = 0;
        let mut was_visible = false;

        for _ in 0..FRAMES {
            stars.update(WIDTH, HEIGHT, &mut rng);
            let is_visible = stars.has_shooting_star();
            if is_visible && !was_visible {
                count += 1;
            }
            was_visible = is_visible;
        }

        count
    }

    #[test]
    fn test_meteor_shower_brings_about_three_times_more_shooting_stars() {
        let normal = count_shooting_stars(false, true);
        let meteor_shower = count_shooting_stars(true, true);
        let ratio = meteor_shower as f32 / normal as f32;

        assert!((2.5..=3.5).contains(&ratio), "ratio {ratio}");
    }

    #[test]
    fn test_covered_sky_hides_every_shooting_star() {
        assert_eq!(count_shooting_stars(false, false), 0);
        assert_eq!(count_shooting_stars(true, false), 0);
    }
}
