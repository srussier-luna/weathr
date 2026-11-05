use crate::render::TerminalRenderer;
use crate::weather::WeatherCondition;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;
use std::sync::OnceLock;

static CLOUD_SHAPES: OnceLock<Vec<Vec<String>>> = OnceLock::new();

struct Cloud {
    x: f32,
    y: f32,
    speed: f32,
    shape: Vec<String>,
    color: Color,
}

pub struct CloudSystem {
    clouds: Vec<Cloud>,
    terminal_width: u16,
    terminal_height: u16,
}

pub fn cloud_color(condition: Option<WeatherCondition>) -> Color {
    match condition {
        Some(WeatherCondition::Clear) => Color::White,
        Some(WeatherCondition::PartlyCloudy) => Color::Grey,
        _ => Color::DarkGrey,
    }
}

fn visible_part(line: &str, x: f32) -> Option<(u16, &str)> {
    let outline = line.trim();
    let leading = line.chars().take_while(|c| c.is_whitespace()).count();
    let column = x.floor() as i16 + leading as i16;
    if column >= 0 {
        return Some((column as u16, outline));
    }
    outline
        .char_indices()
        .nth(column.unsigned_abs() as usize)
        .map(|(start, _)| (0, &outline[start..]))
}

impl CloudSystem {
    pub fn new(terminal_width: u16, terminal_height: u16) -> Self {
        let mut rng = rand::rng();
        // Add a few initial clouds
        let count = std::cmp::max(1, terminal_width / 20);

        let max_capacity = (terminal_width / 20) as usize;
        let mut clouds = Vec::with_capacity(max_capacity);

        for _ in 0..count {
            clouds.push(Self::create_random_cloud(
                terminal_width,
                terminal_height,
                true,
                Color::White,
                &mut rng,
            ));
        }

        Self {
            clouds,
            terminal_width,
            terminal_height,
        }
    }

    fn create_random_cloud(
        width: u16,
        height: u16,
        random_x: bool,
        color: Color,
        rng: &mut impl Rng,
    ) -> Cloud {
        let shapes = CLOUD_SHAPES.get_or_init(Self::create_cloud_shapes);

        let shape_idx = (rng.random::<u32>() as usize) % shapes.len();
        let shape = shapes[shape_idx].clone();

        let y_range = height / 3;
        let y = (rng.random::<u16>() % std::cmp::max(1, y_range)) as f32;

        let x = if random_x {
            (rng.random::<u16>() % width) as f32
        } else {
            -(shape[0].len() as f32)
        };

        let speed = 0.05 + (rng.random::<f32>() * 0.1);

        Cloud {
            x,
            y,
            speed,
            shape,
            color,
        }
    }

    fn create_cloud_shapes() -> Vec<Vec<String>> {
        let shapes = [
            vec![
                "   .--.   ".to_string(),
                " .-(    ). ".to_string(),
                "(___.__)_)".to_string(),
            ],
            vec![
                "      _  _   ".to_string(),
                "    ( `   )_ ".to_string(),
                "   (    )    `)".to_string(),
                "    \\_  (___  )".to_string(),
            ],
            vec![
                "     .--.    ".to_string(),
                "  .-(    ).  ".to_string(),
                " (___.__)__) ".to_string(),
            ],
            vec![
                "   _  _   ".to_string(),
                "  ( `   )_ ".to_string(),
                " (    )   `)".to_string(),
                "  `--'     ".to_string(),
            ],
        ];

        shapes.to_vec()
    }

    pub fn update(
        &mut self,
        terminal_width: u16,
        terminal_height: u16,
        is_clear: bool,
        cloud_color: Color,
        rng: &mut impl Rng,
    ) {
        self.terminal_width = terminal_width;
        self.terminal_height = terminal_height;

        for cloud in &mut self.clouds {
            cloud.x += cloud.speed;
            cloud.color = cloud_color;
        }

        self.clouds.retain(|c| c.x < terminal_width as f32);

        let max_clouds = if is_clear {
            (terminal_width / 40) as usize
        } else {
            (terminal_width / 20) as usize
        };

        let spawn_chance = if is_clear { 0.002 } else { 0.005 };

        if self.clouds.len() < max_clouds && rng.random::<f32>() < spawn_chance {
            self.clouds.push(Self::create_random_cloud(
                terminal_width,
                terminal_height,
                false,
                cloud_color,
                rng,
            ));
        }
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        for cloud in &self.clouds {
            for (i, line) in cloud.shape.iter().enumerate() {
                let y = cloud.y as i16 + i as i16;

                if y >= 0
                    && y < self.terminal_height as i16
                    && let Some((x, visible)) = visible_part(line, cloud.x)
                {
                    renderer.render_line_colored(x, y as u16, visible, cloud.color)?;
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
    const HEIGHT: u16 = 40;
    const FRAMES: usize = 20_000;

    #[test]
    fn test_cloud_at_negative_x_shows_only_its_visible_part() {
        let line = "(___.__)_)";

        assert_eq!(visible_part(line, -4.0), Some((0, ".__)_)")));
        assert_eq!(visible_part(line, -3.6), Some((0, ".__)_)")));
        assert_eq!(visible_part(line, -0.5), Some((0, "___.__)_)")));
        assert_eq!(visible_part(line, 3.2), Some((3, line)));
        assert_eq!(visible_part(line, -10.0), None);
        assert_eq!(visible_part("é.--.", -1.0), Some((0, ".--.")));
        assert_eq!(visible_part("   (  )   ", 2.0), Some((5, "(  )")));
        assert_eq!(visible_part("   (  )   ", -4.0), Some((0, "  )")));
        assert_eq!(visible_part("   (  )   ", -7.0), None);
    }

    fn paint(row: &mut [char], line: &str, x: f32) {
        if let Some((column, part)) = visible_part(line, x) {
            for (offset, ch) in part.chars().enumerate() {
                row[usize::from(column) + offset] = ch;
            }
        }
    }

    #[test]
    fn test_front_cloud_hides_the_back_one_only_inside_its_outline() {
        let mut row = vec![' '; 30];

        paint(&mut row, "(___.__)_)", 10.0);
        paint(&mut row, "   (  )   ", 5.0);
        paint(&mut row, "  .--.  ", 16.0);

        assert_eq!(
            row.iter().collect::<String>().trim_end(),
            "        (  )__.__).--."
        );
    }

    #[test]
    fn test_new_cloud_has_the_color_of_the_clouds_already_there() {
        let skies = [
            (None, Color::DarkGrey),
            (Some(WeatherCondition::Clear), Color::White),
            (Some(WeatherCondition::PartlyCloudy), Color::Grey),
            (Some(WeatherCondition::Cloudy), Color::DarkGrey),
            (Some(WeatherCondition::Overcast), Color::DarkGrey),
        ];

        for (condition, expected) in skies {
            assert_eq!(cloud_color(condition), expected);

            let is_clear = condition == Some(WeatherCondition::Clear);
            let mut rng = StdRng::seed_from_u64(104);
            let mut clouds = CloudSystem::new(WIDTH, HEIGHT);
            let mut saw_newborn = false;

            for _ in 0..FRAMES {
                clouds.update(WIDTH, HEIGHT, is_clear, cloud_color(condition), &mut rng);
                assert!(clouds.clouds.iter().all(|cloud| cloud.color == expected));
                saw_newborn |= clouds.clouds.iter().any(|cloud| cloud.x < 0.0);
            }

            assert!(saw_newborn);
        }
    }
}
