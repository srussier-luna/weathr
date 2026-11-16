#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MailStep {
    PostmanNote,
    Letter(usize),
    Arcane(usize),
    Close,
}

pub fn is_postman_note_waiting(box_full: bool, note_day: Option<usize>, day: usize) -> bool {
    box_full && note_day != Some(day)
}

pub fn next_mail(
    open_letter: Option<usize>,
    available: usize,
    letters_read: usize,
    is_note_waiting: bool,
    waiting_arcane: Option<usize>,
) -> Option<MailStep> {
    let arcane_or = |step| Some(waiting_arcane.map_or(step, MailStep::Arcane));
    match open_letter {
        Some(index) if index + 1 < available => Some(MailStep::Letter(index + 1)),
        Some(_) => arcane_or(MailStep::Close),
        None if available == 0 => waiting_arcane.map(MailStep::Arcane),
        None if is_note_waiting => Some(MailStep::PostmanNote),
        None if letters_read < available => Some(MailStep::Letter(letters_read)),
        None => arcane_or(MailStep::Letter(available - 1)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::content::letters;
    use crate::story::pause::is_box_full;
    use crate::story::{StoryClock, Timeline, sources};
    use chrono::NaiveDate;

    fn timeline_on(day: usize) -> Timeline {
        let clock = StoryClock::new(NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(), 9);
        Timeline::new(&sources::registered(letters::defaults().len()), &clock, day)
    }

    #[test]
    fn test_full_box_opens_the_postman_note_before_any_letter() {
        let timeline = timeline_on(7);
        let available = timeline.delivered_by(7);
        let waiting = is_postman_note_waiting(is_box_full(&timeline, 7, 0), None, 7);

        assert!(waiting);
        assert_eq!(
            next_mail(None, available, 0, waiting, None),
            Some(MailStep::PostmanNote)
        );
        assert_eq!(
            next_mail(None, available, 0, false, None),
            Some(MailStep::Letter(0))
        );
        assert_eq!(
            next_mail(Some(0), available, 0, waiting, None),
            Some(MailStep::Letter(1))
        );
        assert_eq!(
            next_mail(Some(available - 1), available, 0, waiting, None),
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
            next_mail(None, available, 0, waiting, None),
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
    fn test_empty_courrier_opens_nothing() {
        assert_eq!(next_mail(None, 0, 0, true, None), None);
    }

    #[test]
    fn test_waiting_arcane_comes_after_the_unread_letters() {
        assert_eq!(
            next_mail(None, 5, 3, false, Some(1)),
            Some(MailStep::Letter(3))
        );
        assert_eq!(
            next_mail(Some(3), 5, 4, false, Some(1)),
            Some(MailStep::Letter(4))
        );
        assert_eq!(
            next_mail(Some(4), 5, 5, false, Some(1)),
            Some(MailStep::Arcane(1))
        );
        assert_eq!(
            next_mail(None, 5, 5, false, Some(1)),
            Some(MailStep::Arcane(1))
        );
        assert_eq!(
            next_mail(None, 5, 5, false, None),
            Some(MailStep::Letter(4))
        );
        assert_eq!(
            next_mail(None, 5, 3, true, Some(1)),
            Some(MailStep::PostmanNote)
        );
    }
}
