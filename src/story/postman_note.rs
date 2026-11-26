#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MailStep {
    PostmanNote,
    Letter(usize),
    Close,
}

pub fn is_postman_note_waiting(box_full: bool, note_day: Option<usize>, day: usize) -> bool {
    box_full && note_day != Some(day)
}

pub fn next_mail(
    open_letter: Option<usize>,
    range_start: usize,
    available: usize,
    letters_read: usize,
    is_note_waiting: bool,
) -> Option<MailStep> {
    if available == 0 {
        return None;
    }
    Some(match open_letter {
        Some(index) if index < range_start => {
            if letters_read < available {
                MailStep::Letter(letters_read)
            } else if is_note_waiting {
                MailStep::PostmanNote
            } else {
                MailStep::Close
            }
        }
        Some(index) if index + 1 < available => MailStep::Letter(index + 1),
        Some(_) => MailStep::Close,
        None if is_note_waiting => MailStep::PostmanNote,
        None => MailStep::Letter(letters_read.min(available - 1)),
    })
}

pub fn has_unread_after(open_letter: usize, available: usize, letters_read: usize) -> bool {
    available > letters_read.max(open_letter + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::content::letters;
    use crate::story::pause::is_box_full;
    use crate::story::sources::{self, Recipes};
    use crate::story::{Entry, StoryClock, Timeline};
    use chrono::NaiveDate;

    fn timeline_on(day: usize) -> Timeline {
        let clock = StoryClock::new(NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(), 9);
        Timeline::new(
            &sources::registered(letters::defaults().len(), Recipes::default()),
            &clock,
            day,
        )
    }

    #[test]
    fn test_full_box_opens_the_postman_note_before_any_letter() {
        let timeline = timeline_on(7);
        let available = timeline.delivered_by(7);
        let waiting = is_postman_note_waiting(is_box_full(&timeline, 7, 0), None, 7);

        assert!(waiting);
        assert_eq!(
            next_mail(None, 0, available, 0, waiting),
            Some(MailStep::PostmanNote)
        );
        assert_eq!(
            next_mail(None, 0, available, 0, false),
            Some(MailStep::Letter(0))
        );
        assert_eq!(
            next_mail(Some(0), 0, available, 0, waiting),
            Some(MailStep::Letter(1))
        );
        assert_eq!(
            next_mail(Some(available - 1), 0, available, 0, waiting),
            Some(MailStep::Close)
        );
    }

    #[test]
    fn test_opened_postman_note_comes_back_only_with_a_new_pause() {
        let timeline = timeline_on(7);
        let available = timeline.delivered_by(7);
        let waiting = is_postman_note_waiting(is_box_full(&timeline, 7, 0), Some(7), 7);

        assert!(!waiting);
        assert_eq!(
            next_mail(None, 0, available, 0, waiting),
            Some(MailStep::Letter(0))
        );
        assert!(!is_postman_note_waiting(
            is_box_full(&timeline, 7, 1),
            None,
            7
        ));
        assert!(is_postman_note_waiting(
            is_box_full(&timeline_on(8), 8, 1),
            Some(7),
            8
        ));
    }

    #[test]
    fn test_mail_from_a_past_letter_jumps_to_the_next_unread() {
        assert_eq!(
            next_mail(Some(1), 5, 8, 6, false),
            Some(MailStep::Letter(6))
        );
        assert_eq!(next_mail(Some(4), 5, 8, 6, true), Some(MailStep::Letter(6)));
        assert_eq!(
            next_mail(Some(1), 5, 8, 8, true),
            Some(MailStep::PostmanNote)
        );
        assert_eq!(next_mail(Some(1), 5, 8, 8, false), Some(MailStep::Close));
        assert_eq!(
            next_mail(Some(5), 5, 8, 8, false),
            Some(MailStep::Letter(6))
        );
    }

    #[test]
    fn test_empty_courrier_opens_nothing() {
        assert_eq!(next_mail(None, 0, 0, 0, true), None);
    }

    #[test]
    fn test_unread_mail_shows_after_each_letter_but_the_last() {
        assert!(has_unread_after(1, 3, 2));
        assert!(!has_unread_after(2, 3, 3));
        assert!(!has_unread_after(0, 3, 3));
        assert!(!has_unread_after(0, 0, 0));
    }

    #[test]
    fn test_unread_mail_after_an_archive_shows() {
        let timeline = timeline_on(40);
        let archive = timeline
            .mail()
            .position(|(_, entry)| matches!(entry, Entry::Archive(_)))
            .unwrap();
        let available = timeline.delivered_by(40);

        assert!(available > archive + 1);
        assert!(has_unread_after(archive, available, archive + 1));
        assert!(!has_unread_after(archive, available, available));
    }
}
