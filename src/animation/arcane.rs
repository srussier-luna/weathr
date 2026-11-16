use crate::animation::garden::{self, GardenLayout};
use crate::animation::postman::{ride_time, round_due};
use crate::render::TerminalRenderer;
use crate::scene::decorations::{ARCANE_VIOLET, VIOLET_MAILBOX_WIDTH, violet_mailbox_at};
use chrono::{Local, NaiveDate, NaiveDateTime};
use crossterm::style::Color;
use std::io;

const SPEED: f32 = 0.5;
const CAST_FRAMES: u16 = 90;
const MAX_CAST_FRAMES: u16 = 3 * CAST_FRAMES;
const MAGE: [&str; 4] = ["      /*\\", "     /___\\", "     (o.o)", "     \\vvv/"];
const BROOM: &str = "}}}==     ====-";
const WAND: [&str; 3] = ["            *", "           /", "          /"];
const SPRITE_WIDTH: f32 = 15.0;
const BEARD_COLUMN: u16 = 7;
const SPARKLE_ROWS: u16 = 2;
const SPARKLE_MARGIN: u16 = 2;
const SPARKLE_CHANCE: f32 = 0.35;
const SPARKLE_FRAMES: u16 = 5;
const SPARKLES: [char; 4] = ['*', '+', '.', '\''];
const BROOM_BROWN: Color = Color::Rgb {
    r: 150,
    g: 100,
    b: 50,
};
const SPARKLE_GLOW: Color = Color::Rgb {
    r: 225,
    g: 190,
    b: 255,
};
const TURQUOISE: Color = Color::Rgb {
    r: 64,
    g: 224,
    b: 208,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delivery {
    Nothing,
    Due,
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArcaneRound {
    pub hour: u32,
    pub delivery: Delivery,
    pub is_arcane_waiting: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Flight {
    Waiting,
    Arriving,
    Casting(u16),
    Leaving,
}

pub struct ArcanoPostman {
    flight: Flight,
    x: f32,
    round: Option<ArcaneRound>,
    last_round: Option<NaiveDate>,
}

fn violet_mailbox(layout: &GardenLayout) -> Option<(u16, u16)> {
    let mailbox_x = layout.plan.mailbox_x?;
    Some(violet_mailbox_at(mailbox_x, layout.horizon_y))
}

fn stop_x(mailbox_x: u16) -> f32 {
    (f32::from(mailbox_x + VIOLET_MAILBOX_WIDTH / 2) - f32::from(BEARD_COLUMN)).max(0.0)
}

fn altitude(mailbox_y: u16) -> u16 {
    mailbox_y.saturating_sub(SPARKLE_ROWS + MAGE.len() as u16)
}

impl ArcanoPostman {
    pub fn new() -> Self {
        Self {
            flight: Flight::Waiting,
            x: -SPRITE_WIDTH,
            round: None,
            last_round: None,
        }
    }

    pub fn set_round(&mut self, round: Option<ArcaneRound>) {
        self.round = round;
    }

    fn hides_flag(&self, delivery: Delivery) -> bool {
        match self.flight {
            Flight::Casting(_) => true,
            Flight::Arriving => delivery != Delivery::Nothing,
            Flight::Waiting | Flight::Leaving => false,
        }
    }

    pub fn update(&mut self, layout: &GardenLayout) {
        self.fly(layout, Local::now().naive_local());
    }

    fn fly(&mut self, layout: &GardenLayout, now: NaiveDateTime) {
        let (Some(round), Some((mailbox_x, _))) = (self.round, violet_mailbox(layout)) else {
            self.flight = Flight::Waiting;
            return;
        };
        let stop_x = stop_x(mailbox_x);

        self.flight = match self.flight {
            Flight::Waiting => {
                let ride = ride_time(stop_x + SPRITE_WIDTH, SPEED);
                match round_due(round.hour, self.last_round, ride, now) {
                    Some(date) => {
                        self.last_round = Some(date);
                        self.x = -SPRITE_WIDTH;
                        Flight::Arriving
                    }
                    None => Flight::Waiting,
                }
            }
            Flight::Arriving if self.x + SPEED < stop_x => {
                self.x += SPEED;
                Flight::Arriving
            }
            Flight::Arriving if round.delivery == Delivery::Nothing => {
                self.x += SPEED;
                Flight::Leaving
            }
            Flight::Arriving => {
                self.x = stop_x;
                Flight::Casting(0)
            }
            Flight::Casting(elapsed)
                if elapsed >= MAX_CAST_FRAMES
                    || (elapsed >= CAST_FRAMES && round.delivery == Delivery::Done) =>
            {
                Flight::Leaving
            }
            Flight::Casting(elapsed) => Flight::Casting(elapsed + 1),
            Flight::Leaving if self.x <= f32::from(layout.width) => {
                self.x += SPEED;
                Flight::Leaving
            }
            Flight::Leaving => Flight::Waiting,
        };
    }

    pub fn render(&self, renderer: &mut TerminalRenderer, layout: &GardenLayout) -> io::Result<()> {
        let (Some(round), Some((mailbox_x, mailbox_y))) = (self.round, violet_mailbox(layout))
        else {
            return Ok(());
        };

        if round.is_arcane_waiting && !self.hides_flag(round.delivery) {
            let flag_x = mailbox_x + VIOLET_MAILBOX_WIDTH;
            renderer.render_char(flag_x, mailbox_y, '>', TURQUOISE)?;
            renderer.render_char(flag_x, mailbox_y + 1, '|', TURQUOISE)?;
        }

        if self.flight == Flight::Waiting {
            return Ok(());
        }

        let x = self.x.round() as i32;
        let y = altitude(mailbox_y);
        let broom_y = y + MAGE.len() as u16 - 1;
        garden::draw_clipped(renderer, x, broom_y, &[BROOM], BROOM_BROWN)?;
        garden::draw_clipped(renderer, x, y, &MAGE, ARCANE_VIOLET)?;

        if let Flight::Casting(elapsed) = self.flight {
            garden::draw_clipped(renderer, x, y, &WAND, ARCANE_VIOLET)?;
            render_sparkles(renderer, mailbox_x, mailbox_y, elapsed)?;
        }
        Ok(())
    }
}

fn render_sparkles(
    renderer: &mut TerminalRenderer,
    mailbox_x: u16,
    mailbox_y: u16,
    elapsed: u16,
) -> io::Result<()> {
    let seed = u32::from(elapsed / SPARKLE_FRAMES);
    let first_column = mailbox_x.saturating_sub(SPARKLE_MARGIN);
    let last_column = mailbox_x + VIOLET_MAILBOX_WIDTH + SPARKLE_MARGIN;
    for row in mailbox_y.saturating_sub(SPARKLE_ROWS)..mailbox_y {
        for column in first_column..last_column {
            let roll = garden::scatter(column.wrapping_add(row.wrapping_mul(last_column)), seed);
            if roll < SPARKLE_CHANCE {
                let sparkle = SPARKLES[(roll * 100.0) as usize % SPARKLES.len()];
                let color = if roll < SPARKLE_CHANCE / 2.0 {
                    ARCANE_VIOLET
                } else {
                    SPARKLE_GLOW
                };
                renderer.render_char(column, row, sparkle, color)?;
            }
        }
    }
    Ok(())
}

impl Default for ArcanoPostman {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::tree::Growth;
    use chrono::Duration;

    fn layout() -> GardenLayout {
        GardenLayout::new(80, 24, Growth::Full)
    }

    fn round(delivery: Delivery) -> Option<ArcaneRound> {
        Some(ArcaneRound {
            hour: 21,
            delivery,
            is_arcane_waiting: false,
        })
    }

    fn evening(hour: u32, minute: u32, second: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2026, 11, 20)
            .and_then(|date| date.and_hms_opt(hour, minute, second))
            .unwrap()
    }

    fn flying(delivery: Delivery) -> ArcanoPostman {
        let mut mage = ArcanoPostman::new();
        mage.set_round(round(delivery));
        mage.flight = Flight::Arriving;
        mage
    }

    fn fly_until_not(mage: &mut ArcanoPostman, flight: Flight) {
        let now = evening(21, 0, 0);
        for _ in 0..1000 {
            if mage.flight != flight {
                return;
            }
            mage.fly(&layout(), now);
        }
    }

    #[test]
    fn test_leaves_to_reach_the_violet_mailbox_on_the_hour() {
        let layout = layout();
        let (mailbox_x, _) = violet_mailbox(&layout).unwrap();
        let ride = ride_time(stop_x(mailbox_x) + SPRITE_WIDTH, SPEED);
        let departure = evening(21, 0, 0) - ride;

        let mut mage = ArcanoPostman::new();
        mage.set_round(round(Delivery::Due));
        mage.fly(&layout, departure - Duration::seconds(5));
        assert_eq!(mage.flight, Flight::Waiting);
        mage.fly(&layout, departure);
        assert_eq!(mage.flight, Flight::Arriving);
    }

    #[test]
    fn test_stays_home_while_the_secret_is_locked() {
        let mut mage = ArcanoPostman::new();
        mage.fly(&layout(), evening(20, 59, 58));
        assert_eq!(mage.flight, Flight::Waiting);
    }

    #[test]
    fn test_stops_in_the_air_above_the_violet_mailbox_and_casts() {
        let layout = layout();
        let (mailbox_x, mailbox_y) = violet_mailbox(&layout).unwrap();
        let mut mage = flying(Delivery::Due);
        fly_until_not(&mut mage, Flight::Arriving);
        assert_eq!(mage.flight, Flight::Casting(0));
        assert_eq!(mage.x, stop_x(mailbox_x));
        assert!(altitude(mailbox_y) + (MAGE.len() as u16) < mailbox_y);
        assert!(mage.hides_flag(Delivery::Due));

        for _ in 0..CAST_FRAMES * 2 {
            mage.fly(&layout, evening(21, 0, 0));
        }
        assert!(matches!(mage.flight, Flight::Casting(_)));

        mage.set_round(round(Delivery::Done));
        mage.fly(&layout, evening(21, 0, 5));
        assert_eq!(mage.flight, Flight::Leaving);
        assert!(!mage.hides_flag(Delivery::Done));
    }

    #[test]
    fn test_crosses_the_sky_without_stopping_when_nothing_is_due() {
        let mut mage = flying(Delivery::Nothing);
        assert!(!mage.hides_flag(Delivery::Nothing));
        fly_until_not(&mut mage, Flight::Arriving);
        assert_eq!(mage.flight, Flight::Leaving);
        fly_until_not(&mut mage, Flight::Leaving);
        assert_eq!(mage.flight, Flight::Waiting);
    }

    #[test]
    fn test_casting_ends_even_if_the_arcane_never_comes() {
        let mut mage = flying(Delivery::Due);
        fly_until_not(&mut mage, Flight::Arriving);
        for _ in 0..=MAX_CAST_FRAMES {
            mage.fly(&layout(), evening(21, 0, 0));
        }
        assert_eq!(mage.flight, Flight::Leaving);
    }

    #[test]
    fn test_sprite_fits_its_width() {
        let widest = MAGE
            .iter()
            .chain(&WAND)
            .chain(&[BROOM])
            .map(|row| row.chars().count())
            .max()
            .unwrap();
        assert_eq!(widest as f32, SPRITE_WIDTH);
    }
}
