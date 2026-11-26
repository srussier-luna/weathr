#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TodayLetters {
    pub day: usize,
    pub start: usize,
}

impl TodayLetters {
    pub fn on_day(saved: Option<Self>, day: usize, letters_read: usize) -> Self {
        let start = match saved {
            Some(saved) if saved.day == day => saved.start.min(letters_read),
            _ => letters_read,
        };
        Self { day, start }
    }

    pub fn contains(self, letter: usize) -> bool {
        letter >= self.start
    }

    pub fn browse(self, letter: usize, step: isize, available: usize) -> Option<usize> {
        let target = letter.checked_add_signed(step)?;
        (self.contains(letter) && self.contains(target) && target < available && target != letter)
            .then_some(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_on(day: usize, saved: Option<TodayLetters>, letters_read: usize) -> TodayLetters {
        TodayLetters::on_day(saved, day, letters_read)
    }

    #[test]
    fn test_letters_read_the_same_day_browse_together() {
        let morning = read_on(12, Some(TodayLetters { day: 11, start: 3 }), 5);
        let evening = read_on(12, Some(morning), 7);

        assert_eq!(evening, TodayLetters { day: 12, start: 5 });
        assert_eq!(evening.browse(6, -1, 7), Some(5));
        assert_eq!(evening.browse(5, 1, 7), Some(6));
        assert_eq!(evening.browse(5, -1, 7), None);
        assert_eq!(evening.browse(6, 1, 7), None);
    }

    #[test]
    fn test_yesterday_letters_read_today_join_the_range() {
        let launch = read_on(12, Some(TodayLetters { day: 10, start: 2 }), 4);
        let today = read_on(12, Some(launch), 8);

        assert_eq!(today, TodayLetters { day: 12, start: 4 });
        assert_eq!(today.browse(4, 1, 9), Some(5));
        assert_eq!(today.browse(7, -1, 9), Some(6));
        assert_eq!(today.browse(5, -1, 9), Some(4));
        assert_eq!(today.browse(4, -1, 9), None);
    }

    #[test]
    fn test_new_day_empties_the_range() {
        let today = read_on(13, Some(TodayLetters { day: 12, start: 5 }), 7);

        assert_eq!(today, TodayLetters { day: 13, start: 7 });
        assert!(!today.contains(6));
        assert_eq!(today.browse(6, 1, 9), None);
        assert_eq!(today.browse(6, -1, 9), None);
    }

    #[test]
    fn test_relaunch_the_same_day_keeps_the_range() {
        let saved = TodayLetters { day: 12, start: 5 };

        assert_eq!(read_on(12, Some(saved), 7), saved);
    }

    #[test]
    fn test_old_save_starts_the_range_now() {
        let today = read_on(12, None, 7);

        assert_eq!(today, TodayLetters { day: 12, start: 7 });
        assert_eq!(today.browse(7, -1, 9), None);
        assert_eq!(today.browse(7, 1, 9), Some(8));
    }

    #[test]
    fn test_past_letter_does_not_browse() {
        let today = TodayLetters { day: 12, start: 5 };

        assert_eq!(today.browse(3, 1, 9), None);
        assert_eq!(today.browse(4, 1, 9), None);
        assert_eq!(today.browse(3, -1, 9), None);
    }

    #[test]
    fn test_next_delivered_unread_opens_from_the_last_letter() {
        let today = TodayLetters { day: 12, start: 5 };

        assert_eq!(today.browse(6, 1, 9), Some(7));
        assert_eq!(today.browse(8, 1, 9), None);
    }

    #[test]
    fn test_reset_letters_read_shrinks_the_start() {
        let today = read_on(12, Some(TodayLetters { day: 12, start: 5 }), 3);

        assert_eq!(today.start, 3);
    }
}
