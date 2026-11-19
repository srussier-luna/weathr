use crate::animation::duck::{PUDDLE_WIDTH, puddle_x};
use crate::animation::garden::{self, Bubble, GardenLayout};
use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;
use std::ops::Range;

const DOGHOUSE_OFFSET: u16 = 6;
const HOME_OFFSET: u16 = 16;
const DOG_WIDTH: u16 = 11;
const WALK_SPEED: f32 = 0.15;
const SLIDE_SPEED: f32 = 0.45;
const BATHING_CHANCE: f32 = 1.0 / 6.0;
const SITTING_FRAMES: Range<u16> = 90..300;
const SPLASHING_FRAMES: Range<u16> = 150..240;
const SHAKING_FRAMES: u16 = 90;
const BANK_GAP: u16 = 1;
const PAW_GAP: u16 = 5;
const DOGHOUSE_ROOF: [&str; 2] = [" _______ ", "/_______\\"];
const DOGHOUSE_WALLS: [&str; 2] = ["| .---. |", "|_|   |_|"];
const BODY_LEFT: [&str; 3] = ["  __       ", "_/o \\_____/", "@__/______)"];
const BODY_RIGHT: [&str; 3] = ["       __  ", "\\_____/ o\\_", "(______\\__@"];
const LEGS_LEFT: [&str; 2] = ["   ||   || ", "   /\\   /\\ "];
const LEGS_RIGHT: [&str; 2] = [" ||   ||   ", " /\\   /\\   "];
const LEGS_APART_LEFT: &str = "  //     \\\\";
const LEGS_APART_RIGHT: &str = "//     \\\\  ";
const RIPPLES_LEFT: [&str; 2] = ["  (~~) (~~)", "  ~~~~ ~~~~"];
const RIPPLES_RIGHT: [&str; 2] = ["(~~) (~~)  ", "~~~~ ~~~~  "];
const SPLASHES: [[&str; 4]; 2] = [
    [
        "  '           '  ",
        " .             . ",
        "'               '",
        "  .           .  ",
    ],
    [
        "'               '",
        "  '           '  ",
        " .             . ",
        ". '           ' .",
    ],
];
const DROPS: [[&str; 4]; 3] = [
    [
        "     '       '     ",
        "   '           '   ",
        "   .           .   ",
        "",
    ],
    [
        "   '           '   ",
        "  '             '  ",
        "",
        "  .             .  ",
    ],
    [
        " '               ' ",
        "",
        "'                 '",
        " .               . ",
    ],
];
const DOG_FUR: Color = Color::Rgb {
    r: 170,
    g: 110,
    b: 60,
};
const WATER: Color = Color::Rgb {
    r: 170,
    g: 210,
    b: 255,
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

#[derive(Clone, Copy)]
enum Bathing {
    Wading(f32),
    Splashing(u16),
    Leaving(f32),
    ShakingOff(u16),
}

enum Activity {
    Sitting(u16),
    Walking(f32),
    Bathing(Bathing),
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
        is_frozen: bool,
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
        let max_x = Self::max_x(layout);
        let x = *self.x.get_or_insert(home_x);

        match self.activity {
            Activity::Sitting(remaining) if remaining > 0 => {
                self.activity = Activity::Sitting(remaining - 1);
                if rng.random::<f32>() < 0.008 {
                    self.is_wagging = !self.is_wagging;
                }
            }
            Activity::Sitting(_) => {
                let is_bathing = rng.random::<f32>() < BATHING_CHANCE;
                let target = if is_bathing {
                    Self::bathing_x(layout, is_frozen)
                } else if rng.random::<f32>() < 0.3 || max_x <= 1.0 {
                    home_x
                } else {
                    rng.random_range(1.0..max_x)
                };
                self.is_facing_left = target < x;
                self.activity = if is_bathing {
                    Activity::Bathing(Bathing::Wading(target))
                } else {
                    Activity::Walking(target)
                };
            }
            Activity::Walking(target) => {
                if self.walk_to(x, target, WALK_SPEED) {
                    self.sit(rng);
                }
            }
            Activity::Bathing(bathing) => self.bathe(bathing, x, layout, is_frozen, rng),
        }

        self.woof.update(0.0015, 45, rng.random::<f32>());
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        conditions: &WeatherConditions,
        is_frozen: bool,
    ) -> io::Result<()> {
        let (doghouse_x, doghouse_y) = Self::doghouse(layout);
        self.render_doghouse(renderer, doghouse_x, doghouse_y, conditions.is_day)?;

        if Self::is_inside(conditions) {
            self.render_inside(renderer, doghouse_x, doghouse_y, conditions.is_day)
        } else {
            self.render_outside(renderer, layout, is_frozen)
        }
    }

    pub fn head(&self, layout: &GardenLayout, conditions: &WeatherConditions) -> (u16, u16) {
        if Self::is_inside(conditions) {
            let (x, y) = Self::doghouse(layout);
            return (x + 4, y);
        }
        let (x, y) = self.outside(layout);
        let head_column = if self.is_facing_left { 2 } else { 8 };
        (x + head_column, y)
    }

    fn bathe(
        &mut self,
        bathing: Bathing,
        x: f32,
        layout: &GardenLayout,
        is_frozen: bool,
        rng: &mut impl Rng,
    ) {
        let speed = if self.is_sliding(layout, is_frozen) {
            SLIDE_SPEED
        } else {
            WALK_SPEED
        };

        match bathing {
            Bathing::Wading(target) => {
                if self.walk_to(x, target, speed) {
                    if is_frozen {
                        self.leave_puddle(layout, target);
                    } else {
                        let splashing = rng.random_range(SPLASHING_FRAMES);
                        self.activity = Activity::Bathing(Bathing::Splashing(splashing));
                    }
                }
            }
            Bathing::Splashing(0) => self.leave_puddle(layout, x),
            Bathing::Splashing(remaining) => {
                self.activity = Activity::Bathing(Bathing::Splashing(remaining - 1));
            }
            Bathing::Leaving(target) => {
                if self.walk_to(x, target, speed) {
                    if is_frozen {
                        self.sit(rng);
                    } else {
                        self.activity = Activity::Bathing(Bathing::ShakingOff(SHAKING_FRAMES));
                    }
                }
            }
            Bathing::ShakingOff(0) => self.sit(rng),
            Bathing::ShakingOff(remaining) => {
                self.activity = Activity::Bathing(Bathing::ShakingOff(remaining - 1));
            }
        }
    }

    fn sit(&mut self, rng: &mut impl Rng) {
        self.activity = Activity::Sitting(rng.random_range(SITTING_FRAMES));
    }

    fn walk_to(&mut self, x: f32, target: f32, speed: f32) -> bool {
        let has_arrived = (target - x).abs() <= speed;
        self.x = Some(if has_arrived {
            target
        } else if target < x {
            x - speed
        } else {
            x + speed
        });
        has_arrived
    }

    fn leave_puddle(&mut self, layout: &GardenLayout, x: f32) {
        let bank = Self::bank_x(layout);
        self.is_facing_left = bank < x;
        self.activity = Activity::Bathing(Bathing::Leaving(bank));
    }

    fn is_moving(&self) -> bool {
        matches!(
            self.activity,
            Activity::Walking(_) | Activity::Bathing(Bathing::Wading(_) | Bathing::Leaving(_))
        )
    }

    fn is_sliding(&self, layout: &GardenLayout, is_frozen: bool) -> bool {
        is_frozen
            && matches!(
                self.activity,
                Activity::Bathing(Bathing::Wading(_) | Bathing::Leaving(_))
            )
            && self.paws_in_puddle(layout).next().is_some()
    }

    fn is_in_water(&self, layout: &GardenLayout, is_frozen: bool) -> bool {
        !is_frozen && self.paws_in_puddle(layout).next().is_some()
    }

    fn paws_in_puddle(&self, layout: &GardenLayout) -> impl Iterator<Item = u16> {
        let (x, _) = self.outside(layout);
        let start = puddle_x(layout.width);
        let puddle = start..start + PUDDLE_WIDTH;
        let first_paw = if self.is_facing_left { 3 } else { 1 };
        [x + first_paw, x + first_paw + PAW_GAP]
            .into_iter()
            .filter(move |paw| puddle.contains(paw))
    }

    fn splashing_paws(&self, layout: &GardenLayout, is_frozen: bool) -> impl Iterator<Item = u16> {
        (!is_frozen && self.is_moving())
            .then(|| self.paws_in_puddle(layout))
            .into_iter()
            .flatten()
    }

    fn doghouse(layout: &GardenLayout) -> (u16, u16) {
        (
            layout.house_x + layout.house.porch_x() + DOGHOUSE_OFFSET,
            layout.horizon_y.saturating_sub(4),
        )
    }

    fn outside(&self, layout: &GardenLayout) -> (u16, u16) {
        let x = self.x.map_or(Self::home_x(layout), |x| x.round() as u16);
        (x, layout.horizon_y.saturating_sub(3))
    }

    fn home_x(layout: &GardenLayout) -> u16 {
        layout.house_x + layout.house.porch_x() + HOME_OFFSET
    }

    fn max_x(layout: &GardenLayout) -> f32 {
        layout.width.saturating_sub(DOG_WIDTH + 1) as f32
    }

    fn bathing_x(layout: &GardenLayout, is_frozen: bool) -> f32 {
        let start = puddle_x(layout.width);
        let spot = if is_frozen {
            start + PUDDLE_WIDTH - DOG_WIDTH
        } else {
            start + (PUDDLE_WIDTH - DOG_WIDTH) / 2
        };
        (spot as f32).min(Self::max_x(layout)).max(1.0)
    }

    fn bank_x(layout: &GardenLayout) -> f32 {
        let bank = puddle_x(layout.width).saturating_sub(DOG_WIDTH + BANK_GAP);
        (bank as f32).min(Self::max_x(layout)).max(1.0)
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

    fn render_outside(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        is_frozen: bool,
    ) -> io::Result<()> {
        let (x, y) = self.outside(layout);
        let is_moving = self.is_moving();
        let stride = usize::from(is_moving && !self.is_staring && (self.frame / 5) % 2 == 1);
        let is_tail_down =
            !is_moving && !self.is_staring && self.is_wagging && (self.frame / 5) % 2 == 0;

        let (body, nose_x, tail_x, tail, bubble_x) = if self.is_facing_left {
            let tail = if is_tail_down { '-' } else { '/' };
            (&BODY_LEFT, x, x + 10, tail, x.saturating_sub(1))
        } else {
            let tail = if is_tail_down { '-' } else { '\\' };
            (&BODY_RIGHT, x + 10, x, tail, x + 6)
        };
        let (legs, legs_color) = self.legs(layout, is_frozen, stride);

        garden::draw_solid(renderer, x, y, body, DOG_FUR)?;
        garden::draw(renderer, x, y + 3, &[legs], legs_color)?;
        renderer.render_char(nose_x, y + 2, '@', Color::DarkGrey)?;
        renderer.render_char(tail_x, y + 1, tail, DOG_FUR)?;

        let (left_mark, right_mark) = if stride == 0 {
            ('\'', '.')
        } else {
            ('.', '\'')
        };
        for paw in self.splashing_paws(layout, is_frozen) {
            renderer.render_char(paw.saturating_sub(1), y + 3, left_mark, WATER)?;
            renderer.render_char(paw + 2, y + 3, right_mark, WATER)?;
        }
        self.render_spray(renderer, layout, x, y, is_frozen)?;

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

    fn legs(&self, layout: &GardenLayout, is_frozen: bool, stride: usize) -> (&'static str, Color) {
        let (legs, apart, ripples) = if self.is_facing_left {
            (&LEGS_LEFT, LEGS_APART_LEFT, &RIPPLES_LEFT)
        } else {
            (&LEGS_RIGHT, LEGS_APART_RIGHT, &RIPPLES_RIGHT)
        };
        match self.activity {
            Activity::Bathing(Bathing::Splashing(remaining))
                if self.is_in_water(layout, is_frozen) =>
            {
                (ripples[usize::from(remaining / 8 % 2)], WATER)
            }
            _ if self.is_sliding(layout, is_frozen) => (apart, DOG_FUR),
            _ => (legs[stride], DOG_FUR),
        }
    }

    fn render_spray(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &GardenLayout,
        x: u16,
        y: u16,
        is_frozen: bool,
    ) -> io::Result<()> {
        match self.activity {
            Activity::Bathing(Bathing::Splashing(remaining))
                if self.is_in_water(layout, is_frozen) =>
            {
                let splashes = &SPLASHES[usize::from(remaining / 6 % 2)];
                garden::draw_clipped(renderer, i32::from(x) - 3, y, splashes, WATER)
            }
            Activity::Bathing(Bathing::ShakingOff(remaining)) => {
                let burst = usize::from(SHAKING_FRAMES.saturating_sub(remaining) / 4) % DROPS.len();
                garden::draw_clipped(
                    renderer,
                    i32::from(x) - 4,
                    y.saturating_sub(1),
                    &DROPS[burst],
                    WATER,
                )
            }
            _ => Ok(()),
        }
    }
}

impl Default for Dog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::tree::Growth;

    fn sunny_day() -> WeatherConditions {
        WeatherConditions {
            is_day: true,
            ..WeatherConditions::default()
        }
    }

    fn garden() -> GardenLayout {
        GardenLayout::new(120, 30, Growth::Full)
    }

    fn update_until(
        dog: &mut Dog,
        layout: &GardenLayout,
        is_frozen: bool,
        rng: &mut StdRng,
        is_done: impl Fn(&Dog) -> bool,
    ) {
        for _ in 0..10_000 {
            if is_done(dog) {
                return;
            }
            dog.update(layout, &sunny_day(), is_frozen, rng);
        }
        panic!("the dog never got there");
    }

    #[test]
    fn test_dog_goes_bathing_about_one_walk_in_six() {
        let layout = garden();
        let mut rng = StdRng::seed_from_u64(69);
        let mut dog = Dog::new();
        let baths = (0..600)
            .filter(|_| {
                dog.activity = Activity::Sitting(0);
                dog.update(&layout, &sunny_day(), false, &mut rng);
                matches!(dog.activity, Activity::Bathing(Bathing::Wading(_)))
            })
            .count();
        assert!((70..=130).contains(&baths), "{baths} baths in 600 walks");
    }

    #[test]
    fn test_bathing_dog_splashes_in_the_puddle_then_shakes_off_on_the_bank() {
        let layout = garden();
        let mut rng = StdRng::seed_from_u64(69);
        let mut dog = Dog::new();
        dog.activity = Activity::Bathing(Bathing::Wading(Dog::bathing_x(&layout, false)));

        update_until(&mut dog, &layout, false, &mut rng, |dog| {
            matches!(dog.activity, Activity::Bathing(Bathing::Splashing(_)))
        });
        assert_eq!(dog.paws_in_puddle(&layout).count(), 2);
        let (legs, color) = dog.legs(&layout, false, 0);
        assert!(!legs.contains('|'));
        assert_eq!(color, WATER);

        update_until(&mut dog, &layout, false, &mut rng, |dog| {
            matches!(dog.activity, Activity::Bathing(Bathing::ShakingOff(_)))
        });
        assert_eq!(dog.paws_in_puddle(&layout).count(), 0);

        update_until(&mut dog, &layout, false, &mut rng, |dog| {
            matches!(dog.activity, Activity::Sitting(_))
        });
    }

    #[test]
    fn test_bathing_turns_into_a_fast_slide_without_drops_on_a_frozen_puddle() {
        let layout = garden();
        let mut rng = StdRng::seed_from_u64(69);
        let mut dog = Dog::new();
        dog.x = Some(Dog::bank_x(&layout));
        dog.is_facing_left = false;
        dog.activity = Activity::Bathing(Bathing::Wading(Dog::bathing_x(&layout, true)));

        update_until(&mut dog, &layout, true, &mut rng, |dog| {
            dog.is_sliding(&layout, true)
        });
        let before = dog.x.unwrap();
        dog.update(&layout, &sunny_day(), true, &mut rng);
        assert!((dog.x.unwrap() - before - SLIDE_SPEED).abs() < 1e-4);
        assert_eq!(dog.legs(&layout, true, 1), (LEGS_APART_RIGHT, DOG_FUR));

        for _ in 0..10_000 {
            dog.update(&layout, &sunny_day(), true, &mut rng);
            assert!(!matches!(
                dog.activity,
                Activity::Bathing(Bathing::Splashing(_) | Bathing::ShakingOff(_))
            ));
            assert_eq!(dog.splashing_paws(&layout, true).count(), 0);
            if matches!(dog.activity, Activity::Sitting(_)) {
                return;
            }
        }
        panic!("the dog never left the ice");
    }

    #[test]
    fn test_walking_dog_splashes_only_while_over_liquid_water() {
        let layout = garden();
        let mut dog = Dog::new();
        dog.activity = Activity::Walking(1.0);

        dog.x = Some(Dog::bathing_x(&layout, false));
        assert_eq!(dog.splashing_paws(&layout, false).count(), 2);
        assert_eq!(dog.splashing_paws(&layout, true).count(), 0);

        dog.x = Some(Dog::bank_x(&layout));
        assert_eq!(dog.splashing_paws(&layout, false).count(), 0);

        dog.x = Some(Dog::bathing_x(&layout, false));
        dog.activity = Activity::Sitting(10);
        assert_eq!(dog.splashing_paws(&layout, false).count(), 0);
    }

    #[test]
    fn test_rain_sends_a_bathing_dog_back_to_the_doghouse() {
        let layout = garden();
        let mut rng = StdRng::seed_from_u64(69);
        let mut dog = Dog::new();
        dog.x = Some(Dog::bathing_x(&layout, false));
        dog.activity = Activity::Bathing(Bathing::Splashing(100));
        let rain = WeatherConditions {
            is_raining: true,
            ..sunny_day()
        };

        dog.update(&layout, &rain, false, &mut rng);

        assert!(dog.x.is_none());
        assert!(matches!(dog.activity, Activity::Sitting(_)));
    }
}
