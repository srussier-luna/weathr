use crate::animation::lore::Unlocks;
use crate::animation_manager::AnimationManager;
use crate::app_state::AppState;
use crate::badges;
use crate::config::Config;
use crate::demo::{Demo, SCENE_DURATION};
use crate::error::WeatherError;
use crate::holiday::Holiday;
use crate::overlay;
use crate::progress::Progress;
use crate::rain_alert::{self, RainAlert};
use crate::remote_weather::{self, Reply, Request};
use crate::render::TerminalRenderer;
use crate::scene::WorldScene;
use crate::story::content::chronicles::{self, BOTTLE_MESSAGES, CITIES, OBJECTS, RIDDLES};
use crate::story::content::letters::{self, Letter};
use crate::story::content::{self, ReadContext};
use crate::story::{Entry, StoryClock, Timeline, sources};
use crate::views;
use crate::weather::{
    OpenMeteoProvider, WeatherClient, WeatherCondition, WeatherData, WeatherLocation,
};
use chrono::{Local, Timelike};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::collections::{BTreeSet, VecDeque};
use std::io;
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
const INDEX_MARGIN_ROWS: u16 = 12;

fn count_read(timeline: &Timeline, progress: &Progress, kind: fn(&Entry) -> bool) -> usize {
    timeline
        .mail()
        .take(progress.letters_read)
        .filter(|(_, entry)| kind(entry))
        .count()
}

fn unlocks_for(timeline: &Timeline, progress: &Progress) -> Unlocks {
    Unlocks::after_reading(
        count_read(timeline, progress, |entry| matches!(entry, Entry::Main(_))),
        count_read(timeline, progress, |entry| {
            matches!(entry, Entry::Creepy(_))
        }),
        count_read(timeline, progress, |entry| {
            matches!(entry, Entry::Archive(_))
        }),
        &progress.answers,
    )
}

fn story_today(clock: &StoryClock) -> usize {
    clock.day_on(Local::now().naive_local())
}

fn build_timeline(village_count: usize, clock: &StoryClock, today: usize) -> Timeline {
    Timeline::new(&sources::registered(village_count), clock, today)
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
}

#[derive(Clone, Copy)]
enum Overlay {
    Letter(usize),
    Badges,
    Album(usize),
    Bottle(usize),
    Help,
    Index(usize),
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
    overlay: Option<Overlay>,
    is_simulated: bool,
    toasts: VecDeque<String>,
    toast_until: Option<Instant>,
    session_start: Instant,
    stars_seen: u32,
    was_star_visible: bool,
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

        let mut progress = Progress::load();
        if progress.record_visit() {
            progress.save();
        }
        let rain_receiver =
            (!is_simulated).then(|| rain_alert::spawn(location.latitude, location.longitude));
        let letters = letters::load();
        let start = progress
            .start_date()
            .unwrap_or_else(|| Local::now().date_naive());
        let clock = StoryClock::new(start, config.mail.hour);
        let timeline_day = story_today(&clock);
        let timeline = build_timeline(letters.len(), &clock, timeline_day);
        let (remote_sender, remote_receiver) = mpsc::channel(REMOTE_CHANNEL_SIZE);
        animations.set_unlocks(unlocks_for(&timeline, &progress));

        Self {
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
            overlay: None,
            is_simulated: is_simulated || simulation.night,
            toasts: VecDeque::new(),
            toast_until: None,
            session_start: Instant::now(),
            stars_seen: 0,
            was_star_visible: false,
            rain_receiver,
            rain_alert: None,
            riddle_input: None,
            remote_sender,
            remote_receiver,
            pending_requests: BTreeSet::new(),
            frame_count: 0,
            timeline_day,
        }
    }

    fn refresh_progress(&mut self) {
        if self.progress.record_visit() {
            self.progress.save();
        }
        self.refresh_timeline();
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
        self.animations.set_mail_waiting(unread > 0);
        let earned = badges::BADGES
            .iter()
            .filter(|badge| self.progress.badges.contains(badge.id))
            .count();
        self.state
            .set_progress_summary(unread > 0, earned, badges::BADGES.len());
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
            if sightings.shooting_star && !self.was_star_visible {
                self.stars_seen += 1;
            }
            self.was_star_visible = sightings.shooting_star;

            let has_all_weathers =
                badges::weather_badges().all(|badge| self.progress.badges.contains(badge.id));
            let events = [
                (holiday.is_christmas, "christmas"),
                (holiday.is_national_day, "national_day"),
                (holiday.is_halloween, "halloween"),
                (holiday.new_year.is_some(), "new_year"),
                (holiday.is_easter, "easter"),
                (self.state.shows_rainbow(), "rainbow"),
                (sightings.shooting_star, "shooting_star"),
                (sightings.postman, "postman"),
                (sightings.snowman, "snowman"),
                (self.stars_seen >= STARS_FOR_HUNTER, "star_hunter"),
                (has_all_weathers, "full_weather"),
            ];
            found.extend(events.iter().filter(|(seen, _)| *seen).map(|(_, id)| *id));
        }

        self.award(found);
    }

    fn award(&mut self, ids: Vec<&'static str>) {
        let new_badges: Vec<&str> = ids
            .into_iter()
            .filter(|id| self.progress.badges.insert(id.to_string()))
            .map(badges::name)
            .collect();
        if !new_badges.is_empty() {
            let label = if new_badges.len() > 1 {
                "Nouveaux badges"
            } else {
                "Nouveau badge"
            };
            let message = format!("* {} : {} ! *", label, new_badges.join(", "));
            self.toasts.push_back(message);
            self.progress.save();
        }
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

    fn refresh_unlocks(&mut self) {
        self.animations
            .set_unlocks(unlocks_for(&self.timeline, &self.progress));
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
        let available = self.available_letters();
        if available == 0 {
            return;
        }

        let index = match self.overlay {
            Some(Overlay::Letter(index)) if index + 1 < available => index + 1,
            Some(Overlay::Letter(_)) => {
                self.overlay = None;
                return;
            }
            _ => self.progress.letters_read.min(available - 1),
        };
        self.open_letter(index);
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
            self.refresh_unlocks();
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
                let Some((day, entry)) = self.timeline.mail_at(index) else {
                    return Ok(());
                };
                content::entry_view(
                    day,
                    entry,
                    index + 1,
                    self.available_letters(),
                    &self.read_context(),
                )
            }
            Some(Overlay::Album(page)) => {
                let mut pages = self.album_pages();
                let page = page.min(pages.len().saturating_sub(1));
                pages.swap_remove(page)
            }
            Some(Overlay::Bottle(index)) => views::bottle_view(index),
            Some(Overlay::Index(selected)) => views::index_view(
                &self.read_entries(),
                selected,
                renderer.get_size().1.saturating_sub(INDEX_MARGIN_ROWS) as usize,
                &self.read_context(),
            ),
            Some(Overlay::Help) => views::help_view(
                &unlocks_for(&self.timeline, &self.progress),
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
            None => return Ok(()),
        };
        overlay::render_view(renderer, &view)
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

    fn read_entries(&self) -> Vec<(usize, Entry)> {
        self.timeline
            .mail()
            .take(self.progress.letters_read.min(self.available_letters()))
            .collect()
    }

    fn album_pages(&self) -> Vec<overlay::View> {
        views::album_pages(
            &self.read_entries(),
            &self.progress,
            &unlocks_for(&self.timeline, &self.progress),
        )
    }

    fn refresh_timeline(&mut self) {
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
        for (_, entry) in self.timeline.mail().take(available) {
            let request = match entry {
                Entry::Postcard(index) if !self.progress.postcard_weather.contains_key(&index) => {
                    let city = &CITIES[index % CITIES.len()];
                    (Request::Postcard(index), city.latitude, city.longitude)
                }
                Entry::Gazette(week) if !self.progress.gazette_weather.contains_key(&week) => (
                    Request::WeekRecap(week),
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
                Request::WeekRecap(week) => self.progress.gazette_weather.insert(week, text),
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

    fn reread_selected(&mut self) {
        if let Some(Overlay::Index(selected)) = self.overlay
            && selected < self.read_entries().len()
        {
            self.open_letter(selected);
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
                    Event::Key(key_event) => match key_event.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            break;
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
                        KeyCode::Up => self.select_in_index(-1),
                        KeyCode::Down => self.select_in_index(1),
                        KeyCode::Enter => self.reread_selected(),
                        KeyCode::Char('r') | KeyCode::Char('R') => self.pick_up(),
                        KeyCode::Char('e') | KeyCode::Char('E') => self.start_riddle(),
                        KeyCode::Esc => self.overlay = None,
                        KeyCode::Left => self.browse(-1),
                        KeyCode::Right => self.browse(1),
                        _ => {}
                    },
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
