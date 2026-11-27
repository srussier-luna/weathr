use chrono::{DateTime, Local};
use std::time::SystemTime;

#[cfg(feature = "dev-date")]
use chrono::{Days, NaiveDate, TimeDelta};
#[cfg(feature = "dev-date")]
use std::sync::OnceLock;

#[cfg(feature = "dev-date")]
static DAY_OFFSET: OnceLock<i64> = OnceLock::new();

pub fn now() -> DateTime<Local> {
    let now = real_now();
    #[cfg(feature = "dev-date")]
    if let Some(&days) = DAY_OFFSET.get() {
        return shifted(now, days);
    }
    now
}

#[cfg(feature = "dev-date")]
pub fn set_today(today: NaiveDate) {
    let _ = DAY_OFFSET.set(days_between(real_now().date_naive(), today));
}

fn real_now() -> DateTime<Local> {
    DateTime::from(SystemTime::now())
}

#[cfg(feature = "dev-date")]
fn days_between(from: NaiveDate, to: NaiveDate) -> i64 {
    (to - from).num_days()
}

#[cfg(feature = "dev-date")]
fn shifted(now: DateTime<Local>, days: i64) -> DateTime<Local> {
    let span = Days::new(days.unsigned_abs());
    let moved = if days >= 0 {
        now.checked_add_days(span)
    } else {
        now.checked_sub_days(span)
    };
    moved.unwrap_or_else(|| now + TimeDelta::days(days))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_follows_the_real_clock_without_test_date() {
        let before = real_now();
        let now = now();
        let after = real_now();
        assert!(before <= now && now <= after);
    }

    #[cfg(feature = "dev-date")]
    mod dev_date {
        use super::super::*;
        use chrono::{TimeZone, Timelike};

        fn at(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Local> {
            Local
                .with_ymd_and_hms(year, month, day, hour, minute, 0)
                .single()
                .unwrap()
        }

        #[test]
        fn test_days_between_counts_forward_and_backward() {
            let real = NaiveDate::from_ymd_opt(2026, 11, 27).unwrap();
            let december = NaiveDate::from_ymd_opt(2026, 12, 3).unwrap();
            assert_eq!(days_between(real, december), 6);
            assert_eq!(days_between(december, real), -6);
            assert_eq!(days_between(real, real), 0);
        }

        #[test]
        fn test_shifted_moves_the_date_and_keeps_the_time_of_day() {
            let shifted = shifted(at(2026, 11, 27, 14, 35), 4);
            assert_eq!(
                shifted.date_naive(),
                NaiveDate::from_ymd_opt(2026, 12, 1).unwrap()
            );
            assert_eq!((shifted.hour(), shifted.minute()), (14, 35));
        }

        #[test]
        fn test_shifted_goes_back_in_time() {
            let shifted = shifted(at(2026, 12, 3, 9, 5), -3);
            assert_eq!(
                shifted.date_naive(),
                NaiveDate::from_ymd_opt(2026, 11, 30).unwrap()
            );
            assert_eq!((shifted.hour(), shifted.minute()), (9, 5));
        }

        #[test]
        fn test_shifted_keeps_the_time_of_day_across_daylight_saving() {
            let shifted = shifted(at(2026, 11, 27, 14, 35), -150);
            assert_eq!((shifted.hour(), shifted.minute()), (14, 35));
        }
    }
}
