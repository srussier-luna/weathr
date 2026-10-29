use crate::animation::garden::{self, GardenLayout};
use crate::render::TerminalRenderer;
use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use crossterm::style::Color;
use std::io;

const SPEED: f32 = 0.3;
const STOP_FRAMES: u16 = 90;
const ARRIVAL_FRAMES: f32 = 75.0;
const HEAD_COLUMN: u16 = 5;
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
const PAPER_FISH: &str = "><>";
const PAPER_PINK: Color = Color::Rgb {
    r: 255,
    g: 150,
    b: 190,
};
const MAILBOX_FISH_COLUMN: u16 = 2;

enum Trip {
    Waiting,
    Riding,
    Delivering(u16),
    Leaving,
    Arriving,
    Staying,
}

pub struct Postman {
    trip: Trip,
    x: f32,
    is_mail_waiting: bool,
    rings_bell: bool,
    is_april_fools: bool,
    is_story_frozen: bool,
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
            is_april_fools: false,
            is_story_frozen: false,
            round_hour: DEFAULT_ROUND_HOUR,
            last_round: None,
        }
    }

    pub fn set_round_hour(&mut self, hour: u32) {
        self.round_hour = hour;
    }

    pub fn set_story_frozen(&mut self, is_frozen: bool) {
        self.is_story_frozen = is_frozen;
    }

    pub fn set_mail_waiting(&mut self, is_waiting: bool) {
        self.is_mail_waiting = is_waiting;
    }

    pub fn set_bell(&mut self, rings_bell: bool) {
        self.rings_bell = rings_bell;
    }

    pub fn set_april_fools(&mut self, is_april_fools: bool) {
        self.is_april_fools = is_april_fools;
    }

    pub fn is_delivering(&self) -> bool {
        matches!(self.trip, Trip::Delivering(_))
    }

    pub fn visit(&mut self) {
        self.x = -SPRITE_WIDTH;
        self.trip = Trip::Arriving;
    }

    pub fn leave(&mut self) {
        if matches!(self.trip, Trip::Arriving | Trip::Staying) {
            self.trip = if self.x < 0.0 {
                Trip::Waiting
            } else {
                Trip::Leaving
            };
        }
    }

    pub fn is_arriving(&self, layout: &GardenLayout) -> bool {
        layout.plan.mailbox_x.is_some() && matches!(self.trip, Trip::Arriving)
    }

    pub fn head(&self, layout: &GardenLayout) -> Option<(u16, u16)> {
        let mailbox_x = layout.plan.mailbox_x?;
        matches!(self.trip, Trip::Staying)
            .then_some((mailbox_x + HEAD_COLUMN, layout.horizon_y.saturating_sub(2)))
    }

    fn due_round(&self, stop_x: f32, now: NaiveDateTime) -> Option<NaiveDate> {
        if self.is_story_frozen {
            return None;
        }
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
            Trip::Arriving => {
                self.x += ((stop_x + SPRITE_WIDTH) / ARRIVAL_FRAMES).max(SPEED);
                if self.x >= stop_x {
                    self.x = stop_x;
                    Trip::Staying
                } else {
                    Trip::Arriving
                }
            }
            Trip::Staying => {
                self.x = stop_x;
                Trip::Staying
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

        if let Some((x, y)) = self.mailbox_fish(layout) {
            garden::draw(renderer, x, y, &[PAPER_FISH], PAPER_PINK)?;
        }

        if matches!(self.trip, Trip::Waiting) {
            return Ok(());
        }

        let x = self.x.round() as i32;
        let y = layout.horizon_y.saturating_sub(2);
        garden::draw_clipped(renderer, x, y, &RIDER, POSTAL_YELLOW)?;
        garden::draw_clipped(renderer, x, y + 2, &[WHEELS], Color::DarkGrey)?;

        if self.is_april_fools {
            garden::draw_clipped(renderer, x, y, &[PAPER_FISH], PAPER_PINK)?;
        }

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

    fn mailbox_fish(&self, layout: &GardenLayout) -> Option<(u16, u16)> {
        let mailbox_x = layout.plan.mailbox_x?;
        self.is_april_fools.then_some((
            mailbox_x + MAILBOX_FISH_COLUMN,
            layout.horizon_y.saturating_sub(2),
        ))
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
    fn test_frozen_story_skips_rounds() {
        let mut postman = Postman::new();
        postman.set_round_hour(8);
        postman.set_story_frozen(true);
        assert_eq!(postman.due_round(40.0, at(22, 7, 59, 56)), None);
    }

    #[test]
    fn test_visit_stays_at_the_mailbox_until_told_to_leave() {
        let layout = GardenLayout::new(120, 40);
        let mailbox_x = layout.plan.mailbox_x.unwrap();
        let mut postman = Postman::new();
        postman.visit();
        assert!(postman.is_arriving(&layout));
        assert_eq!(postman.head(&layout), None);

        for _ in 0..ARRIVAL_FRAMES as usize + 1 {
            postman.update(&layout);
        }
        assert!(!postman.is_arriving(&layout));
        assert_eq!(
            postman.head(&layout),
            Some((mailbox_x + HEAD_COLUMN, layout.horizon_y - 2))
        );
        for _ in 0..STOP_FRAMES * 2 {
            postman.update(&layout);
        }
        assert!(matches!(postman.trip, Trip::Staying));
        assert!(!postman.is_delivering());

        postman.leave();
        assert!(matches!(postman.trip, Trip::Leaving));
        assert_eq!(postman.head(&layout), None);
    }

    #[test]
    fn test_paper_fish_only_on_april_fools() {
        let layout = GardenLayout::new(120, 40);
        let mailbox_x = layout.plan.mailbox_x.unwrap();
        let mut postman = Postman::new();
        assert_eq!(postman.mailbox_fish(&layout), None);

        postman.set_april_fools(true);
        assert_eq!(
            postman.mailbox_fish(&layout),
            Some((mailbox_x + MAILBOX_FISH_COLUMN, layout.horizon_y - 2))
        );
    }

    #[test]
    fn test_mailbox_fish_leaves_the_flag_and_skips_a_missing_mailbox() {
        let mut layout = GardenLayout::new(120, 40);
        let mut postman = Postman::new();
        postman.set_april_fools(true);
        let (fish_x, _) = postman.mailbox_fish(&layout).unwrap();
        let mailbox_x = layout.plan.mailbox_x.unwrap();
        assert!(fish_x + PAPER_FISH.len() as u16 <= mailbox_x + 6);

        layout.plan.mailbox_x = None;
        postman.visit();
        postman.update(&layout);
        assert_eq!(postman.mailbox_fish(&layout), None);
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
