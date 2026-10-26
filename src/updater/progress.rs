use super::PREFIX;
use std::io::{self, IsTerminal, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub const AFTER_PULL: u8 = 10;
pub const DONE: u8 = 100;
const BUILD_CEILING: u8 = 95;
const BAR_WIDTH: usize = 20;
const SPINNER: [char; 4] = ['/', '|', '\\', '-'];
const TICK: Duration = Duration::from_millis(100);
const LABEL: &str = "Installation en cours";

pub fn build_percent(artifacts: usize, total: usize) -> u8 {
    if total == 0 {
        return AFTER_PULL;
    }
    let span = usize::from(BUILD_CEILING - AFTER_PULL);
    let advance = (artifacts * span / total).min(span);
    AFTER_PULL + advance as u8
}

pub fn package_count(lock: &str) -> usize {
    lock.lines()
        .filter(|line| line.trim() == "[[package]]")
        .count()
}

fn spinner_frame(tick: usize) -> char {
    SPINNER[tick % SPINNER.len()]
}

fn bar(percent: u8) -> String {
    let filled = usize::from(percent.min(DONE)) * BAR_WIDTH / usize::from(DONE);
    if filled == BAR_WIDTH {
        return format!("[{}]", "=".repeat(BAR_WIDTH));
    }
    format!(
        "[{}>{}]",
        "=".repeat(filled),
        " ".repeat(BAR_WIDTH - filled - 1)
    )
}

fn line(tick: usize, percent: u8) -> String {
    format!(
        "{PREFIX} {LABEL} {}  {} {percent} %",
        spinner_frame(tick),
        bar(percent)
    )
}

pub struct Progress {
    percent: Arc<AtomicU8>,
    stop: Arc<AtomicBool>,
    ticker: Option<JoinHandle<usize>>,
}

impl Progress {
    pub fn start() -> Self {
        let percent = Arc::new(AtomicU8::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        let ticker = if io::stdout().is_terminal() {
            let percent = Arc::clone(&percent);
            let stop = Arc::clone(&stop);
            Some(thread::spawn(move || {
                let mut tick = 0;
                while !stop.load(Ordering::Relaxed) {
                    draw(tick, percent.load(Ordering::Relaxed));
                    tick += 1;
                    thread::sleep(TICK);
                }
                tick
            }))
        } else {
            println!("{PREFIX} {LABEL}…");
            None
        };
        Self {
            percent,
            stop,
            ticker,
        }
    }

    pub fn set(&self, percent: u8) {
        self.percent.fetch_max(percent, Ordering::Relaxed);
    }

    pub fn finish(mut self) {
        self.stop.store(true, Ordering::Relaxed);
        let Some(ticker) = self.ticker.take() else {
            return;
        };
        let tick = ticker.join().unwrap_or_default();
        draw(tick, self.percent.load(Ordering::Relaxed));
        println!();
    }
}

fn draw(tick: usize, percent: u8) {
    let mut stdout = io::stdout();
    let _ = write!(stdout, "\r{}", line(tick, percent));
    let _ = stdout.flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_artifact_stays_at_pull_step() {
        assert_eq!(build_percent(0, 40), AFTER_PULL);
    }

    #[test]
    fn test_artifacts_move_between_pull_and_ceiling() {
        assert_eq!(build_percent(20, 40), 52);
        assert_eq!(build_percent(40, 40), BUILD_CEILING);
    }

    #[test]
    fn test_extra_artifacts_stay_capped_before_install() {
        assert_eq!(build_percent(90, 40), BUILD_CEILING);
    }

    #[test]
    fn test_unknown_total_stays_at_pull_step() {
        assert_eq!(build_percent(12, 0), AFTER_PULL);
    }

    #[test]
    fn test_package_count_reads_lock_entries() {
        let lock = "version = 4\n\n[[package]]\nname = \"a\"\n\n[[package]]\nname = \"weathr\"\n";
        assert_eq!(package_count(lock), 2);
    }

    #[test]
    fn test_bar_fills_with_percentage() {
        assert_eq!(bar(0), format!("[>{}]", " ".repeat(19)));
        assert_eq!(bar(45), "[=========>          ]");
        assert_eq!(bar(DONE), format!("[{}]", "=".repeat(20)));
    }

    #[test]
    fn test_spinner_cycles_through_frames() {
        let frames: String = (0..5).map(spinner_frame).collect();
        assert_eq!(frames, "/|\\-/");
    }

    #[test]
    fn test_line_shows_spinner_bar_and_percentage() {
        assert_eq!(
            line(0, 45),
            "[weathr] Installation en cours /  [=========>          ] 45 %"
        );
    }
}
