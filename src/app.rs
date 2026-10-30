use crate::animation::lore::{Choices, Unlocks};
use crate::animation_manager::AnimationManager;
use crate::app_state::AppState;
use crate::badges::{self, Badge};
use crate::config::Config;
use crate::demo::{Demo, SCENE_DURATION};
use crate::error::WeatherError;
use crate::holiday::{Holiday, MeteorShower};
use crate::overlay;
use crate::progress::Progress;
use crate::rain_alert::{self, RainAlert};
use crate::release_notes::{self, Section};
use crate::remote_weather::{self, Reply, Request};
use crate::render::TerminalRenderer;
use crate::scene::WorldScene;
use crate::story::content::chronicles::{self, BOTTLE_MESSAGES, CITIES, OBJECTS, RIDDLES};
use crate::story::content::letters::{self, Letter};
use crate::story::content::{self, ReadContext};
use crate::story::postman_note::{self, MailStep};
use crate::story::{Entry, StoryClock, Thing, Timeline, pause, sources};
use crate::views;
use crate::weather::{
    OpenMeteoProvider, WeatherClient, WeatherCondition, WeatherData, WeatherLocation,
};
use crate::welcome::{self, Step};
use chrono::{Local, NaiveDate, Timelike};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::io;
use std::num::NonZeroU16;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

const REFRESH_INTERVAL: Duration = Duration::from_secs(300);
const INPUT_POLL_FPS: u64 = 30;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / INPUT_POLL_FPS);
const TOAST_DURATION: Duration = Duration::from_secs(4);
const PATIENCE: Duration = Duration::from_secs(60 * 60);
const STARS_FOR_HUNTER: u32 = 3;
const REMOTE_CHECK_FRAMES: u32 = 300;
const MAX_ANSWER_LENGTH: usize = 30;
const REMOTE_CHANNEL_SIZE: usize = 16;
const OVERLAY_MARGIN_ROWS: u16 = 12;

fn count_read(timeline: &Timeline, progress: &Progress, kind: fn(&Entry) -> bool) -> usize {
    timeline
        .mail()
        .take(progress.letters_read)
        .filter(|(_, entry)| kind(entry))
        .count()
}

fn unlocks_on(timeline: &Timeline, day: usize, answers: &BTreeMap<String, u8>) -> Unlocks {
    let arrived: Vec<Thing> = timeline.arrived_by(day).collect();
    let has_arrived = |thing: Thing| arrived.contains(&thing);
    Unlocks {
        bench: has_arrived(Thing::Bench),
        cat_bow: has_arrived(Thing::CatBow),
        footprints: has_arrived(Thing::Footprints),
        well: has_arrived(Thing::Well),
        bone: has_arrived(Thing::Bone),
        key: has_arrived(Thing::Key),
        treehouse: has_arrived(Thing::Treehouse),
        owl_friend: has_arrived(Thing::OwlFriend),
        blue_lantern: has_arrived(Thing::BlueLantern),
        garland: has_arrived(Thing::Garland),
        kittens: has_arrived(Thing::Kittens),
        x_mark: has_arrived(Thing::XMark),
        balloon: has_arrived(Thing::Balloon),
        ducklings: has_arrived(Thing::Ducklings),
        weather_vane: has_arrived(Thing::WeatherVane),
        bunting: has_arrived(Thing::Bunting),
        chest: has_arrived(Thing::Chest),
        wet_footprints: has_arrived(Thing::WetFootprints),
        silhouette: has_arrived(Thing::Silhouette),
        whispering_well: has_arrived(Thing::WhisperingWell),
        staring: has_arrived(Thing::Staring),
        found_objects: has_arrived(Thing::FoundObjects),
        bottles: has_arrived(Thing::Bottles),
        choices: Choices::from_answers(answers),
    }
}

fn earn(earned: &mut BTreeSet<String>, ids: Vec<&'static str>) -> Vec<&'static Badge> {
    ids.into_iter()
        .filter(|id| earned.insert(id.to_string()))
        .filter_map(badges::find)
        .collect()
}

fn badge_card(overlay: Option<Overlay>) -> Option<&'static Badge> {
    match overlay {
        Some(Overlay::NewBadge(badge) | Overlay::NoteBadge(badge)) => Some(badge),
        _ => None,
    }
}

fn next_overlay(
    overlay: Option<Overlay>,
    pending_badges: &mut VecDeque<&'static Badge>,
) -> Option<Overlay> {
    match overlay {
        None => pending_badges.pop_front().map(Overlay::NewBadge),
        Some(Overlay::PostmanNote) => Some(
            pending_badges
                .pop_front()
                .map_or(Overlay::PostmanNote, Overlay::NoteBadge),
        ),
        overlay => overlay,
    }
}

fn closed_overlay(overlay: Option<Overlay>) -> Option<Overlay> {
    match overlay {
        Some(Overlay::NoteBadge(_)) => Some(Overlay::PostmanNote),
        _ => None,
    }
}

fn requeue_replaced_badge(
    previous: Option<Overlay>,
    current: Option<Overlay>,
    pending_badges: &mut VecDeque<&'static Badge>,
) {
    if let (Some(badge), Some(current)) = (badge_card(previous), current)
        && badge_card(Some(current)).is_none()
        && !matches!(
            (previous, current),
            (Some(Overlay::NoteBadge(_)), Overlay::PostmanNote)
        )
    {
        pending_badges.push_front(badge);
    }
}

#[derive(Clone, Copy)]
enum Launch {
    Fresh,
    Suppressed,
    Normal,
}

fn notes_on_launch(
    progress: &mut Progress,
    sections: Vec<Section>,
    launch: Launch,
) -> Vec<Section> {
    match launch {
        Launch::Fresh => {
            if let Some(newest) = sections.first() {
                progress.mark_notes_seen(newest.date);
            }
            Vec::new()
        }
        Launch::Suppressed => Vec::new(),
        Launch::Normal => release_notes::unseen(sections, progress.notes_seen_date()),
    }
}

fn welcome_on_launch(progress: &Progress, launch: Launch) -> bool {
    !progress.welcome_seen && !matches!(launch, Launch::Suppressed)
}

fn notes_overlay(has_notes: bool) -> Option<Overlay> {
    has_notes.then_some(Overlay::Notes(0))
}

fn opening_overlay(shows_welcome: bool, has_notes: bool) -> Option<Overlay> {
    if shows_welcome {
        Some(Overlay::Welcome(Step::Arriving))
    } else {
        notes_overlay(has_notes)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WelcomeOutcome {
    Next(Step),
    Done,
}

fn welcome_after_key(step: Step, code: KeyCode, last: usize) -> WelcomeOutcome {
    match code {
        KeyCode::Enter => step
            .advance()
            .map_or(WelcomeOutcome::Done, WelcomeOutcome::Next),
        KeyCode::Esc => WelcomeOutcome::Done,
        KeyCode::Up => WelcomeOutcome::Next(step.scroll(-1, last)),
        KeyCode::Down => WelcomeOutcome::Next(step.scroll(1, last)),
        _ => WelcomeOutcome::Next(step),
    }
}

fn overlay_rows(height: u16) -> usize {
    height.saturating_sub(OVERLAY_MARGIN_ROWS) as usize
}

fn notes_width(renderer: &TerminalRenderer) -> usize {
    overlay::text_width(renderer, views::NOTES_FRAME)
}

fn welcome_width(renderer: &TerminalRenderer) -> usize {
    overlay::text_width(renderer, welcome::NOTE_FRAME)
}

fn story_clock(progress: &Progress, mail_hour: u32) -> StoryClock {
    let start = progress
        .start_date()
        .unwrap_or_else(|| Local::now().date_naive());
    StoryClock::new(start, mail_hour).with_pauses(progress.paused_days.clone())
}

fn story_today(clock: &StoryClock) -> usize {
    clock.day_on(Local::now().naive_local())
}

fn build_timeline(village_count: usize, clock: &StoryClock, today: usize) -> Timeline {
    Timeline::new(&sources::registered(village_count), clock, today)
}

fn is_live_box_full(
    clock: &StoryClock,
    timeline: &Timeline,
    day: usize,
    letters_read: usize,
) -> bool {
    !clock.is_frozen() && pause::is_box_full(timeline, day, letters_read)
}

fn generate_offline_weather(rng: &mut impl rand::Rng) -> WeatherData {
    use rand::RngExt;

    let now = Local::now();
    let hour = now.hour();
    let is_day = (6..18).contains(&hour);

    let conditions = [
        WeatherCondition::Clear,
        WeatherCondition::PartlyCloudy,
        WeatherCondition::Cloudy,
        WeatherCondition::Rain,
    ];

    let condition = conditions[rng.random_range(0..conditions.len())];

    WeatherData {
        condition,
        temperature: rng.random_range(10.0..25.0),
        apparent_temperature: rng.random_range(10.0..25.0),
        humidity: rng.random_range(40.0..80.0),
        precipitation: if condition.is_raining() {
            rng.random_range(1.0..5.0)
        } else {
            0.0
        },
        wind_speed: rng.random_range(5.0..15.0),
        wind_direction: rng.random_range(0.0..360.0),
        cloud_cover: rng.random_range(20.0..80.0),
        pressure: rng.random_range(1000.0..1020.0),
        visibility: Some(10000.0),
        is_day,
        moon_phase: Some(0.5),
        timestamp: now.format("%Y-%m-%dT%H:%M:%S").to_string(),
    }
}

fn simulated_weather(condition: WeatherCondition, is_day: bool) -> WeatherData {
    WeatherData {
        condition,
        temperature: 20.0,
        apparent_temperature: 19.0,
        humidity: 65.0,
        precipitation: if condition.is_raining() { 2.5 } else { 0.0 },
        wind_speed: if condition.is_thunderstorm() {
            45.0
        } else {
            10.0
        },
        wind_direction: 225.0,
        cloud_cover: 50.0,
        pressure: 1013.0,
        visibility: Some(10000.0),
        is_day,
        moon_phase: Some(0.5),
        timestamp: "simulated".to_string(),
    }
}

fn apply_simulated_weather(
    state: &mut AppState,
    animations: &mut AnimationManager,
    weather: WeatherData,
) {
    animations.update_rain_intensity(weather.condition.rain_intensity());
    animations.update_snow_intensity(weather.condition.snow_intensity());
    animations.update_fog_intensity(weather.condition.fog_intensity());
    animations.update_wind(weather.wind_speed as f32, weather.wind_direction as f32);
    state.update_weather(weather);
}

fn apply_demo_scene(state: &mut AppState, animations: &mut AnimationManager, demo: &Demo) {
    let scene = demo.current();
    apply_simulated_weather(
        state,
        animations,
        simulated_weather(scene.condition, scene.is_day),
    );
    state.set_rainbow(scene.rainbow.then(|| Instant::now() + SCENE_DURATION));
    state.set_demo_label(Some(demo.label()));
}

pub struct SimulationOptions {
    pub condition: Option<String>,
    pub night: bool,
    pub holiday: Option<Holiday>,
    pub demo: bool,
    pub day: Option<NonZeroU16>,
}

#[derive(Clone, Copy)]
enum Overlay {
    Letter(usize),
    Badges,
    Album(usize),
    Bottle(usize),
    Help,
    Index(usize),
    NewBadge(&'static Badge),
    NoteBadge(&'static Badge),
    Notes(usize),
    PostmanNote,
    Welcome(Step),
}

#[derive(Default)]
struct StarWatch {
    seen: u32,
    was_visible: bool,
}

impl StarWatch {
    fn observe(
        &mut self,
        is_visible: bool,
        meteor_shower: Option<MeteorShower>,
    ) -> Option<&'static str> {
        if is_visible && !self.was_visible && meteor_shower.is_none() {
            self.seen += 1;
        }
        self.was_visible = is_visible;

        is_visible.then_some(match meteor_shower {
            Some(MeteorShower::Perseids) => "perseids",
            Some(MeteorShower::Geminids) => "geminids",
            None => "shooting_star",
        })
    }
}

pub struct App {
    state: AppState,
    animations: AnimationManager,
    scene: WorldScene,
    weather_receiver: mpsc::Receiver<Result<WeatherData, WeatherError>>,
    hide_hud: bool,
    simulated_holiday: Option<Holiday>,
    demo: Option<Demo>,
    progress: Progress,
    letters: Vec<Letter>,
    clock: StoryClock,
    timeline: Timeline,
    release_notes: Vec<Section>,
    overlay: Option<Overlay>,
    is_simulated: bool,
    awards_badges: bool,
    toasts: VecDeque<String>,
    toast_until: Option<Instant>,
    pending_badges: VecDeque<&'static Badge>,
    session_start: Instant,
    star_watch: StarWatch,
    rain_receiver: Option<mpsc::Receiver<Option<RainAlert>>>,
    rain_alert: Option<RainAlert>,
    riddle_input: Option<String>,
    remote_sender: mpsc::Sender<Reply>,
    remote_receiver: mpsc::Receiver<Reply>,
    pending_requests: BTreeSet<Request>,
    frame_count: u32,
    timeline_day: usize,
}

impl App {
    pub fn new(
        config: &Config,
        simulation: SimulationOptions,
        show_leaves: bool,
        term_width: u16,
        term_height: u16,
    ) -> Self {
        let is_simulated =
            simulation.demo || simulation.condition.is_some() || simulation.holiday.is_some();
        let location = WeatherLocation {
            latitude: config.location.latitude,
            longitude: config.location.longitude,
            elevation: None,
        };

        let mut state = AppState::new(
            location,
            config.location.city.clone(),
            config.location.display,
            config.location.hide,
            config.units,
        );
        let mut animations = AnimationManager::new(term_width, term_height, show_leaves);
        animations.set_mail_hour(config.mail.hour);
        animations.set_story_frozen(simulation.day.is_some());
        let scene = WorldScene::new(term_width, term_height);

        let (tx, rx) = mpsc::channel(1);

        let demo = simulation.demo.then(Demo::new);

        if let Some(demo) = &demo {
            apply_demo_scene(&mut state, &mut animations, demo);
        } else if let Some(ref condition_str) = simulation.condition {
            let simulated_condition =
                condition_str
                    .parse::<WeatherCondition>()
                    .unwrap_or_else(|e| {
                        eprintln!("{}", e);
                        WeatherCondition::Clear
                    });

            apply_simulated_weather(
                &mut state,
                &mut animations,
                simulated_weather(simulated_condition, !simulation.night),
            );
        } else {
            let provider = Arc::new(OpenMeteoProvider::new());
            let weather_client = WeatherClient::new(provider, REFRESH_INTERVAL);
            let units = config.units;

            tokio::spawn(async move {
                loop {
                    let result = weather_client.get_current_weather(&location, &units).await;
                    if tx.send(result).await.is_err() {
                        break;
                    }
                    tokio::time::sleep(REFRESH_INTERVAL).await;
                }
            });
        }

        let launch = if simulation.day.is_some() {
            Launch::Suppressed
        } else if !Progress::exists() {
            Launch::Fresh
        } else if is_simulated || simulation.night || config.silent {
            Launch::Suppressed
        } else {
            Launch::Normal
        };
        let (mut progress, letters, clock) = match simulation.day {
            Some(day) => {
                let clock = StoryClock::frozen(Local::now().date_naive(), config.mail.hour, day);
                (
                    Progress::in_memory(clock.date_of(1)),
                    letters::read(),
                    clock,
                )
            }
            None => {
                let progress = Progress::load();
                let clock = story_clock(&progress, config.mail.hour);
                (progress, letters::load(), clock)
            }
        };
        let release_notes = notes_on_launch(&mut progress, release_notes::load(), launch);
        let shows_welcome = welcome_on_launch(&progress, launch);
        if shows_welcome {
            animations.postman_visit();
        }
        if progress.record_visit() {
            progress.save();
        }
        let rain_receiver =
            (!is_simulated).then(|| rain_alert::spawn(location.latitude, location.longitude));
        let timeline_day = story_today(&clock);
        let timeline = build_timeline(letters.len(), &clock, timeline_day);
        let (remote_sender, remote_receiver) = mpsc::channel(REMOTE_CHANNEL_SIZE);
        animations.set_unlocks(unlocks_on(&timeline, timeline_day, &progress.answers));

        let mut app = Self {
            state,
            animations,
            scene,
            weather_receiver: rx,
            hide_hud: config.hide_hud,
            simulated_holiday: simulation.holiday,
            demo,
            progress,
            letters,
            clock,
            timeline,
            overlay: opening_overlay(shows_welcome, !release_notes.is_empty()),
            release_notes,
            is_simulated: is_simulated || simulation.night,
            awards_badges: simulation.day.is_none(),
            toasts: VecDeque::new(),
            toast_until: None,
            pending_badges: VecDeque::new(),
            session_start: Instant::now(),
            star_watch: StarWatch::default(),
            rain_receiver,
            rain_alert: None,
            riddle_input: None,
            remote_sender,
            remote_receiver,
            pending_requests: BTreeSet::new(),
            frame_count: 0,
            timeline_day,
        };
        app.refresh_timeline();
        app
    }

    fn refresh_progress(&mut self) {
        if self.progress.record_visit() {
            self.progress.save();
        }
        self.refresh_timeline();
        self.state.set_story_day(self.timeline_day);
        self.request_remote_weather();
        self.receive_remote_weather();
        self.animations.set_finds_remaining(
            self.progress.found_objects.len() < OBJECTS.len(),
            self.progress.bottles.len() < BOTTLE_MESSAGES.len(),
        );
        let hint = match self.animations.visible_finds() {
            (true, true) => Some("Objet et bouteille ! [r]"),
            (true, false) => Some("Objet ! [r]"),
            (false, true) => Some("Bouteille ! [r]"),
            (false, false) => None,
        };
        self.state.set_finds_hint(hint);
        let unread = self
            .available_letters()
            .saturating_sub(self.progress.letters_read);
        let is_mail_waiting = unread > 0 && !self.hides_mail();
        self.animations.set_mail_waiting(is_mail_waiting);
        self.animations
            .set_story_frozen(self.clock.is_frozen() || self.is_box_full());
        let earned = badges::BADGES
            .iter()
            .filter(|badge| self.progress.badges.contains(badge.id))
            .count();
        self.state
            .set_progress_summary(is_mail_waiting, earned, badges::BADGES.len());
    }

    fn collect_badges(&mut self) {
        let mut found = Vec::new();
        let hour = Local::now().hour();
        let milestones = [
            ((0..5).contains(&hour), "night_owl"),
            ((5..7).contains(&hour), "early_bird"),
            (self.session_start.elapsed() >= PATIENCE, "patience"),
            (self.progress.streak >= 3, "streak_3"),
            (self.progress.streak >= 7, "streak_7"),
            (self.progress.streak >= 30, "streak_30"),
            (self.has_read_all_letters(), "all_letters"),
            (!self.progress.riddles_solved.is_empty(), "riddle_1"),
            (self.progress.riddles_solved.len() >= 10, "riddle_10"),
            (
                self.progress.found_objects.len() >= OBJECTS.len(),
                "all_objects",
            ),
            (!self.progress.bottles.is_empty(), "first_bottle"),
        ];
        found.extend(
            milestones
                .iter()
                .filter(|(reached, _)| *reached)
                .map(|(_, id)| *id),
        );

        if !self.is_simulated {
            if let Some(weather) = &self.state.current_weather
                && !self.state.is_offline
            {
                found.push(badges::weather_badge(weather.condition));
            }

            let holiday = self.state.holiday;
            let sightings = self.animations.sightings(&self.state.weather_conditions);
            found.extend(
                self.star_watch
                    .observe(sightings.shooting_star, holiday.meteor_shower),
            );

            let has_all_weathers =
                badges::weather_badges().all(|badge| self.progress.badges.contains(badge.id));
            let events = [
                (holiday.is_christmas, "christmas"),
                (holiday.is_national_day, "national_day"),
                (holiday.is_halloween, "halloween"),
                (holiday.new_year.is_some(), "new_year"),
                (holiday.is_easter, "easter"),
                (holiday.is_music_day, "music_day"),
                (self.state.shows_rainbow(), "rainbow"),
                (sightings.postman, "postman"),
                (sightings.snowman, "snowman"),
                (self.star_watch.seen >= STARS_FOR_HUNTER, "star_hunter"),
                (has_all_weathers, "full_weather"),
            ];
            found.extend(events.iter().filter(|(seen, _)| *seen).map(|(_, id)| *id));
        }

        self.award(found);
    }

    fn award(&mut self, ids: Vec<&'static str>) {
        if !self.awards_badges {
            return;
        }
        let new_badges = earn(&mut self.progress.badges, ids);
        if !new_badges.is_empty() {
            self.pending_badges.extend(new_badges);
            self.progress.save();
        }
    }

    fn open_pending_badge(&mut self) {
        self.overlay = next_overlay(self.overlay, &mut self.pending_badges);
    }

    fn hides_mail(&self) -> bool {
        matches!(self.overlay, Some(Overlay::Welcome(step)) if step.hides_mail())
    }

    fn update_welcome(&mut self, term_width: u16, term_height: u16) {
        if let Some(Overlay::Welcome(step)) = self.overlay {
            let has_arrived = !self.animations.is_postman_arriving(term_width, term_height);
            self.overlay = Some(Overlay::Welcome(step.tick(has_arrived)));
        }
    }

    fn press_welcome_key(&mut self, code: KeyCode, rows: usize, width: usize) {
        if let Some(Overlay::Welcome(step)) = self.overlay {
            match welcome_after_key(step, code, welcome::scroll_limit(rows, width)) {
                WelcomeOutcome::Next(step) => self.overlay = Some(Overlay::Welcome(step)),
                WelcomeOutcome::Done => self.end_welcome(),
            }
        }
    }

    fn end_welcome(&mut self) {
        self.progress.welcome_seen = true;
        self.progress.save();
        self.animations.postman_leave();
        self.overlay = notes_overlay(!self.release_notes.is_empty());
    }

    fn has_read_all_letters(&self) -> bool {
        let village = count_read(&self.timeline, &self.progress, |entry| {
            matches!(entry, Entry::Main(_))
        });
        let strange = count_read(&self.timeline, &self.progress, |entry| {
            matches!(entry, Entry::Creepy(_))
        });
        village >= self.letters.len() && strange >= letters::CREEPY_LETTER_COUNT
    }

    fn unlocks(&self) -> Unlocks {
        unlocks_on(&self.timeline, self.timeline_day, &self.progress.answers)
    }

    fn refresh_unlocks(&mut self) {
        self.animations.set_unlocks(self.unlocks());
    }

    fn is_box_full(&self) -> bool {
        is_live_box_full(
            &self.clock,
            &self.timeline,
            self.timeline_day,
            self.progress.letters_read,
        )
    }

    fn available_letters(&self) -> usize {
        self.timeline.delivered_by(story_today(&self.clock))
    }

    fn village_letter_number(&self, index: usize) -> Option<usize> {
        match self.timeline.mail_at(index)? {
            (_, Entry::Main(main)) => Some(main + 1),
            _ => None,
        }
    }

    fn answer_letter(&mut self, option: u8) {
        let Some(Overlay::Letter(index)) = self.overlay else {
            return;
        };
        let Some(choice) = self
            .village_letter_number(index)
            .and_then(letters::choice_for)
        else {
            return;
        };
        if self.progress.answers.contains_key(choice.id) {
            return;
        }
        self.progress.answers.insert(choice.id.to_string(), option);
        self.progress.save();
        self.refresh_unlocks();
    }

    fn refresh_rain_alert(&mut self) {
        if let Some(receiver) = &mut self.rain_receiver {
            while let Ok(alert) = receiver.try_recv() {
                self.rain_alert = alert;
            }
        }
        let conditions = &self.state.weather_conditions;
        let is_wet = conditions.is_raining || conditions.is_thunderstorm;
        let message = self
            .rain_alert
            .as_ref()
            .filter(|_| !is_wet)
            .map(RainAlert::message);
        self.state.set_umbrella(message);
    }

    fn press_mail(&mut self) {
        let is_note_waiting = postman_note::is_postman_note_waiting(
            self.is_box_full(),
            self.progress.postman_note_day,
            self.timeline_day,
        );
        let open_letter = match self.overlay {
            Some(Overlay::Letter(index)) => Some(index),
            _ => None,
        };
        match postman_note::next_mail(
            open_letter,
            self.available_letters(),
            self.progress.letters_read,
            is_note_waiting,
        ) {
            Some(MailStep::PostmanNote) => self.open_postman_note(),
            Some(MailStep::Letter(index)) => self.open_letter(index),
            Some(MailStep::Close) => self.overlay = None,
            None => {}
        }
    }

    fn open_postman_note(&mut self) {
        self.overlay = Some(Overlay::PostmanNote);
        self.progress.postman_note_day = Some(self.timeline_day);
        self.progress.save();
        self.award(vec!["summer_holidays"]);
    }

    fn browse_letters(&mut self, step: isize) {
        if let Some(Overlay::Letter(index)) = self.overlay {
            let available = self.available_letters();
            let target = index
                .saturating_add_signed(step)
                .min(available.saturating_sub(1));
            self.open_letter(target);
        }
    }

    fn open_letter(&mut self, index: usize) {
        self.overlay = Some(Overlay::Letter(index));
        if content::is_night_hour(Local::now().hour()) {
            self.award(vec!["night_reading"]);
        }
        if index + 1 > self.progress.letters_read {
            self.progress.letters_read = index + 1;
            self.progress.save();
        }
    }

    fn update_toasts(&mut self) {
        let now = Instant::now();
        if self.toast_until.is_some_and(|until| now >= until) {
            self.toasts.pop_front();
            self.toast_until = None;
        }
        if self.toast_until.is_none() && !self.toasts.is_empty() {
            self.toast_until = Some(now + TOAST_DURATION);
        }
    }

    fn render_overlays(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        if let Some(message) = self.toasts.front()
            && self.toast_until.is_some()
        {
            let length = message.chars().count() as u16;
            let x = renderer.get_size().0.saturating_sub(length) / 2;
            renderer.render_line_colored(x, 0, message, crossterm::style::Color::Yellow)?;
        }

        let view = match self.overlay {
            Some(Overlay::Letter(index)) => {
                let delivered = self.timeline.first_mail(self.available_letters());
                let Some(view) = content::entry_view(delivered, index, &self.read_context()) else {
                    return Ok(());
                };
                view
            }
            Some(Overlay::Album(page)) => {
                let mut pages = self.album_pages();
                let page = page.min(pages.len().saturating_sub(1));
                pages.swap_remove(page)
            }
            Some(Overlay::Bottle(index)) => views::bottle_view(index),
            Some(Overlay::NewBadge(badge) | Overlay::NoteBadge(badge)) => {
                views::badge_card_view(badge)
            }
            Some(Overlay::PostmanNote) => views::postman_note_view(self.back_to_school()),
            Some(Overlay::Notes(first)) => views::notes_view(
                &self.release_notes,
                first,
                overlay_rows(renderer.get_size().1),
                notes_width(renderer),
            ),
            Some(Overlay::Index(selected)) => {
                let rows = overlay_rows(renderer.get_size().1);
                if self.is_box_full() {
                    views::full_box_index_view(
                        self.read_entries(),
                        selected,
                        rows,
                        overlay::text_width(renderer, views::INDEX_FRAME),
                        &self.read_context(),
                        self.back_to_school(),
                    )
                } else {
                    views::index_view(self.read_entries(), selected, rows, &self.read_context())
                }
            }
            Some(Overlay::Help) => views::help_view(
                &self.unlocks(),
                count_read(&self.timeline, &self.progress, |entry| {
                    matches!(entry, Entry::Gazette(_))
                }) > 0,
            ),
            Some(Overlay::Badges) => {
                return overlay::render_badges(
                    renderer,
                    &self.progress.badges,
                    self.progress.streak,
                );
            }
            Some(Overlay::Welcome(step)) => return self.render_welcome(renderer, step),
            None => return Ok(()),
        };
        overlay::render_view(renderer, &view)
    }

    fn render_welcome(&self, renderer: &mut TerminalRenderer, step: Step) -> io::Result<()> {
        let (width, height) = renderer.get_size();
        let speaker = self.animations.postman_head(width, height);
        let rows = overlay_rows(height);
        let text_width = welcome_width(renderer);
        match step {
            Step::Arriving => Ok(()),
            Step::Greeting => {
                overlay::render_bubble(renderer, welcome::ARRIVAL, welcome::FOOTER, speaker)
            }
            Step::Unfolding(frame) => {
                overlay::render_view(renderer, &welcome::unfolding_view(frame, rows, text_width))
            }
            Step::Reading(first) => {
                overlay::render_view(renderer, &welcome::note_view(first, rows, text_width))
            }
            Step::Farewell => {
                overlay::render_bubble(renderer, welcome::DEPARTURE, welcome::FOOTER, speaker)
            }
        }
    }

    fn read_context(&self) -> ReadContext<'_> {
        ReadContext {
            letters: &self.letters,
            progress: &self.progress,
            clock: &self.clock,
            reading_hour: Local::now().hour(),
            riddle_input: self.riddle_input.as_deref(),
        }
    }

    fn read_entries(&self) -> &[(usize, Entry)] {
        self.timeline
            .first_mail(self.progress.letters_read.min(self.available_letters()))
    }

    fn album_pages(&self) -> Vec<overlay::View> {
        views::album_pages(self.read_entries(), &self.progress, &self.unlocks())
    }

    fn back_to_school(&self) -> NaiveDate {
        self.clock.next_rollover(Local::now().naive_local())
    }

    fn hold_full_box(&mut self) {
        if self.clock.is_frozen() {
            return;
        }
        let today = self.clock.mail_date(Local::now().naive_local());
        if let Some(checked_on) = pause::hold_since(
            &mut self.clock,
            &self.timeline,
            self.progress.letters_read,
            self.progress.pause_checked_date(),
            today,
        ) {
            self.progress.paused_days = self.clock.pauses().clone();
            self.progress.mark_pause_checked(checked_on);
            self.progress.save();
        }
    }

    fn refresh_timeline(&mut self) {
        self.hold_full_box();
        let today = story_today(&self.clock);
        if today != self.timeline_day {
            self.timeline = build_timeline(self.letters.len(), &self.clock, today);
            self.timeline_day = today;
            self.refresh_unlocks();
        }
    }

    fn request_remote_weather(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);
        if self.frame_count % REMOTE_CHECK_FRAMES != 1 {
            return;
        }

        let available = self.available_letters();
        let mut requests = Vec::new();
        for (day, entry) in self.timeline.mail().take(available) {
            let request = match entry {
                Entry::Postcard(index) if !self.progress.postcard_weather.contains_key(&index) => {
                    let city = &CITIES[index % CITIES.len()];
                    (Request::Postcard(index), city.latitude, city.longitude)
                }
                Entry::Gazette(week) if !self.progress.gazette_weather.contains_key(&week) => (
                    Request::WeekForecast {
                        week,
                        monday: self.clock.date_of(day),
                    },
                    self.state.location.latitude,
                    self.state.location.longitude,
                ),
                _ => continue,
            };
            requests.push(request);
        }

        for (request, latitude, longitude) in requests {
            if self.pending_requests.insert(request) {
                remote_weather::spawn(request, latitude, longitude, self.remote_sender.clone());
            }
        }
    }

    fn receive_remote_weather(&mut self) {
        let mut has_changed = false;
        while let Ok(Reply { request, text }) = self.remote_receiver.try_recv() {
            self.pending_requests.remove(&request);
            match request {
                Request::Postcard(index) => self.progress.postcard_weather.insert(index, text),
                Request::WeekForecast { week, .. } => {
                    self.progress.gazette_weather.insert(week, text)
                }
            };
            has_changed = true;
        }
        if has_changed {
            self.progress.save();
        }
    }

    fn pick_up(&mut self) {
        if self.animations.take_object()
            && let Some(index) =
                (0..OBJECTS.len()).find(|index| !self.progress.found_objects.contains(index))
        {
            self.progress.found_objects.push(index);
            self.toasts
                .push_back(format!("* Objet trouvé : {} ! *", OBJECTS[index]));
            self.progress.save();
        }

        if self.animations.take_bottle()
            && let Some(index) =
                (0..BOTTLE_MESSAGES.len()).find(|index| !self.progress.bottles.contains(index))
        {
            self.progress.bottles.push(index);
            self.overlay = Some(Overlay::Bottle(index));
            self.progress.save();
        }
    }

    fn gazette_week(&self) -> Option<usize> {
        match self.overlay {
            Some(Overlay::Letter(index)) => match self.timeline.mail_at(index)?.1 {
                Entry::Gazette(week) => Some(week),
                _ => None,
            },
            _ => None,
        }
    }

    fn start_riddle(&mut self) {
        if let Some(week) = self.gazette_week()
            && !self.progress.riddles_solved.contains(&week)
        {
            self.riddle_input = Some(String::new());
        }
    }

    fn handle_riddle_key(&mut self, code: KeyCode) {
        let Some(input) = self.riddle_input.as_mut() else {
            return;
        };
        match code {
            KeyCode::Char(ch) if input.chars().count() < MAX_ANSWER_LENGTH => input.push(ch),
            KeyCode::Backspace => {
                input.pop();
            }
            KeyCode::Esc => self.riddle_input = None,
            KeyCode::Enter => self.submit_riddle(),
            _ => {}
        }
    }

    fn submit_riddle(&mut self) {
        let (Some(week), Some(input)) = (self.gazette_week(), self.riddle_input.take()) else {
            return;
        };
        if chronicles::is_right_answer(&RIDDLES[week % RIDDLES.len()], &input) {
            self.progress.riddles_solved.insert(week);
            self.progress.save();
            self.toasts.push_back("* Bonne réponse ! *".to_string());
        } else {
            self.toasts
                .push_back("* Ce n'est pas ça… essaie encore. *".to_string());
        }
    }

    fn toggle_index(&mut self) {
        self.overlay = match self.overlay {
            Some(Overlay::Index(_)) => None,
            _ => Some(Overlay::Index(self.read_entries().len().saturating_sub(1))),
        };
    }

    fn select_in_index(&mut self, step: isize) {
        if let Some(Overlay::Index(selected)) = self.overlay {
            let last = self.read_entries().len().saturating_sub(1);
            self.overlay = Some(Overlay::Index(
                selected.saturating_add_signed(step).min(last),
            ));
        }
    }

    fn scroll(&mut self, step: isize, rows: usize, width: usize) {
        match self.overlay {
            Some(Overlay::Notes(first)) => {
                let last = views::notes_scroll_limit(&self.release_notes, rows, width);
                self.overlay = Some(Overlay::Notes(first.saturating_add_signed(step).min(last)));
            }
            _ => self.select_in_index(step),
        }
    }

    fn mark_notes_seen_when_left(&mut self, previous: Option<Overlay>) {
        if matches!(previous, Some(Overlay::Notes(_)))
            && !matches!(self.overlay, Some(Overlay::Notes(_)))
            && let Some(newest) = self.release_notes.first()
        {
            self.progress.mark_notes_seen(newest.date);
            self.progress.save();
        }
    }

    fn reread_selected(&mut self) {
        if let Some(Overlay::Index(selected)) = self.overlay
            && selected < self.read_entries().len()
        {
            self.open_letter(selected);
        }
    }

    fn press_enter(&mut self) {
        match self.overlay {
            Some(
                Overlay::NewBadge(_)
                | Overlay::NoteBadge(_)
                | Overlay::Notes(_)
                | Overlay::PostmanNote,
            ) => self.overlay = closed_overlay(self.overlay),
            _ => self.reread_selected(),
        }
    }

    fn toggle_help(&mut self) {
        self.overlay = match self.overlay {
            Some(Overlay::Help) => None,
            _ => Some(Overlay::Help),
        };
    }

    fn toggle_album(&mut self) {
        self.overlay = match self.overlay {
            Some(Overlay::Album(_)) => None,
            _ => Some(Overlay::Album(0)),
        };
    }

    fn browse(&mut self, step: isize) {
        match self.overlay {
            Some(Overlay::Album(page)) => {
                let last = self.album_pages().len().saturating_sub(1);
                self.overlay = Some(Overlay::Album(page.saturating_add_signed(step).min(last)));
            }
            Some(Overlay::Letter(_)) => self.browse_letters(step),
            _ => {}
        }
    }

    pub async fn run(&mut self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        let mut rng = rand::rng();
        loop {
            if let Some(demo) = &mut self.demo {
                if demo.advance_if_due() {
                    apply_demo_scene(&mut self.state, &mut self.animations, demo);
                }
            }

            self.state.holiday = match &self.demo {
                Some(demo) => demo.current().holiday,
                None => self.simulated_holiday.unwrap_or_else(Holiday::today),
            };
            self.animations.set_season(self.state.holiday.season);
            self.refresh_progress();

            if let Ok(result) = self.weather_receiver.try_recv() {
                match result {
                    Ok(weather) => {
                        let rain_intensity = weather.condition.rain_intensity();
                        let snow_intensity = weather.condition.snow_intensity();
                        let fog_intensity = weather.condition.fog_intensity();
                        let wind_speed = weather.wind_speed;
                        let wind_direction = weather.wind_direction;

                        self.state.update_weather(weather);
                        self.animations.update_rain_intensity(rain_intensity);
                        self.animations.update_snow_intensity(snow_intensity);
                        self.animations.update_fog_intensity(fog_intensity);
                        self.animations
                            .update_wind(wind_speed as f32, wind_direction as f32);
                    }
                    Err(error) => {
                        let _error_msg = match &error {
                            WeatherError::Network(net_err) => net_err.user_friendly_message(),
                            _ => format!("Failed to fetch weather: {}", error),
                        };

                        if self.state.current_weather.is_none() {
                            let offline_weather = generate_offline_weather(&mut rng);
                            let rain_intensity = offline_weather.condition.rain_intensity();
                            let snow_intensity = offline_weather.condition.snow_intensity();
                            let fog_intensity = offline_weather.condition.fog_intensity();
                            let wind_speed = offline_weather.wind_speed;
                            let wind_direction = offline_weather.wind_direction;

                            self.state.update_weather(offline_weather);
                            self.state.set_offline_mode(true);
                            self.animations.update_rain_intensity(rain_intensity);
                            self.animations.update_snow_intensity(snow_intensity);
                            self.animations.update_fog_intensity(fog_intensity);
                            self.animations
                                .update_wind(wind_speed as f32, wind_direction as f32);
                        } else {
                            self.state.set_offline_mode(true);
                        }
                    }
                }
            }

            renderer.clear()?;

            let (term_width, term_height) = renderer.get_size();

            self.animations.render_background(
                renderer,
                &self.state.weather_conditions,
                &self.state,
                term_width,
                term_height,
                &mut rng,
            )?;

            self.scene.render(
                renderer,
                &self.state.weather_conditions,
                &self.state.holiday,
            )?;

            self.animations.render_chimney_smoke(
                renderer,
                &self.state.weather_conditions,
                term_width,
                term_height,
            )?;

            self.animations.render_garden(
                renderer,
                &self.state.weather_conditions,
                &self.state.holiday,
                term_width,
                term_height,
                &mut rng,
            )?;

            self.animations.render_foreground(
                renderer,
                &self.state.weather_conditions,
                term_width,
                term_height,
                &mut rng,
            )?;

            self.collect_badges();
            self.open_pending_badge();
            self.update_welcome(term_width, term_height);
            self.update_toasts();
            self.refresh_rain_alert();
            self.state.update_loading_animation();
            self.state.update_cached_info();

            if !self.hide_hud {
                renderer.render_line_colored(
                    2,
                    1,
                    &self.state.cached_weather_info,
                    crossterm::style::Color::Cyan,
                )?;
            }

            self.render_overlays(renderer)?;
            renderer.flush()?;

            if event::poll(FRAME_DURATION)? {
                match event::read()? {
                    Event::Resize(width, height) => {
                        renderer.manual_resize(width, height)?;
                    }
                    Event::Key(key_event)
                        if self.riddle_input.is_some()
                            && !key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        self.handle_riddle_key(key_event.code);
                    }
                    Event::Key(key_event) => {
                        let previous = self.overlay;
                        let rows = overlay_rows(renderer.get_size().1);
                        let width = notes_width(renderer);
                        match key_event.code {
                            KeyCode::Char('q') | KeyCode::Char('Q') => break,
                            KeyCode::Char('c')
                                if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                            {
                                break;
                            }
                            _ if matches!(self.overlay, Some(Overlay::Welcome(_))) => {
                                self.press_welcome_key(
                                    key_event.code,
                                    rows,
                                    welcome_width(renderer),
                                );
                            }
                            KeyCode::Char('c') | KeyCode::Char('C') => self.press_mail(),
                            KeyCode::Char('b') | KeyCode::Char('B') => {
                                self.overlay = match self.overlay {
                                    Some(Overlay::Badges) => None,
                                    _ => Some(Overlay::Badges),
                                };
                            }
                            KeyCode::Char('1') => self.answer_letter(1),
                            KeyCode::Char('2') => self.answer_letter(2),
                            KeyCode::Char('a') | KeyCode::Char('A') => self.toggle_album(),
                            KeyCode::Char('h') | KeyCode::Char('H') => self.toggle_help(),
                            KeyCode::Char('l') | KeyCode::Char('L') => self.toggle_index(),
                            KeyCode::Up => self.scroll(-1, rows, width),
                            KeyCode::Down => self.scroll(1, rows, width),
                            KeyCode::Enter => self.press_enter(),
                            KeyCode::Char('r') | KeyCode::Char('R') => self.pick_up(),
                            KeyCode::Char('e') | KeyCode::Char('E') => self.start_riddle(),
                            KeyCode::Esc => self.overlay = closed_overlay(self.overlay),
                            KeyCode::Left => self.browse(-1),
                            KeyCode::Right => self.browse(1),
                            _ => {}
                        }
                        requeue_replaced_badge(previous, self.overlay, &mut self.pending_badges);
                        self.mark_notes_seen_when_left(previous);
                    }
                    _ => {}
                }
            }

            let (term_width, term_height) = renderer.get_size();
            self.scene.update_size(term_width, term_height);

            self.animations
                .update_sunny_animation(&self.state.weather_conditions);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LocationDisplay;
    use crate::progress::DATE_FORMAT;
    use crate::weather::WeatherUnits;
    use chrono::{NaiveDate, NaiveDateTime};

    fn card_id(overlay: Option<Overlay>) -> Option<&'static str> {
        badge_card(overlay).map(|badge| badge.id)
    }

    #[test]
    fn test_new_badges_open_one_card_each_in_award_order() {
        let mut earned = BTreeSet::from(["clear".to_string()]);
        let mut pending: VecDeque<_> =
            earn(&mut earned, vec!["rain", "clear", "patience", "rain"]).into();
        assert_eq!(pending.len(), 2);

        let overlay = next_overlay(None, &mut pending);
        assert_eq!(card_id(overlay), Some("rain"));
        let overlay = next_overlay(overlay, &mut pending);
        assert_eq!(card_id(overlay), Some("rain"));
        assert_eq!(card_id(next_overlay(None, &mut pending)), Some("patience"));
        assert!(next_overlay(None, &mut pending).is_none());
    }

    fn notes() -> Vec<Section> {
        release_notes::parse("## 2026-10-26\n- Neuve\n## 2026-10-12\n- Ancienne\n")
    }

    #[test]
    fn test_fresh_install_shows_no_notes_and_marks_newest_seen() {
        let mut progress = Progress::default();
        assert!(notes_on_launch(&mut progress, notes(), Launch::Fresh).is_empty());
        assert_eq!(progress.notes_seen_on.as_deref(), Some("2026-10-26"));
        assert!(notes_on_launch(&mut progress, notes(), Launch::Normal).is_empty());
    }

    #[test]
    fn test_suppressed_launch_marks_seen_only_on_fresh_install() {
        let mut fresh = Progress::default();
        assert!(notes_on_launch(&mut fresh, notes(), Launch::Fresh).is_empty());
        assert_eq!(fresh.notes_seen_on.as_deref(), Some("2026-10-26"));

        let mut existing = Progress::default();
        assert!(notes_on_launch(&mut existing, notes(), Launch::Suppressed).is_empty());
        assert_eq!(existing.notes_seen_on, None);
        assert_eq!(
            notes_on_launch(&mut existing, notes(), Launch::Normal),
            notes()[..1]
        );
    }

    #[test]
    fn test_old_save_without_seen_date_shows_newest_notes_only() {
        let mut progress = Progress::default();
        let shown = notes_on_launch(&mut progress, notes(), Launch::Normal);
        assert_eq!(shown, notes()[..1]);
        assert_eq!(progress.notes_seen_on, None);
    }

    #[test]
    fn test_notes_after_seen_date_show_newest_first() {
        let mut progress = Progress::default();
        progress.notes_seen_on = Some("2026-10-01".to_string());
        assert_eq!(
            notes_on_launch(&mut progress, notes(), Launch::Normal),
            notes()
        );

        progress.mark_notes_seen(notes()[0].date);
        assert!(notes_on_launch(&mut progress, notes(), Launch::Normal).is_empty());
    }

    #[test]
    fn test_badge_card_waits_for_the_open_window_to_close() {
        let mut pending: VecDeque<_> = earn(&mut BTreeSet::new(), vec!["night_reading"]).into();

        let overlay = next_overlay(Some(Overlay::Letter(3)), &mut pending);
        assert!(matches!(overlay, Some(Overlay::Letter(3))));
        assert_eq!(pending.len(), 1);
        assert_eq!(
            card_id(next_overlay(None, &mut pending)),
            Some("night_reading")
        );
    }

    #[test]
    fn test_badge_card_replaced_by_another_window_comes_back_first() {
        let mut pending: VecDeque<_> = earn(&mut BTreeSet::new(), vec!["rain", "patience"]).into();
        let card = next_overlay(None, &mut pending);

        requeue_replaced_badge(card, Some(Overlay::Help), &mut pending);
        assert_eq!(pending.len(), 2);
        assert_eq!(card_id(next_overlay(None, &mut pending)), Some("rain"));

        requeue_replaced_badge(card, card, &mut pending);
        requeue_replaced_badge(card, None, &mut pending);
        assert_eq!(pending.len(), 1);
    }

    #[test]
    fn test_postman_note_badge_opens_over_the_note_then_returns_to_it() {
        let mut pending: VecDeque<_> = earn(&mut BTreeSet::new(), vec!["summer_holidays"]).into();
        let card = next_overlay(Some(Overlay::PostmanNote), &mut pending);
        assert!(matches!(card, Some(Overlay::NoteBadge(_))));
        assert_eq!(card_id(card), Some("summer_holidays"));

        let closed = closed_overlay(card);
        assert!(matches!(closed, Some(Overlay::PostmanNote)));
        requeue_replaced_badge(card, closed, &mut pending);
        assert!(pending.is_empty());
        assert!(matches!(
            next_overlay(closed, &mut pending),
            Some(Overlay::PostmanNote)
        ));
        assert!(closed_overlay(closed).is_none());
    }

    #[test]
    fn test_welcome_shows_until_seen_except_on_suppressed_launch() {
        let mut progress = Progress::default();
        assert!(welcome_on_launch(&progress, Launch::Fresh));
        assert!(welcome_on_launch(&progress, Launch::Normal));
        assert!(!welcome_on_launch(&progress, Launch::Suppressed));

        progress.welcome_seen = true;
        assert!(!welcome_on_launch(&progress, Launch::Fresh));
        assert!(!welcome_on_launch(&progress, Launch::Normal));
    }

    #[test]
    fn test_notes_wait_for_the_welcome_and_badge_cards_too() {
        assert!(matches!(
            opening_overlay(true, true),
            Some(Overlay::Welcome(Step::Arriving))
        ));
        assert!(matches!(
            opening_overlay(false, true),
            Some(Overlay::Notes(0))
        ));
        assert!(opening_overlay(false, false).is_none());

        let mut pending: VecDeque<_> = earn(&mut BTreeSet::new(), vec!["rain"]).into();
        let overlay = next_overlay(opening_overlay(true, false), &mut pending);
        assert!(matches!(overlay, Some(Overlay::Welcome(_))));
        assert_eq!(pending.len(), 1);
    }

    #[test]
    fn test_welcome_keys_advance_skip_or_do_nothing() {
        let steps = [
            Step::Arriving,
            Step::Greeting,
            Step::Unfolding(4),
            Step::Reading(1),
            Step::Farewell,
        ];
        let ignored = "clabher12"
            .chars()
            .map(KeyCode::Char)
            .chain([KeyCode::Left, KeyCode::Right]);
        for step in steps {
            assert_eq!(
                welcome_after_key(step, KeyCode::Esc, 5),
                WelcomeOutcome::Done
            );
            for code in ignored.clone() {
                assert_eq!(welcome_after_key(step, code, 5), WelcomeOutcome::Next(step));
            }
        }
        assert_eq!(
            welcome_after_key(Step::Greeting, KeyCode::Enter, 5),
            WelcomeOutcome::Next(Step::Unfolding(0))
        );
        assert_eq!(
            welcome_after_key(Step::Farewell, KeyCode::Enter, 5),
            WelcomeOutcome::Done
        );
        assert_eq!(
            welcome_after_key(Step::Reading(1), KeyCode::Down, 5),
            WelcomeOutcome::Next(Step::Reading(2))
        );
        assert_eq!(
            welcome_after_key(Step::Reading(1), KeyCode::Up, 5),
            WelcomeOutcome::Next(Step::Reading(0))
        );
    }

    const METEOR_SHOWER_BADGES: [(MeteorShower, &str); 2] = [
        (MeteorShower::Perseids, "perseids"),
        (MeteorShower::Geminids, "geminids"),
    ];

    fn watch(
        star_watch: &mut StarWatch,
        frames: &[bool],
        meteor_shower: Option<MeteorShower>,
    ) -> Vec<&'static str> {
        frames
            .iter()
            .filter_map(|&is_visible| star_watch.observe(is_visible, meteor_shower))
            .collect()
    }

    #[test]
    fn test_shooting_star_counts_once_per_star() {
        let mut star_watch = StarWatch::default();
        let badges = watch(&mut star_watch, &[true, true, false, true, false], None);

        assert_eq!(badges, ["shooting_star"; 3]);
        assert_eq!(star_watch.seen, 2);
    }

    #[test]
    fn test_meteor_shower_awards_only_its_badge() {
        for (meteor_shower, badge) in METEOR_SHOWER_BADGES {
            let mut star_watch = StarWatch::default();
            let badges = watch(
                &mut star_watch,
                &[true, false, true, false, true],
                Some(meteor_shower),
            );

            assert_eq!(badges, [badge; 3]);
            assert_eq!(star_watch.seen, 0);
        }
    }

    #[test]
    fn test_meteor_shower_keeps_star_hunter_count() {
        for (meteor_shower, _) in METEOR_SHOWER_BADGES {
            let mut star_watch = StarWatch::default();
            watch(&mut star_watch, &[true, false, true, false], None);
            watch(
                &mut star_watch,
                &[true, false, true, false, true],
                Some(meteor_shower),
            );

            assert_eq!(star_watch.seen, 2);
        }
    }

    const ARRIVAL_DAYS: [(Thing, usize); 23] = [
        (Thing::Bench, 4),
        (Thing::CatBow, 6),
        (Thing::Footprints, 8),
        (Thing::Well, 13),
        (Thing::Bone, 15),
        (Thing::Key, 17),
        (Thing::Treehouse, 22),
        (Thing::OwlFriend, 24),
        (Thing::BlueLantern, 26),
        (Thing::Garland, 31),
        (Thing::Kittens, 33),
        (Thing::XMark, 35),
        (Thing::Balloon, 40),
        (Thing::Ducklings, 42),
        (Thing::FoundObjects, 45),
        (Thing::WeatherVane, 49),
        (Thing::Bunting, 58),
        (Thing::Chest, 59),
        (Thing::WetFootprints, 13),
        (Thing::Silhouette, 21),
        (Thing::WhisperingWell, 33),
        (Thing::Staring, 45),
        (Thing::Bottles, 40),
    ];

    fn shows(unlocks: &Unlocks, thing: Thing) -> bool {
        match thing {
            Thing::Bench => unlocks.bench,
            Thing::CatBow => unlocks.cat_bow,
            Thing::Footprints => unlocks.footprints,
            Thing::Well => unlocks.well,
            Thing::Bone => unlocks.bone,
            Thing::Key => unlocks.key,
            Thing::Treehouse => unlocks.treehouse,
            Thing::OwlFriend => unlocks.owl_friend,
            Thing::BlueLantern => unlocks.blue_lantern,
            Thing::Garland => unlocks.garland,
            Thing::Kittens => unlocks.kittens,
            Thing::XMark => unlocks.x_mark,
            Thing::Balloon => unlocks.balloon,
            Thing::Ducklings => unlocks.ducklings,
            Thing::FoundObjects => unlocks.found_objects,
            Thing::WeatherVane => unlocks.weather_vane,
            Thing::Bunting => unlocks.bunting,
            Thing::Chest => unlocks.chest,
            Thing::WetFootprints => unlocks.wet_footprints,
            Thing::Silhouette => unlocks.silhouette,
            Thing::WhisperingWell => unlocks.whispering_well,
            Thing::Staring => unlocks.staring,
            Thing::Bottles => unlocks.bottles,
        }
    }

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    fn clock() -> StoryClock {
        StoryClock::new(date(2026, 10, 1), 9)
    }

    fn timeline_on(day: usize) -> Timeline {
        build_timeline(letters::defaults().len(), &clock(), day)
    }

    fn unlocks_at(now: NaiveDateTime) -> Unlocks {
        let today = clock().day_on(now);
        unlocks_on(&timeline_on(today), today, &BTreeMap::new())
    }

    fn unlocks_on_day(day: usize) -> Unlocks {
        unlocks_at(clock().date_of(day).and_hms_opt(12, 0, 0).unwrap())
    }

    #[test]
    fn test_every_thing_arrives_on_its_day() {
        let timeline = timeline_on(120);
        assert_eq!(timeline.arrived_by(120).count(), ARRIVAL_DAYS.len());

        for (thing, day) in ARRIVAL_DAYS {
            assert!(
                !shows(&unlocks_on_day(day - 1), thing),
                "{thing:?} on day {}",
                day - 1
            );
            assert!(shows(&unlocks_on_day(day), thing), "{thing:?} on day {day}");
        }
    }

    #[test]
    fn test_arrivals_ignore_reading() {
        let unlocks = unlocks_on(&timeline_on(30), 30, &BTreeMap::new());
        assert!(unlocks.bench && unlocks.well && unlocks.treehouse);
    }

    #[test]
    fn test_bench_arrives_at_mail_hour_rollover() {
        let morning = date(2026, 10, 4);
        assert!(!unlocks_at(morning.and_hms_opt(8, 59, 0).unwrap()).bench);
        assert!(unlocks_at(morning.and_hms_opt(9, 0, 0).unwrap()).bench);
    }

    #[test]
    fn test_choices_keep_default_until_answered() {
        let timeline = timeline_on(60);
        let unanswered = unlocks_on(&timeline, 60, &BTreeMap::new());
        assert!(unanswered.cat_bow && !unanswered.choices.bow_blue);

        let answers = BTreeMap::from([("bow".to_string(), 2)]);
        assert!(unlocks_on(&timeline, 60, &answers).choices.bow_blue);
    }

    #[test]
    fn test_bar_story_day_rolls_over_at_mail_hour() {
        let clock = StoryClock::new(date(2026, 10, 22), 9);
        let morning = date(2026, 10, 29);
        let location = WeatherLocation {
            latitude: 0.0,
            longitude: 0.0,
            elevation: None,
        };
        let mut state = AppState::new(
            location,
            None,
            LocationDisplay::default(),
            true,
            WeatherUnits::default(),
        );

        for (hour, minute, day) in [(8, 59, 7), (9, 0, 8)] {
            state.set_story_day(clock.day_on(morning.and_hms_opt(hour, minute, 0).unwrap()));
            state.update_cached_info();
            assert!(
                state
                    .cached_weather_info
                    .starts_with(&format!("{} | Jour {day} | ", state.displayed_clock))
            );
        }
    }

    #[test]
    fn test_box_full_holds_the_postman_until_a_letter_is_read() {
        let timeline = timeline_on(7);
        assert!(is_live_box_full(&clock(), &timeline, 7, 0));
        assert!(!is_live_box_full(&clock(), &timeline, 7, 1));
        assert!(!is_live_box_full(&clock(), &timeline, 6, 0));
    }

    #[test]
    fn test_postman_note_badge_is_earned_once() {
        let mut earned = BTreeSet::new();
        let first = earn(&mut earned, vec!["summer_holidays"]);
        assert_eq!(
            first.iter().map(|badge| badge.id).collect::<Vec<_>>(),
            ["summer_holidays"]
        );
        assert!(earn(&mut earned, vec!["summer_holidays"]).is_empty());
    }

    #[test]
    fn test_frozen_clock_never_fills_the_box() {
        let frozen = StoryClock::frozen(date(2026, 10, 26), 9, NonZeroU16::new(7).unwrap());
        let timeline = build_timeline(letters::defaults().len(), &frozen, 7);
        assert!(!is_live_box_full(&frozen, &timeline, 7, 0));
    }

    #[test]
    fn test_fresh_progress_starts_on_day_one() {
        let mut progress = Progress::default();
        progress.started_on = Some(Local::now().format(DATE_FORMAT).to_string());
        for mail_hour in [0, 23] {
            assert_eq!(story_today(&story_clock(&progress, mail_hour)), 1);
        }
    }
}
