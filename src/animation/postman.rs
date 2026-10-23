use crate::animation::garden::{self, GardenLayout};
use crate::render::TerminalRenderer;
use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use crossterm::style::Color;
use std::io;

const SPEED: f32 = 0.3;
const STOP_FRAMES: u16 = 90;
const FRAME_MILLIS: f32 = 1000.0 / 30.0;
const DEFAULT_ROUND_HOUR: u32 = 9;
const ROUND_WINDOW_MINUTES: i64 = 1;
const RIDER: [&str; 2] = ["   __o", " _`\\<,_"];
const WHEELS: &str = "(*)/ (*)";
const SPRITE_WIDTH: f32 = 8.0;
const POSTAL_YELLOW: Color = Color::Rgb {
    r: 255,
    g: 204,
    b: 0,
};

enum Trip {
    Waiting,
    Riding,
    Delivering(u16),
    Leaving,
}

pub struct Postman {
    trip: Trip,
    x: f32,
    is_mail_waiting: bool,
    rings_bell: bool,
    round_hour: u32,
    last_round: Option<NaiveDate>,
}

impl Postman {
    pub fn new() -> Self {
        Self {
            trip: Trip::Waiting,
            x: -SPRITE_WIDTH,
            is_mail_waiting: false,
            rings_bell: false,
            round_hour: DEFAULT_ROUND_HOUR,
            last_round: None,
        }
    }

    pub fn set_round_hour(&mut self, hour: u32) {
        self.round_hour = hour;
    }

    pub fn set_mail_waiting(&mut self, is_waiting: bool) {
        self.is_mail_waiting = is_waiting;
    }

    pub fn set_bell(&mut self, rings_bell: bool) {
        self.rings_bell = rings_bell;
    }

    pub fn is_delivering(&self) -> bool {
        matches!(self.trip, Trip::Delivering(_))
    }

    fn due_round(&self, stop_x: f32, now: NaiveDateTime) -> Option<NaiveDate> {
        let ride = Duration::milliseconds(((stop_x + SPRITE_WIDTH) / SPEED * FRAME_MILLIS) as i64);
        let today = now.date();
        [Some(today), today.succ_opt()]
            .into_iter()
            .flatten()
            .filter(|date| self.last_round != Some(*date))
            .find(|date| {
                date.and_hms_opt(self.round_hour, 0, 0)
                    .is_some_and(|arrival| {
                        let departure = arrival - ride;
                        departure <= now
                            && now < departure + Duration::minutes(ROUND_WINDOW_MINUTES)
                    })
            })
    }

    pub fn update(&mut self, layout: &GardenLayout) {
        let Some(mailbox_x) = layout.plan.mailbox_x else {
            return;
        };
        let stop_x = mailbox_x as f32;

        self.trip = match self.trip {
            Trip::Waiting => match self.due_round(stop_x, Local::now().naive_local()) {
                Some(round) => {
                    self.last_round = Some(round);
                    self.x = -SPRITE_WIDTH;
                    Trip::Riding
                }
                None => Trip::Waiting,
            },
            Trip::Riding => {
                self.x += SPEED;
                if self.x >= stop_x {
                    Trip::Delivering(STOP_FRAMES)
                } else {
                    Trip::Riding
                }
            }
            Trip::Delivering(remaining) if remaining > 0 => Trip::Delivering(remaining - 1),
            Trip::Delivering(_) => Trip::Leaving,
            Trip::Leaving => {
                self.x += SPEED;
                if self.x > layout.width as f32 {
                    Trip::Waiting
                } else {
                    Trip::Leaving
                }
            }
        };
    }

    pub fn render(&self, renderer: &mut TerminalRenderer, layout: &GardenLayout) -> io::Result<()> {
        if let Some(mailbox_x) = layout.plan.mailbox_x
            && self.is_mail_waiting
        {
            let flag_y = layout.horizon_y.saturating_sub(3);
            renderer.render_char(mailbox_x + 6, flag_y, '>', Color::Red)?;
            renderer.render_char(mailbox_x + 6, flag_y + 1, '|', Color::Red)?;
        }

        if matches!(self.trip, Trip::Waiting) {
            return Ok(());
        }

        let x = self.x.round() as i32;
        let y = layout.horizon_y.saturating_sub(2);
        garden::draw_clipped(renderer, x, y, &RIDER, POSTAL_YELLOW)?;
        garden::draw_clipped(renderer, x, y + 2, &[WHEELS], Color::DarkGrey)?;

        if self.rings_bell && matches!(self.trip, Trip::Riding) {
            garden::draw_clipped(
                renderer,
                x + 3,
                y.saturating_sub(1),
                &["Dring !"],
                Color::White,
            )?;
        }

        if matches!(self.trip, Trip::Delivering(_)) {
            garden::draw_clipped(
                renderer,
                x + 3,
                y.saturating_sub(1),
                &["Courrier !"],
                Color::White,
            )?;
        }
        Ok(())
    }
}

impl Default for Postman {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(day: u32, hour: u32, minute: u32, second: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2026, 10, day)
            .and_then(|date| date.and_hms_opt(hour, minute, second))
            .unwrap()
    }

    #[test]
    fn test_postman_leaves_to_reach_mailbox_on_the_hour() {
        let mut postman = Postman::new();
        postman.set_round_hour(8);
        let today = NaiveDate::from_ymd_opt(2026, 10, 22);
        assert_eq!(postman.due_round(40.0, at(22, 7, 59, 50)), None);
        assert_eq!(postman.due_round(40.0, at(22, 7, 59, 56)), today);
        assert_eq!(postman.due_round(40.0, at(22, 8, 2, 0)), None);
        postman.last_round = today;
        assert_eq!(postman.due_round(40.0, at(22, 7, 59, 58)), None);
    }

    #[test]
    fn test_midnight_round_belongs_to_next_day() {
        let mut postman = Postman::new();
        postman.set_round_hour(0);
        assert_eq!(
            postman.due_round(40.0, at(22, 23, 59, 57)),
            NaiveDate::from_ymd_opt(2026, 10, 23)
        );
    }
}
