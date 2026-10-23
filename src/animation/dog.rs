use crate::animation::garden::{self, Bubble, GardenLayout};
use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

const DOGHOUSE_OFFSET: u16 = 6;
const HOME_OFFSET: u16 = 16;
const DOG_WIDTH: u16 = 11;
const WALK_SPEED: f32 = 0.15;
const DOGHOUSE_ROOF: [&str; 2] = [" _______ ", "/_______\\"];
const DOGHOUSE_WALLS: [&str; 2] = ["| .---. |", "|_|   |_|"];
const BODY_LEFT: [&str; 3] = ["  __       ", "_/o \\_____/", "@__/______)"];
const BODY_RIGHT: [&str; 3] = ["       __  ", "\\_____/ o\\_", "(______\\__@"];
const LEGS_LEFT: [&str; 2] = ["   ||   || ", "   /\\   /\\ "];
const LEGS_RIGHT: [&str; 2] = [" ||   ||   ", " /\\   /\\   "];
const DOG_FUR: Color = Color::Rgb {
    r: 170,
    g: 110,
    b: 60,
};
const DAY_WOOD: Color = Color::Rgb {
    r: 210,
    g: 180,
    b: 140,
};
const NIGHT_WOOD: Color = Color::Rgb {
    r: 100,
    g: 70,
    b: 50,
};
const DOOR_COLOR: Color = Color::Rgb {
    r: 139,
    g: 69,
    b: 19,
};

enum Activity {
    Sitting(u16),
    Walking(f32),
}

pub struct Dog {
    frame: u32,
    x: Option<f32>,
    is_facing_left: bool,
    activity: Activity,
    is_wagging: bool,
    is_staring: bool,
    woof: Bubble,
}

impl Dog {
    pub fn new() -> Self {
        Self {
            frame: 0,
            x: None,
            is_facing_left: true,
            activity: Activity::Sitting(120),
            is_wagging: true,
            is_staring: false,
            woof: Bubble::new(),
        }
    }

    pub fn set_staring(&mut self, is_staring: bool) {
        self.is_staring = is_staring;
    }

    pub fn update(
        &mut self,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
        rng: &mut impl Rng,
    ) {
        self.frame = self.frame.wrapping_add(1);

        if Self::is_inside(conditions) {
            self.woof.hide();
            self.x = None;
            self.activity = Activity::Sitting(120);
            return;
        }

        if self.is_staring {
            self.woof.hide();
            return;
        }

        let home_x = Self::home_x(layout) as f32;
        let max_x = layout.width.saturating_sub(DOG_WIDTH + 1) as f32;
        let x = *self.x.get_or_insert(home_x);

        match self.activity {
            Activity::Sitting(remaining) if remaining > 0 => {
                self.activity = Activity::Sitting(remaining - 1);
                if rng.random::<f32>() < 0.008 {
                    self.is_wagging = !self.is_wagging;
                }
            }
            Activity::Sitting(_) => {
                let target = if rng.random::<f32>() < 0.3 || max_x <= 1.0 {
                    home_x
                } else {
                    rng.random_range(1.0..max_x)
                };
                self.is_facing_left = target < x;
                self.activity = Activity::Walking(target);
            }
            Activity::Walking(target) => {
                if (target - x).abs() <= WALK_SPEED {
                    self.x = Some(target);
                    self.activity = Activity::Sitting(rng.random_range(90..300));
                } else {
                    let step = if target < x { -WALK_SPEED } else { WALK_SPEED };
                    self.x = Some(x + step);
                }
            }
        }

        self.woof.update(0.0015, 45, rng.random::<f32>());
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
    ) -> io::Result<()> {
        let doghouse_x = layout.house_x + layout.house.porch_x() + DOGHOUSE_OFFSET;
        let doghouse_y = layout.horizon_y.saturating_sub(4);
        self.render_doghouse(renderer, doghouse_x, doghouse_y, conditions.is_day)?;

        if Self::is_inside(conditions) {
            self.render_inside(renderer, doghouse_x, doghouse_y, conditions.is_day)
        } else {
            let x = self.x.map_or(Self::home_x(layout), |x| x.round() as u16);
            self.render_outside(renderer, x, layout.horizon_y.saturating_sub(3))
        }
    }

    fn home_x(layout: &GardenLayout) -> u16 {
        layout.house_x + layout.house.porch_x() + HOME_OFFSET
    }

    fn is_inside(conditions: &WeatherConditions) -> bool {
        conditions.is_raining || conditions.is_thunderstorm || !conditions.is_day
    }

    fn render_doghouse(
        &self,
        renderer: &mut TerminalRenderer,
        x: u16,
        y: u16,
        is_day: bool,
    ) -> io::Result<()> {
        let (roof, wood) = if is_day {
            (Color::DarkRed, DAY_WOOD)
        } else {
            (Color::DarkMagenta, NIGHT_WOOD)
        };

        garden::draw_solid(renderer, x, y, &DOGHOUSE_ROOF, roof)?;
        garden::draw_solid(renderer, x, y + 2, &DOGHOUSE_WALLS, wood)?;
        garden::draw(renderer, x + 2, y + 2, &[".---."], DOOR_COLOR)
    }

    fn render_inside(
        &self,
        renderer: &mut TerminalRenderer,
        x: u16,
        y: u16,
        is_awake: bool,
    ) -> io::Result<()> {
        let face = if is_awake { "o.o" } else { "-.-" };
        garden::draw(renderer, x + 3, y + 3, &[face], DOG_FUR)?;

        if !is_awake {
            let phase = (self.frame / 20) % 3;
            if phase >= 1 {
                renderer.render_char(x + 9, y.saturating_sub(1), 'z', Color::White)?;
            }
            if phase >= 2 {
                renderer.render_char(x + 10, y.saturating_sub(2), 'Z', Color::White)?;
            }
        }
        Ok(())
    }

    fn render_outside(&self, renderer: &mut TerminalRenderer, x: u16, y: u16) -> io::Result<()> {
        let is_walking = matches!(self.activity, Activity::Walking(_));
        let stride = usize::from(is_walking && !self.is_staring && (self.frame / 5) % 2 == 1);
        let is_tail_down =
            !is_walking && !self.is_staring && self.is_wagging && (self.frame / 5) % 2 == 0;

        let (body, legs, nose_x, tail_x, tail, bubble_x) = if self.is_facing_left {
            let tail = if is_tail_down { '-' } else { '/' };
            (
                &BODY_LEFT,
                LEGS_LEFT[stride],
                x,
                x + 10,
                tail,
                x.saturating_sub(1),
            )
        } else {
            let tail = if is_tail_down { '-' } else { '\\' };
            (&BODY_RIGHT, LEGS_RIGHT[stride], x + 10, x, tail, x + 6)
        };

        garden::draw_solid(renderer, x, y, body, DOG_FUR)?;
        garden::draw(renderer, x, y + 3, &[legs], DOG_FUR)?;
        renderer.render_char(nose_x, y + 2, '@', Color::DarkGrey)?;
        renderer.render_char(tail_x, y + 1, tail, DOG_FUR)?;

        if self.woof.is_visible() {
            garden::draw_solid(
                renderer,
                bubble_x,
                y.saturating_sub(1),
                &["Wouf!"],
                Color::White,
            )?;
        }
        Ok(())
    }
}

impl Default for Dog {
    fn default() -> Self {
        Self::new()
    }
}
