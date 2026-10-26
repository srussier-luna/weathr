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

impl ShootingStar {
    fn spawn(terminal_width: u16, terminal_height: u16, rng: &mut impl Rng) -> Self {
        let start_x = (rng.random::<u16>() % (terminal_width / 2)) + (terminal_width / 4);
        let start_y = rng.random::<u16>() % (terminal_height / 4);

        Self {
            x: start_x as f32,
            y: start_y as f32,
            speed_x: if rng.random::<bool>() { 0.9 } else { -0.9 },
            speed_y: 0.3 + (rng.random::<f32>() * 0.2),
            length: 9,
            active: true,
        }
    }

    fn is_gone(&self, terminal_height: u16) -> bool {
        self.x < 0.0 || self.y as u16 >= terminal_height || self.length == 0
    }
}

pub struct StarSystem {
    stars: Vec<Star>,
    shooting_stars: Vec<ShootingStar>,
    is_meteor_shower: bool,
    is_sky_clear: bool,
    terminal_width: u16,
    terminal_height: u16,
}

impl StarSystem {
    const MIN_DISTANCE: f32 = 3.0; // Minimum distance between stars
    const SHOOTING_STAR_CHANCE: f32 = 0.005;
    const METEOR_SHOWER_CHANCE: f32 = 0.3;
    const METEOR_SHOWER_SIZE: usize = 16;

    pub fn new(terminal_width: u16, terminal_height: u16) -> Self {
        let stars = Self::create_stars(terminal_width, terminal_height, &[]);

        Self {
            stars,
            shooting_stars: Vec::new(),
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
            self.shooting_stars.clear();
        }
    }

    fn spawn_rule(&self) -> (usize, f32) {
        if !self.is_sky_clear {
            (0, 0.0)
        } else if self.is_meteor_shower {
            (Self::METEOR_SHOWER_SIZE, Self::METEOR_SHOWER_CHANCE)
        } else {
            (1, Self::SHOOTING_STAR_CHANCE)
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
        let (limit, chance) = self.spawn_rule();
        self.shooting_stars.truncate(limit);
        let free_slots = limit - self.shooting_stars.len();

        for star in &mut self.shooting_stars {
            star.x += star.speed_x;
            star.y += star.speed_y;
        }
        self.shooting_stars
            .retain(|star| !star.is_gone(terminal_height));

        for _ in 0..free_slots {
            if rng.random::<f32>() < chance {
                self.shooting_stars
                    .push(ShootingStar::spawn(terminal_width, terminal_height, rng));
            }
        }
    }

    pub fn has_shooting_star(&self) -> bool {
        self.shooting_stars.iter().any(|star| star.active)
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

        for star in self.shooting_stars.iter().filter(|star| star.active) {
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
    const FRAMES: usize = 30_000;

    fn shooting_stars_on_screen(stars: &StarSystem) -> usize {
        stars
            .shooting_stars
            .iter()
            .filter(|star| star.x < WIDTH as f32 && star.y < HEIGHT as f32)
            .count()
    }

    fn sky(is_meteor_shower: bool) -> StarSystem {
        let mut stars = StarSystem::new(WIDTH, HEIGHT);
        stars.set_meteor_shower(is_meteor_shower);
        stars
    }

    #[test]
    fn test_meteor_shower_fills_the_sky_within_half_a_second() {
        let mut rng = StdRng::seed_from_u64(15);
        let mut stars = sky(true);

        for _ in 0..15 {
            stars.update(WIDTH, HEIGHT, &mut rng);
        }

        assert!(shooting_stars_on_screen(&stars) > 5);
    }

    #[test]
    fn test_meteor_shower_keeps_many_shooting_stars_at_once() {
        let mut rng = StdRng::seed_from_u64(16);
        let mut stars = sky(true);
        let mut crowded = 0;

        for _ in 0..FRAMES {
            stars.update(WIDTH, HEIGHT, &mut rng);
            if shooting_stars_on_screen(&stars) > 5 {
                crowded += 1;
            }
        }

        assert!(crowded > FRAMES * 9 / 10, "{crowded} of {FRAMES} frames");
    }

    #[test]
    fn test_outside_meteor_shower_one_shooting_star_at_most() {
        let mut rng = StdRng::seed_from_u64(17);
        let mut stars = sky(false);
        let mut frames_with_star = 0;

        for _ in 0..FRAMES {
            stars.update(WIDTH, HEIGHT, &mut rng);
            assert!(stars.shooting_stars.len() <= 1);
            if stars.has_shooting_star() {
                frames_with_star += 1;
            }
        }

        assert!(frames_with_star > 0);
    }

    #[test]
    fn test_meteor_shower_end_leaves_one_shooting_star_at_most() {
        let mut rng = StdRng::seed_from_u64(18);
        let mut stars = sky(true);
        for _ in 0..30 {
            stars.update(WIDTH, HEIGHT, &mut rng);
        }

        stars.set_meteor_shower(false);
        stars.update(WIDTH, HEIGHT, &mut rng);

        assert!(stars.shooting_stars.len() <= 1);
    }

    #[test]
    fn test_covered_sky_hides_every_shooting_star() {
        for is_meteor_shower in [false, true] {
            let mut rng = StdRng::seed_from_u64(19);
            let mut stars = sky(is_meteor_shower);
            for _ in 0..30 {
                stars.update(WIDTH, HEIGHT, &mut rng);
            }

            stars.set_sky_clear(false);
            assert!(stars.shooting_stars.is_empty());

            for _ in 0..FRAMES {
                stars.update(WIDTH, HEIGHT, &mut rng);
                assert!(!stars.has_shooting_star());
            }
        }
    }
}
