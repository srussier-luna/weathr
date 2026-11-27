use crate::advent::{self, view as advent_view};
use crate::animation::lore::{Choices, Unlocks};
use crate::animation::neighbours_party::Neighbour;
use crate::animation::trophies::Trophy;
use crate::animation_manager::AnimationManager;
use crate::app_state::AppState;
use crate::badges::{self, Award, SAM_TRAIL, Tier};
use crate::clock;
use crate::config::{Config, LocationDisplay, Setting};
use crate::demo::{Demo, SCENE_DURATION};
use crate::error::{GeolocationError, WeatherError};
use crate::geolocation::{self, DetectedLocation, GeoLocation};
use crate::holiday::{Holiday, HolidayBox, MeteorShower};
use crate::hud::{self, Placed, toast, toolbar, weather_bar::WeatherBar};
use crate::overlay;
use crate::progress::{Opening, Progress, Recovery};
use crate::rain_alert::{self, RainAlert};
use crate::release_notes::{self, Section};
use crate::remote_weather::{self, Reply, Request};
use crate::render::TerminalRenderer;
use crate::scene::WorldScene;
use crate::scene::tree::Growth;
use crate::settings::{self, AutoLocation, CitySearch, Row, Settings};
use crate::story::content::chronicles::{self, BOTTLE_MESSAGES, CITIES, OBJECTS, RIDDLES};
use crate::story::content::letters::{self, Choice, Letter};
use crate::story::content::neighbours_note::NeighboursNote;
use crate::story::content::recipes::{
    Diet, diet_for_key, diet_ids, diets_from_ids, has_every_recipe, toggle_diet,
};
use crate::story::content::tabs::{self, Tab};
use crate::story::content::{self, ReadContext};
use crate::story::postman_note::{self, MailStep};
use crate::story::reach::{launch_reach, settle};
use crate::story::sources::{self, Recipes};
use crate::story::today_letters::TodayLetters;
use crate::story::{Entry, StoryClock, Thing, Timeline, pause, punishment};
use crate::travel;
use crate::updater;
use crate::views;
use crate::weather::{
    OpenMeteoProvider, WeatherClient, WeatherCondition, WeatherData, WeatherLocation, WeatherUnits,
};
use crate::welcome::{self, Step};
use chrono::{Datelike, NaiveDate, Timelike};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::io;
use std::num::NonZeroU16;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, oneshot};

const REFRESH_INTERVAL: Duration = Duration::from_secs(300);
const INPUT_POLL_FPS: u64 = 30;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / INPUT_POLL_FPS);
const PATIENCE: Duration = Duration::from_secs(60 * 60);
const STARS_FOR_HUNTER: u32 = 3;
const REMOTE_CHECK_FRAMES: u32 = 300;
const MAX_ANSWER_LENGTH: usize = 30;
const MAX_REINDEER_NAME_LENGTH: usize = 20;
const REMOTE_CHANNEL_SIZE: usize = 16;
const OVERLAY_MARGIN_ROWS: u16 = 12;
const NEIGHBOUR_FIRST_LETTERS: [(Neighbour, usize); 5] = [
    (Neighbour::Rose, 1),
    (Neighbour::Paulo, 4),
    (Neighbour::Odile, 7),
    (Neighbour::Gaston, 10),
    (Neighbour::Leo, 16),
];
const FOOTPRINT_DAYS: [usize; 3] = [8, 11, 16];
const WET_FOOTPRINT_DAYS: [usize; 2] = [13, 30];

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
        cat: has_arrived(Thing::Cat),
        dog: has_arrived(Thing::Dog),
        owl: has_arrived(Thing::Owl),
        duck: has_arrived(Thing::Duck),
        snail: has_arrived(Thing::Snail),
        squirrel: has_arrived(Thing::Squirrel),
        frog: has_arrived(Thing::Frog),
        bench: has_arrived(Thing::Bench),
        cat_bow: has_arrived(Thing::CatBow),
        footprints: has_arrived(Thing::Footprints) && FOOTPRINT_DAYS.contains(&day),
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
        wet_footprints: has_arrived(Thing::WetFootprints) && WET_FOOTPRINT_DAYS.contains(&day),
        silhouette: has_arrived(Thing::Silhouette),
        whispering_well: has_arrived(Thing::WhisperingWell),
        staring: has_arrived(Thing::Staring),
        found_objects: has_arrived(Thing::FoundObjects),
        bottles: has_arrived(Thing::Bottles),
        choices: Choices::from_answers(answers),
    }
}

fn guests_on(timeline: &Timeline, day: usize) -> Vec<Neighbour> {
    let delivered = timeline.first_mail(timeline.delivered_by(day));
    NEIGHBOUR_FIRST_LETTERS
        .into_iter()
        .filter(|(_, number)| {
            delivered
                .iter()
                .any(|(_, entry)| *entry == Entry::Main(number - 1))
        })
        .map(|(neighbour, _)| neighbour)
        .collect()
}

fn earn(earned: &mut BTreeSet<String>, ids: Vec<&'static str>) -> Vec<Award> {
    ids.into_iter()
        .filter(|id| earned.insert(id.to_string()))
        .filter_map(badges::find)
        .map(Award::from)
        .collect()
}

fn mark_unseen(unseen: &mut BTreeSet<String>, awards: &[Award]) {
    unseen.extend(awards.iter().map(|award| award.badge.id.to_string()));
}

fn reveal_badges(
    overlay: Option<Overlay>,
    unseen: &mut BTreeSet<String>,
) -> Option<BTreeSet<String>> {
    (!matches!(overlay, Some(Overlay::Badges))).then(|| std::mem::take(unseen))
}

fn earn_tiers(earned: &mut BTreeSet<String>, before: usize, after: usize) -> Vec<Award> {
    let tiers = Tier::crossed(before, after);
    if tiers.contains(&Tier::Chocolate) {
        earned.insert(SAM_TRAIL.id.to_string());
    }
    tiers.into_iter().map(Award::sam_trail).collect()
}

struct Trip {
    is_new: bool,
    tier_cards: Vec<Award>,
}

fn record_trip(
    progress: &mut Progress,
    launch_location: Option<&GeoLocation>,
    day: Option<NonZeroU16>,
) -> Trip {
    let sam_cities = progress.sam_cities.len();
    let is_new = match (launch_location, day) {
        (Some(location), None) => travel::record(progress, location),
        _ => false,
    };
    let tier_cards = earn_tiers(&mut progress.badges, sam_cities, progress.sam_cities.len());
    mark_unseen(&mut progress.unseen_badges, &tier_cards);
    Trip { is_new, tier_cards }
}

fn badge_card(overlay: Option<Overlay>) -> Option<Award> {
    match overlay {
        Some(Overlay::NewBadge(award) | Overlay::NoteBadge(award)) => Some(award),
        _ => None,
    }
}

fn next_overlay(overlay: Option<Overlay>, pending_badges: &mut VecDeque<Award>) -> Option<Overlay> {
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

fn is_reread(overlay: Option<Overlay>) -> bool {
    matches!(overlay, Some(Overlay::Letter { from: Some(_), .. }))
}

fn closed_overlay(overlay: Option<Overlay>) -> Option<Overlay> {
    match overlay {
        Some(Overlay::NoteBadge(_)) => Some(Overlay::PostmanNote),
        Some(Overlay::ChocolutinIntroduction(day)) => Some(Overlay::Chocolate(day)),
        Some(Overlay::Letter {
            from: Some((tab, selected)),
            ..
        }) => Some(Overlay::Index { tab, selected }),
        _ => None,
    }
}

fn album_choice(
    overlay: Option<Overlay>,
    answers: &BTreeMap<String, u8>,
) -> Option<&'static Choice> {
    match overlay {
        Some(Overlay::Album {
            page: views::ANSWERS_PAGE,
            selected,
        }) => letters::answered_choices(answers)
            .nth(selected)
            .map(|(choice, _)| choice),
        _ => None,
    }
}

fn answer_option(
    overlay: Option<Overlay>,
    answers: &mut BTreeMap<String, u8>,
    letter_choice: Option<&'static Choice>,
    option: u8,
) -> bool {
    let choice = match overlay {
        Some(Overlay::Letter { .. }) => {
            letter_choice.filter(|choice| !answers.contains_key(choice.id))
        }
        _ => album_choice(overlay, answers),
    };
    let Some(choice) = choice else {
        return false;
    };
    answers.insert(choice.id.to_string(), option);
    true
}

fn album_trophy(overlay: Option<Overlay>, badges: &BTreeSet<String>) -> Option<Trophy> {
    match overlay {
        Some(Overlay::Album {
            page: views::TROPHIES_PAGE,
            selected,
        }) => Trophy::won(badges).nth(selected),
        _ => None,
    }
}

fn place_trophy(
    overlay: Option<Overlay>,
    badges: &BTreeSet<String>,
    hidden: &mut BTreeSet<String>,
    option: u8,
) -> bool {
    let Some(trophy) = album_trophy(overlay, badges) else {
        return false;
    };
    if option == 1 {
        hidden.remove(trophy.id());
    } else {
        hidden.insert(trophy.id().to_string());
    }
    true
}

fn album_rows(
    page: usize,
    answers: &BTreeMap<String, u8>,
    badges: &BTreeSet<String>,
) -> Option<usize> {
    match page {
        views::ANSWERS_PAGE => Some(letters::answered_choices(answers).count()),
        views::TROPHIES_PAGE => Some(Trophy::won(badges).count()),
        _ => None,
    }
}

fn album_after_scroll(
    overlay: Option<Overlay>,
    step: isize,
    answers: &BTreeMap<String, u8>,
    badges: &BTreeSet<String>,
) -> Option<Overlay> {
    let Some(Overlay::Album { page, selected }) = overlay else {
        return overlay;
    };
    let Some(rows) = album_rows(page, answers, badges) else {
        return overlay;
    };
    Some(Overlay::Album {
        page,
        selected: selected
            .saturating_add_signed(step)
            .min(rows.saturating_sub(1)),
    })
}

fn requeue_replaced_badge(
    previous: Option<Overlay>,
    current: Option<Overlay>,
    pending_badges: &mut VecDeque<Award>,
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

fn neighbours_note_on_launch(
    day: Option<NonZeroU16>,
    progress: &mut Progress,
    clock: &mut StoryClock,
    mail_date: NaiveDate,
    rng: &mut impl rand::Rng,
) -> Option<NeighboursNote> {
    if day.is_some() {
        return None;
    }
    match punishment::sentence(progress, clock, mail_date) {
        Some(punishment) => Some(NeighboursNote::Announcement(punishment)),
        None => punishment::is_mail_banned(progress, mail_date)
            .then(|| NeighboursNote::disappointed(rng)),
    }
}

fn notes_overlay(has_notes: bool) -> Option<Overlay> {
    has_notes.then_some(Overlay::Notes(0))
}

fn after_welcome(has_neighbours_note: bool, has_notes: bool) -> Option<Overlay> {
    if has_neighbours_note {
        Some(Overlay::NeighboursNote)
    } else {
        notes_overlay(has_notes)
    }
}

fn opening_overlay(
    has_backup: bool,
    shows_welcome: bool,
    after_welcome: Option<Overlay>,
) -> Option<Overlay> {
    if has_backup {
        Some(Overlay::Backup { shows_welcome })
    } else if shows_welcome {
        Some(Overlay::Welcome(Step::Arriving))
    } else {
        after_welcome
    }
}

fn holiday_box_after_launch(
    overlay: Option<Overlay>,
    pending_holiday_box: &mut Option<&'static HolidayBox>,
) -> Option<Overlay> {
    overlay.or_else(|| pending_holiday_box.take().map(Overlay::HolidayBox))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChocolutinMove {
    Wait,
    Visit,
    Give(u32),
    Leave,
}

fn chocolutin_move(
    chocolate_day: Option<u32>,
    overlay: Option<Overlay>,
    is_visiting: bool,
    has_arrived: bool,
) -> ChocolutinMove {
    match chocolate_day {
        None if matches!(
            overlay,
            Some(Overlay::Chocolate(_) | Overlay::ChocolutinIntroduction(_))
        ) =>
        {
            ChocolutinMove::Wait
        }
        None => ChocolutinMove::Leave,
        Some(_) if overlay.is_some() => ChocolutinMove::Wait,
        Some(_) if !is_visiting => ChocolutinMove::Visit,
        Some(day) if has_arrived => ChocolutinMove::Give(day),
        Some(_) => ChocolutinMove::Wait,
    }
}

fn chocolate_overlay(day: u32, is_introduced: bool) -> Overlay {
    if is_introduced {
        Overlay::Chocolate(day)
    } else {
        Overlay::ChocolutinIntroduction(day)
    }
}

fn chocolate_due_on_launch(
    is_forced: bool,
    day: Option<NonZeroU16>,
    progress: &Progress,
    today: NaiveDate,
) -> Option<u32> {
    if is_forced || day.is_some() {
        return None;
    }
    advent::due_day(&progress.advent_days(today.year()), today)
}

fn chocolutin_notes(is_real_date: bool, progress: &Progress, today: NaiveDate) -> Vec<u32> {
    if !is_real_date {
        return Vec::new();
    }
    advent::missed_days(&progress.advent_days(today.year()), today)
}

fn toggled_calendar(
    overlay: Option<Overlay>,
    is_december: bool,
    today: NaiveDate,
) -> Option<Overlay> {
    match overlay {
        Some(Overlay::Calendar(_)) => None,
        _ if is_december => Some(Overlay::Calendar(advent::opening_day(today))),
        overlay => overlay,
    }
}

fn backup_after_key(
    code: KeyCode,
    shows_welcome: bool,
    after_welcome: Option<Overlay>,
) -> Option<Overlay> {
    match code {
        KeyCode::Esc | KeyCode::Enter => opening_overlay(false, shows_welcome, after_welcome),
        _ => Some(Overlay::Backup { shows_welcome }),
    }
}

fn neighbours_note_after_key(code: KeyCode, has_notes: bool) -> Option<Overlay> {
    match code {
        KeyCode::Esc | KeyCode::Enter => notes_overlay(has_notes),
        _ => Some(Overlay::NeighboursNote),
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

fn push_toast(toasts: &mut VecDeque<String>, message: &str) {
    if !toasts.iter().any(|toast| toast == message) {
        toasts.push_back(message.to_string());
    }
}

fn shows_every_habitant(is_forced: bool, day: Option<NonZeroU16>) -> bool {
    is_forced && day.is_none()
}

fn santa_visit_overlay(
    is_real_date: bool,
    is_santa_flying: bool,
    awaits_reindeer_name: bool,
) -> Option<Overlay> {
    (is_real_date && !is_santa_flying && awaits_reindeer_name).then_some(Overlay::SantaNote)
}

fn santa_note_after_key(code: KeyCode) -> Option<Overlay> {
    match code {
        KeyCode::Enter | KeyCode::Esc => Some(Overlay::Reindeer),
        _ => Some(Overlay::SantaNote),
    }
}

fn santa_due_on_launch(is_real_date: bool, progress: &Progress, today: NaiveDate) -> bool {
    is_real_date && progress.is_santa_due(today)
}

fn freeze_recipes(progress: &mut Progress, clock: &StoryClock, today: usize) -> Recipes {
    let recipes = Recipes::from_progress(progress);
    if progress.diet_answered_day.is_some() && progress.diet_recipes_until != Some(today) {
        progress.diet_recipes = recipes.deliveries(clock, today);
        progress.diet_recipes_until = Some(today);
        progress.save();
    }
    recipes
}

fn is_live_box_full(
    clock: &StoryClock,
    timeline: &Timeline,
    day: usize,
    letters_read: usize,
) -> bool {
    !clock.is_frozen() && pause::is_box_full(timeline, day, letters_read)
}

fn spawn_weather(
    sender: mpsc::Sender<Result<WeatherData, WeatherError>>,
    location: WeatherLocation,
    units: WeatherUnits,
) {
    let provider = Arc::new(OpenMeteoProvider::new());
    let weather_client = WeatherClient::new(provider, REFRESH_INTERVAL);
    tokio::spawn(async move {
        loop {
            let result = weather_client.get_current_weather(&location, &units).await;
            if sender.send(result).await.is_err() {
                break;
            }
            tokio::select! {
                () = tokio::time::sleep(REFRESH_INTERVAL) => {}
                () = sender.closed() => break,
            }
        }
    });
}

fn generate_offline_weather(rng: &mut impl rand::Rng) -> WeatherData {
    use rand::RngExt;

    let now = clock::now();
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

fn render_placed(renderer: &mut TerminalRenderer, placed: &Placed) -> io::Result<()> {
    let mut x = placed.x;
    for segment in &placed.line {
        renderer.render_line_colored(x, placed.y, &segment.text, segment.color)?;
        x = x.saturating_add(u16::try_from(segment.text.chars().count()).unwrap_or(u16::MAX));
    }
    Ok(())
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
    Letter {
        index: usize,
        from: Option<(Tab, usize)>,
    },
    Badges,
    Album {
        page: usize,
        selected: usize,
    },
    Backup {
        shows_welcome: bool,
    },
    Bottle(usize),
    Calendar(u32),
    Chocolate(u32),
    ChocolutinIntroduction(u32),
    ChocolutinNote(NaiveDate),
    Help,
    HolidayBox(&'static HolidayBox),
    Index {
        tab: Tab,
        selected: usize,
    },
    NeighboursNote,
    NewBadge(Award),
    NoteBadge(Award),
    Notes(usize),
    PostmanNote,
    Settings(usize),
    Welcome(Step),
    SantaNote,
    Reindeer,
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
    weather_bar: WeatherBar,
    simulated_holiday: Option<Holiday>,
    invites_every_neighbour: bool,
    demo: Option<Demo>,
    progress: Progress,
    letters: Vec<Letter>,
    clock: StoryClock,
    timeline: Timeline,
    release_notes: Vec<Section>,
    recovery: Option<Recovery>,
    neighbours_note: Option<NeighboursNote>,
    bare_village: bool,
    overlay: Option<Overlay>,
    is_simulated: bool,
    awards_badges: bool,
    is_time_master: bool,
    is_village_quill: bool,
    toasts: VecDeque<String>,
    toast_started: Option<Instant>,
    pending_badges: VecDeque<Award>,
    revealed_badges: BTreeSet<String>,
    pending_holiday_box: Option<&'static HolidayBox>,
    chocolate_day: Option<u32>,
    is_real_date: bool,
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
    fetches_weather: bool,
    auto_location: bool,
    city_name_language: String,
    city_search: Option<CitySearch>,
    location_detection: Option<oneshot::Receiver<Result<DetectedLocation, GeolocationError>>>,
    is_located_by_device: bool,
    shows_every_habitant: bool,
    diet_draft: BTreeSet<Diet>,
    diet_editor: Option<BTreeSet<Diet>>,
    is_santa_due: bool,
    reindeer_draft: String,
}

impl App {
    pub fn new(
        config: &Config,
        simulation: SimulationOptions,
        launch_location: Option<&DetectedLocation>,
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
        let fetches_weather = demo.is_none() && simulation.condition.is_none();

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
            spawn_weather(tx, location, config.units);
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
        let (mut progress, letters, clock, opening) = match simulation.day {
            Some(day) => {
                let clock = StoryClock::frozen(clock::now().date_naive(), config.mail.hour, day);
                let mut progress = Progress::in_memory(clock.date_of(1));
                progress.diet_letter_day = Some(sources::diet_letter_day(1));
                (progress, letters::read(), clock, Opening::default())
            }
            None => {
                let (progress, opening) = Progress::load();
                let clock = StoryClock::for_progress(&progress, config.mail.hour);
                (progress, letters::load(), clock, opening)
            }
        };
        let timeline_day = clock.today();
        if simulation.day.is_none() {
            let reach = launch_reach(
                letters.len(),
                &clock,
                timeline_day,
                Recipes::from_progress(&progress),
            );
            if settle(&mut progress, opening.out_of_sync, &reach) {
                progress.save();
            }
        }
        let release_notes = notes_on_launch(&mut progress, release_notes::load(), launch);
        let shows_welcome = welcome_on_launch(&progress, launch);
        if shows_welcome {
            animations.postman_visit();
        }
        let trip = record_trip(
            &mut progress,
            launch_location.map(|detected| &detected.location),
            simulation.day,
        );
        if progress.record_visit() || trip.is_new {
            progress.save();
        }
        let rain_receiver =
            (!is_simulated).then(|| rain_alert::spawn(location.latitude, location.longitude));
        if progress.diet_letter_day.is_none() {
            progress.diet_letter_day = Some(sources::diet_letter_day(timeline_day));
            progress.save();
        }
        let recipes = freeze_recipes(&mut progress, &clock, timeline_day);
        let timeline = Timeline::for_village(letters.len(), &clock, timeline_day, recipes);
        let (remote_sender, remote_receiver) = mpsc::channel(REMOTE_CHANNEL_SIZE);

        let pending_holiday_box = (!simulation.demo)
            .then(|| simulation.holiday.unwrap_or_else(Holiday::today))
            .and_then(|holiday| holiday.holiday_box());

        let is_forced = is_simulated || simulation.night;
        let chocolate_day = chocolate_due_on_launch(
            is_forced,
            simulation.day,
            &progress,
            clock::now().date_naive(),
        );
        let is_real_date = !is_forced && simulation.day.is_none();
        let is_santa_due = santa_due_on_launch(is_real_date, &progress, clock::now().date_naive());
        let has_backup = opening.recovery.is_some();
        // Kept on purpose for future contributor Badges: do not remove.
        let user_name = updater::git_user_name();
        let mut app = Self {
            state,
            animations,
            scene,
            weather_receiver: rx,
            hide_hud: config.hide_hud,
            weather_bar: WeatherBar::default(),
            simulated_holiday: simulation.holiday,
            invites_every_neighbour: simulation.demo
                || (simulation.holiday.is_some() && simulation.day.is_none()),
            demo,
            progress,
            letters,
            clock,
            timeline,
            overlay: None,
            release_notes,
            recovery: opening.recovery,
            neighbours_note: None,
            bare_village: false,
            is_simulated: is_forced,
            awards_badges: simulation.day.is_none(),
            is_time_master: badges::is_time_master(user_name.as_deref()),
            is_village_quill: badges::is_village_quill(user_name.as_deref()),
            toasts: VecDeque::new(),
            toast_started: None,
            pending_badges: trip.tier_cards.into(),
            revealed_badges: BTreeSet::new(),
            pending_holiday_box,
            chocolate_day,
            is_real_date,
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
            fetches_weather,
            auto_location: config.location.auto,
            city_name_language: config.location.city_name_language.clone(),
            city_search: None,
            location_detection: None,
            is_located_by_device: launch_location.is_some_and(|detected| detected.is_from_device),
            shows_every_habitant: shows_every_habitant(is_forced, simulation.day),
            diet_draft: BTreeSet::new(),
            diet_editor: None,
            is_santa_due,
            reindeer_draft: String::new(),
        };
        app.hold_full_box();
        let mail_date = app.mail_date();
        app.neighbours_note = neighbours_note_on_launch(
            simulation.day,
            &mut app.progress,
            &mut app.clock,
            mail_date,
            &mut rand::rng(),
        );
        if matches!(app.neighbours_note, Some(NeighboursNote::Announcement(_))) {
            app.progress.save();
        }
        app.overlay = opening_overlay(has_backup, shows_welcome, app.after_welcome());
        app.bare_village = app.is_village_bare();
        app.refresh_unlocks();
        app.refresh_tree_growth();
        app.refresh_timeline();
        app
    }

    fn refresh_progress(&mut self) {
        if self.progress.record_visit() {
            self.progress.save();
        }
        self.animations
            .set_mail_hour(self.clock.mail_hour_on(clock::now().date_naive()));
        self.refresh_timeline();
        self.state.set_story_day(self.timeline_day);
        self.request_remote_weather();
        self.receive_remote_weather();
        self.animations.set_finds_remaining(
            self.progress.found_objects.len() < OBJECTS.len(),
            self.progress.bottles.len() < BOTTLE_MESSAGES.len(),
        );
        let hint = match self.animations.visible_finds() {
            (true, true) => Some("Objet et bouteille"),
            (true, false) => Some("Objet"),
            (false, true) => Some("Bouteille"),
            (false, false) => None,
        };
        self.state.set_finds_hint(hint);
        let unread = self
            .available_letters()
            .saturating_sub(self.progress.letters_read);
        let is_mail_waiting =
            (unread > 0 || !self.chocolutin_notes().is_empty()) && !self.hides_mail();
        self.animations.set_mail_waiting(is_mail_waiting);
        self.animations.set_story_frozen(
            self.clock.is_frozen() || self.is_box_full() || self.is_mail_banned(),
        );
        let (earned, total) = badges::collection_progress(&self.progress.badges);
        self.state
            .set_progress_summary(is_mail_waiting, earned, total);
        self.state
            .set_new_badge(!self.progress.unseen_badges.is_empty());
    }

    fn collect_badges(&mut self) {
        let mut found = Vec::new();
        let hour = clock::now().hour();
        let milestones = [
            ((0..5).contains(&hour), "night_owl"),
            ((5..7).contains(&hour), "early_bird"),
            (self.session_start.elapsed() >= PATIENCE, "patience"),
            (self.progress.streak >= 3, "streak_3"),
            (self.progress.streak >= 7, "streak_7"),
            (self.progress.streak >= 30, "streak_30"),
            (self.progress.streak >= 60, "streak_60"),
            (self.progress.streak >= 100, "streak_100"),
            (self.progress.streak >= 365, "streak_365"),
            (self.progress.visited_cities.len() >= 7, "traveller"),
            (self.progress.visited_cities.len() >= 15, "backpacker"),
            (self.has_read_all_letters(), "all_letters"),
            (!self.progress.riddles_solved.is_empty(), "riddle_1"),
            (self.progress.riddles_solved.len() >= 10, "riddle_10"),
            (
                self.progress.found_objects.len() >= OBJECTS.len(),
                "all_objects",
            ),
            (
                chronicles::has_solved_every_riddle(&self.progress.riddles_solved),
                "all_riddles",
            ),
            (self.has_received_every_recipe(), "all_recipes"),
            (!self.progress.bottles.is_empty(), "first_bottle"),
            (self.is_time_master, "time_master"),
            (self.is_village_quill, "village_quill"),
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
                (holiday.is_neighbours_party, "neighbours_party"),
                (holiday.is_music_day, "music_day"),
                (holiday.is_candlemas, "candlemas"),
                (holiday.is_may_day, "may_day"),
                (self.state.shows_rainbow(), "rainbow"),
                (sightings.postman, "postman"),
                (sightings.snowman, "snowman"),
                (self.star_watch.seen >= STARS_FOR_HUNTER, "star_hunter"),
                (has_all_weathers, "full_weather"),
                (self.is_located_by_device, "compass"),
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
            mark_unseen(&mut self.progress.unseen_badges, &new_badges);
            self.pending_badges.extend(new_badges);
            self.progress.save();
            self.refresh_trophies();
        }
    }

    fn open_holiday_box(&mut self) {
        self.overlay = holiday_box_after_launch(self.overlay, &mut self.pending_holiday_box);
    }

    fn bring_chocolate(&mut self) {
        match chocolutin_move(
            self.chocolate_day,
            self.overlay,
            self.animations.is_chocolutin_visiting(),
            self.animations.has_chocolutin_arrived(),
        ) {
            ChocolutinMove::Visit => self.animations.chocolutin_visit(),
            ChocolutinMove::Give(day) => self.give_chocolate(day),
            ChocolutinMove::Leave => self.animations.chocolutin_leave(),
            ChocolutinMove::Wait => {}
        }
    }

    fn give_chocolate(&mut self, day: u32) {
        self.chocolate_day = None;
        self.overlay = Some(chocolate_overlay(day, self.progress.chocolutin_introduced));
        self.progress.chocolutin_introduced = true;
        self.progress
            .receive_advent_day(clock::now().date_naive().year(), day);
        self.progress.save();
    }

    fn chocolutin_notes(&self) -> Vec<u32> {
        chocolutin_notes(self.is_real_date, &self.progress, clock::now().date_naive())
    }

    fn open_chocolutin_note(&mut self, day: u32) {
        let year = clock::now().date_naive().year();
        let Some(date) = NaiveDate::from_ymd_opt(year, 12, day) else {
            return;
        };
        self.overlay = Some(Overlay::ChocolutinNote(date));
        if self.progress.receive_advent_day(year, day) {
            self.progress.save();
        }
    }

    fn advent_cells(&self) -> Vec<advent::Cell> {
        let today = clock::now().date_naive();
        advent::cells(&self.progress.advent_days(today.year()), today)
    }

    fn toggle_calendar(&mut self) {
        self.overlay = toggled_calendar(
            self.overlay,
            self.state.holiday.is_december,
            clock::now().date_naive(),
        );
    }

    fn open_pending_badge(&mut self) {
        self.overlay = next_overlay(self.overlay, &mut self.pending_badges);
    }

    fn update_santa_visit(&mut self) {
        if self.overlay.is_some() {
            return;
        }
        let today = clock::now().date_naive();
        if self.is_santa_due {
            self.is_santa_due = false;
            self.progress.welcome_reindeer(today);
            self.progress.save();
            self.animations.send_santa();
            self.refresh_unlocks();
        } else {
            self.overlay = santa_visit_overlay(
                self.is_real_date,
                self.animations.is_santa_flying(),
                self.progress.awaits_reindeer_name(today),
            );
        }
    }

    fn handle_reindeer_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(ch) if self.reindeer_draft.chars().count() < MAX_REINDEER_NAME_LENGTH => {
                self.reindeer_draft.push(ch)
            }
            KeyCode::Backspace => {
                self.reindeer_draft.pop();
            }
            KeyCode::Enter => self.name_reindeer(),
            _ => {}
        }
    }

    fn name_reindeer(&mut self) {
        self.progress
            .name_reindeer(&std::mem::take(&mut self.reindeer_draft));
        self.progress.save();
        self.overlay = None;
        if let Some(name) = self.progress.reindeer_name() {
            push_toast(&mut self.toasts, &format!("Bienvenue au jardin, {name}"));
        }
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

    fn press_backup_key(&mut self, code: KeyCode) {
        if let Some(Overlay::Backup { shows_welcome }) = self.overlay {
            self.overlay = backup_after_key(code, shows_welcome, self.after_welcome());
        }
    }

    fn press_neighbours_note_key(&mut self, code: KeyCode) {
        self.overlay = neighbours_note_after_key(code, !self.release_notes.is_empty());
    }

    fn after_welcome(&self) -> Option<Overlay> {
        after_welcome(
            self.neighbours_note.is_some(),
            !self.release_notes.is_empty(),
        )
    }

    fn end_welcome(&mut self) {
        self.progress.welcome_seen = true;
        self.progress.save();
        self.animations.postman_leave();
        self.overlay = self.after_welcome();
    }

    fn has_received_every_recipe(&self) -> bool {
        if self.progress.diet_answered_day.is_none() {
            return false;
        }
        let received: BTreeSet<usize> = self
            .timeline
            .first_mail(self.available_letters())
            .iter()
            .filter_map(|(_, entry)| entry.recipe_index())
            .collect();
        has_every_recipe(&diets_from_ids(&self.progress.diets), &received)
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
        let unlocks = unlocks_on(&self.timeline, self.timeline_day, &self.progress.answers);
        if self.shows_every_habitant {
            unlocks.with_every_habitant()
        } else {
            unlocks
        }
    }

    fn guests(&self) -> Vec<Neighbour> {
        if self.invites_every_neighbour {
            Neighbour::ALL.to_vec()
        } else {
            guests_on(&self.timeline, self.timeline_day)
        }
    }

    fn refresh_unlocks(&mut self) {
        let unlocks = self.unlocks();
        self.animations.set_unlocks(if self.bare_village {
            unlocks.without_habitants()
        } else {
            unlocks
        });
        self.animations.set_guests(self.guests());
        self.animations
            .set_reindeer_home(self.progress.has_reindeer() && !self.bare_village);
        self.refresh_trophies();
    }

    fn refresh_trophies(&mut self) {
        self.animations.set_trophies(Trophy::shown(
            &self.progress.badges,
            &self.progress.hidden_trophies,
        ));
    }

    fn mail_date(&self) -> NaiveDate {
        self.clock.mail_date(clock::now().naive_local())
    }

    fn is_mail_banned(&self) -> bool {
        punishment::is_mail_banned(&self.progress, self.mail_date())
    }

    fn is_village_bare(&self) -> bool {
        punishment::is_village_bare(&self.progress, self.timeline_day, self.mail_date())
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
        self.timeline.delivered_by(self.clock.today())
    }

    fn village_letter_number(&self, index: usize) -> Option<usize> {
        match self.timeline.mail_at(index)? {
            (_, Entry::Main(main)) => Some(main + 1),
            _ => None,
        }
    }

    fn press_option(&mut self, option: u8) {
        if place_trophy(
            self.overlay,
            &self.progress.badges,
            &mut self.progress.hidden_trophies,
            option,
        ) {
            self.progress.save();
            self.refresh_trophies();
            return;
        }
        let letter_choice = match self.overlay {
            Some(Overlay::Letter { index, .. }) => self
                .village_letter_number(index)
                .and_then(letters::choice_for),
            _ => None,
        };
        if answer_option(
            self.overlay,
            &mut self.progress.answers,
            letter_choice,
            option,
        ) {
            self.progress.save();
            self.refresh_unlocks();
        }
    }

    fn is_diet_letter_waiting(&self) -> bool {
        let Some(Overlay::Letter { index, .. }) = self.overlay else {
            return false;
        };
        self.progress.diet_answered_day.is_none()
            && matches!(self.timeline.mail_at(index), Some((_, Entry::DietLetter)))
    }

    fn press_number(&mut self, key: char) {
        if self.is_diet_letter_waiting() {
            if let Some(diet) = diet_for_key(key) {
                toggle_diet(&mut self.diet_draft, diet);
            }
            return;
        }
        match key {
            '1' => self.press_option(1),
            '2' => self.press_option(2),
            _ => {}
        }
    }

    fn send_diets(&mut self) {
        self.progress.diet_answered_day = Some(self.clock.today());
        self.progress.diets = diet_ids(&self.diet_draft);
        self.progress.save();
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
        if is_reread(self.overlay) {
            return;
        }
        if let Some(&day) = self.chocolutin_notes().first() {
            self.open_chocolutin_note(day);
            return;
        }
        let is_note_waiting = postman_note::is_postman_note_waiting(
            self.is_box_full(),
            self.progress.postman_note_day,
            self.timeline_day,
        );
        if matches!(self.overlay, Some(Overlay::ChocolutinNote(_)))
            && !is_note_waiting
            && self.available_letters() <= self.progress.letters_read
        {
            self.overlay = None;
            return;
        }
        let open_letter = match self.overlay {
            Some(Overlay::Letter { index, .. }) => Some(index),
            _ => None,
        };
        match postman_note::next_mail(
            open_letter,
            self.today_letters().start,
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
        match self.overlay {
            Some(Overlay::Letter {
                from: Some((tab, selected)),
                ..
            }) => {
                if let Some((selected, index)) =
                    tabs::entry_beside(self.read_entries(), tab, selected, step)
                {
                    self.open_letter_from(index, Some((tab, selected)));
                }
            }
            Some(Overlay::Letter { index, from: None }) => {
                if let Some(target) =
                    self.today_letters()
                        .browse(index, step, self.available_letters())
                {
                    self.open_letter(target);
                }
            }
            _ => {}
        }
    }

    fn today_letters(&self) -> TodayLetters {
        TodayLetters::on_day(
            self.saved_today_letters(),
            self.timeline_day,
            self.progress.letters_read,
        )
    }

    fn saved_today_letters(&self) -> Option<TodayLetters> {
        self.progress
            .today_letters()
            .map(|(day, start)| TodayLetters { day, start })
    }

    fn settle_today_letters(&mut self) {
        let today = self.today_letters();
        if self.saved_today_letters() != Some(today) {
            self.progress.set_today_letters(today.day, today.start);
            self.progress.save();
        }
    }

    fn open_letter(&mut self, index: usize) {
        let from = match self.overlay {
            Some(Overlay::Letter { from, .. }) => from,
            _ => None,
        };
        self.open_letter_from(index, from);
    }

    fn open_letter_from(&mut self, index: usize, from: Option<(Tab, usize)>) {
        self.overlay = Some(Overlay::Letter { index, from });
        if content::is_night_hour(clock::now().hour()) {
            self.award(vec!["night_reading"]);
        }
        if index + 1 > self.progress.letters_read {
            self.progress.letters_read = index + 1;
            self.progress.save();
        }
    }

    fn update_toasts(&mut self, terminal_width: u16) {
        let now = Instant::now();
        if let (Some(message), Some(started)) = (self.toasts.front(), self.toast_started)
            && toast::is_over(message, now - started, terminal_width)
        {
            self.toasts.pop_front();
            self.toast_started = None;
        }
        if self.toast_started.is_none() && !self.toasts.is_empty() {
            self.toast_started = Some(now);
        }
    }

    fn render_overlays(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        if let (Some(message), Some(started)) = (self.toasts.front(), self.toast_started) {
            let (width, height) = renderer.get_size();
            for placed in toast::lay_out(message, started.elapsed(), width, height) {
                render_placed(renderer, &placed)?;
            }
        }

        let view = match self.overlay {
            Some(Overlay::Letter { index, from }) => {
                let delivered = self.timeline.first_mail(self.available_letters());
                let Some(view) = content::entry_view(delivered, index, &self.read_context()) else {
                    return Ok(());
                };
                let (has_previous, has_next) = match from {
                    Some((tab, selected)) => {
                        let read = self.read_entries();
                        (
                            tabs::entry_beside(read, tab, selected, -1).is_some(),
                            tabs::entry_beside(read, tab, selected, 1).is_some(),
                        )
                    }
                    None => (
                        false,
                        self.today_letters().contains(index)
                            && postman_note::has_unread_after(
                                index,
                                self.available_letters(),
                                self.progress.letters_read,
                            ),
                    ),
                };
                return overlay::render_with_chevrons(renderer, &view, has_previous, has_next);
            }
            Some(Overlay::Album { page, selected }) => {
                let mut pages = self.album_pages(selected);
                let last = pages.len().saturating_sub(1);
                let page = page.min(last);
                let view = pages.swap_remove(page);
                return overlay::render_with_chevrons(renderer, &view, page > 0, page < last);
            }
            Some(Overlay::Backup { .. }) => match &self.recovery {
                Some(Recovery::NewGame(backup)) => views::backup_view(backup),
                Some(Recovery::Restored) => views::restored_view(),
                None => return Ok(()),
            },
            Some(Overlay::Bottle(index)) => views::bottle_view(index),
            Some(Overlay::Calendar(selected)) => {
                advent_view::calendar_view(&self.advent_cells(), selected)
            }
            Some(Overlay::Chocolate(day)) => match advent_view::chocolate_card_view(day) {
                Some(view) => view,
                None => return Ok(()),
            },
            Some(Overlay::ChocolutinIntroduction(_)) => advent_view::introduction_view(),
            Some(Overlay::ChocolutinNote(date)) => match advent_view::note_view(date) {
                Some(view) => view,
                None => return Ok(()),
            },
            Some(Overlay::NeighboursNote) => match self.neighbours_note {
                Some(note) => views::neighbours_note_view(note),
                None => return Ok(()),
            },
            Some(Overlay::HolidayBox(holiday_box)) => views::holiday_box_view(holiday_box),
            Some(Overlay::NewBadge(award) | Overlay::NoteBadge(award)) => {
                views::badge_card_view(award)
            }
            Some(Overlay::PostmanNote) => views::postman_note_view(self.back_to_school()),
            Some(Overlay::Notes(first)) => views::notes_view(
                &self.release_notes,
                first,
                overlay_rows(renderer.get_size().1),
                notes_width(renderer),
            ),
            Some(Overlay::Index { tab, selected }) => {
                let rows = overlay_rows(renderer.get_size().1);
                let view = if self.is_box_full() {
                    views::full_box_index_view(
                        self.read_entries(),
                        tab,
                        selected,
                        rows,
                        overlay::text_width(renderer, views::INDEX_FRAME),
                        &self.read_context(),
                        self.back_to_school(),
                    )
                } else {
                    views::index_view(
                        self.read_entries(),
                        tab,
                        selected,
                        rows,
                        &self.read_context(),
                    )
                };
                let (has_previous, has_next) = tabs::has_neighbours(self.read_entries(), tab);
                return overlay::render_with_chevrons(renderer, &view, has_previous, has_next);
            }
            Some(Overlay::Help) => views::help_view(
                &self.unlocks(),
                count_read(&self.timeline, &self.progress, |entry| {
                    matches!(entry, Entry::Gazette(_))
                }) > 0,
                self.state.holiday.is_december,
            ),
            Some(Overlay::Settings(selected)) => match (&self.city_search, &self.diet_editor) {
                (Some(search), _) => views::city_search_view(search),
                (None, Some(diets)) => views::diet_view(diets),
                (None, None) => views::settings_view(
                    &self.settings(),
                    selected,
                    self.clock.mail_hour_on(clock::now().date_naive()) != self.clock.mail_hour(),
                ),
            },
            Some(Overlay::Badges) => {
                return overlay::render_badges(
                    renderer,
                    &self.progress.badges,
                    &self.revealed_badges,
                    self.progress.streak,
                    self.progress.sam_cities.len(),
                );
            }
            Some(Overlay::Welcome(step)) => return self.render_welcome(renderer, step),
            Some(Overlay::SantaNote) => views::santa_note_view(),
            Some(Overlay::Reindeer) => views::reindeer_view(&self.reindeer_draft),
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
            reading_hour: clock::now().hour(),
            riddle_input: self.riddle_input.as_deref(),
            diet_draft: &self.diet_draft,
            from_courrier: is_reread(self.overlay),
        }
    }

    fn read_entries(&self) -> &[(usize, Entry)] {
        self.timeline
            .first_mail(self.progress.letters_read.min(self.available_letters()))
    }

    fn album_pages(&self, selected: usize) -> Vec<overlay::View> {
        views::album_pages(
            self.read_entries(),
            &self.progress,
            &self.unlocks(),
            selected,
        )
    }

    fn back_to_school(&self) -> NaiveDate {
        self.clock.next_rollover(clock::now().naive_local())
    }

    fn hold_full_box(&mut self) {
        if self.clock.is_frozen() {
            return;
        }
        let today = self.clock.mail_date(clock::now().naive_local());
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
        let today = self.clock.today();
        let is_new_day = today != self.timeline_day;
        if is_new_day {
            let recipes = freeze_recipes(&mut self.progress, &self.clock, today);
            self.timeline = Timeline::for_village(self.letters.len(), &self.clock, today, recipes);
            self.timeline_day = today;
        }
        let bare_village = self.is_village_bare();
        if is_new_day || bare_village != self.bare_village {
            self.bare_village = bare_village;
            self.refresh_unlocks();
            self.refresh_tree_growth();
        }
        self.settle_today_letters();
    }

    fn tree_growth(&self) -> Growth {
        if self.is_simulated {
            Growth::Full
        } else {
            Growth::on_day(self.timeline_day)
        }
    }

    fn refresh_tree_growth(&mut self) {
        let growth = self.tree_growth();
        self.scene.set_tree_growth(growth);
        self.scene.set_bare_tree(self.bare_village);
        self.animations.set_tree_growth(growth);
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
            Some(Overlay::Letter { index, .. }) => match self.timeline.mail_at(index)?.1 {
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
            Some(Overlay::Index { .. }) => None,
            _ => Some(self.index_on(tabs::opening_tab(self.read_entries()))),
        };
    }

    fn index_on(&self, tab: Tab) -> Overlay {
        Overlay::Index {
            tab,
            selected: self.last_in_tab(tab),
        }
    }

    fn last_in_tab(&self, tab: Tab) -> usize {
        tabs::entries_in(self.read_entries(), tab)
            .len()
            .saturating_sub(1)
    }

    fn select_in_index(&mut self, step: isize) {
        if let Some(Overlay::Index { tab, selected }) = self.overlay {
            self.overlay = Some(Overlay::Index {
                tab,
                selected: selected
                    .saturating_add_signed(step)
                    .min(self.last_in_tab(tab)),
            });
        }
    }

    fn switch_tab(&mut self, step: isize) {
        if let Some(Overlay::Index { tab, .. }) = self.overlay {
            let tab = tabs::switch_tab(self.read_entries(), tab, step);
            self.overlay = Some(self.index_on(tab));
        }
    }

    fn scroll(&mut self, step: isize, rows: usize, width: usize) {
        match self.overlay {
            Some(Overlay::Notes(first)) => {
                let last = views::notes_scroll_limit(&self.release_notes, rows, width);
                self.overlay = Some(Overlay::Notes(first.saturating_add_signed(step).min(last)));
            }
            Some(Overlay::Calendar(selected)) => {
                self.overlay = Some(Overlay::Calendar(advent_view::step(
                    selected,
                    step * advent_view::COLUMNS as isize,
                )));
            }
            Some(Overlay::Settings(selected)) => {
                let last = self.settings().rows().len() - 1;
                self.overlay = Some(Overlay::Settings(
                    selected.saturating_add_signed(step).min(last),
                ));
            }
            Some(Overlay::Album { .. }) => {
                self.overlay = album_after_scroll(
                    self.overlay,
                    step,
                    &self.progress.answers,
                    &self.progress.badges,
                );
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
        if let Some(Overlay::Index { tab, selected }) = self.overlay
            && let Some(&index) = tabs::entries_in(self.read_entries(), tab).get(selected)
        {
            self.open_letter_from(index, Some((tab, selected)));
        }
    }

    fn press_enter(&mut self) {
        match self.overlay {
            Some(
                Overlay::NewBadge(_)
                | Overlay::NoteBadge(_)
                | Overlay::Notes(_)
                | Overlay::PostmanNote
                | Overlay::Chocolate(_)
                | Overlay::ChocolutinIntroduction(_)
                | Overlay::ChocolutinNote(_),
            ) => self.overlay = closed_overlay(self.overlay),
            Some(Overlay::Settings(_)) => self.change_setting(1),
            Some(Overlay::Letter { .. }) if self.is_diet_letter_waiting() => self.send_diets(),
            _ => self.reread_selected(),
        }
    }

    fn toggle_badges(&mut self) {
        match reveal_badges(self.overlay, &mut self.progress.unseen_badges) {
            Some(revealed) => {
                if !revealed.is_empty() {
                    self.progress.save();
                }
                self.revealed_badges = revealed;
                self.overlay = Some(Overlay::Badges);
            }
            None => self.overlay = None,
        }
    }

    fn toggle_help(&mut self) {
        self.overlay = match self.overlay {
            Some(Overlay::Help) => None,
            _ => Some(Overlay::Help),
        };
    }

    fn toggle_settings(&mut self) {
        self.overlay = match self.overlay {
            Some(Overlay::Settings(_)) => None,
            _ => Some(Overlay::Settings(0)),
        };
    }

    fn settings(&self) -> Settings {
        let location = &self.state.location;
        Settings {
            units: self.state.units,
            mail_hour: self.clock.mail_hour(),
            hide_hud: self.hide_hud,
            city: self
                .state
                .city_name
                .clone()
                .unwrap_or_else(|| format!("{:.2}, {:.2}", location.latitude, location.longitude)),
            auto_location: if self.location_detection.is_some() {
                AutoLocation::Locating
            } else if self.auto_location {
                AutoLocation::On
            } else {
                AutoLocation::Off
            },
            diets: self
                .progress
                .diet_answered_day
                .map(|_| diets_from_ids(&self.progress.diets)),
        }
    }

    fn change_setting(&mut self, step: isize) {
        let Some(Overlay::Settings(selected)) = self.overlay else {
            return;
        };
        let Some(row) = self.settings().rows().get(selected).copied() else {
            return;
        };
        match row {
            Row::City => self.city_search = Some(CitySearch::default()),
            Row::AutoLocation => self.toggle_auto_location(),
            Row::Diet => self.diet_editor = Some(diets_from_ids(&self.progress.diets)),
            row => self.step_setting(row, step),
        }
    }

    fn handle_diet_key(&mut self, code: KeyCode) {
        let Some(diets) = self.diet_editor.as_mut() else {
            return;
        };
        match code {
            KeyCode::Char(key) => {
                if let Some(diet) = diet_for_key(key) {
                    toggle_diet(diets, diet);
                }
            }
            KeyCode::Enter => {
                self.progress.diets = diet_ids(diets);
                self.progress.save();
                self.diet_editor = None;
            }
            KeyCode::Esc => self.diet_editor = None,
            _ => {}
        }
    }

    fn step_setting(&mut self, row: Row, step: isize) {
        let mut settings = self.settings();
        let Some(setting) = settings.stepped(row, step) else {
            return;
        };
        settings.apply(setting);
        if let Setting::MailHour(hour) = setting {
            self.change_mail_hour(hour);
        } else if Config::save(setting).is_err() {
            self.toast_unsaved_setting();
        }
        self.hide_hud = settings.hide_hud;
        self.state.set_units(settings.units);
    }

    fn change_mail_hour(&mut self, hour: u32) {
        if self.clock.is_frozen() {
            return;
        }
        let mut clock = self.clock.clone();
        clock.change_mail_hour(hour, clock::now().naive_local());
        self.progress
            .set_previous_mail_hour(clock.previous_mail_hour());
        if self.progress.try_save().is_ok() && Config::save(Setting::MailHour(hour)).is_ok() {
            self.clock = clock;
        } else {
            self.progress
                .set_previous_mail_hour(self.clock.previous_mail_hour());
            self.toast_unsaved_setting();
        }
    }

    fn toast_unsaved_setting(&mut self) {
        push_toast(
            &mut self.toasts,
            "Réglage non enregistré\u{202F}: vérifie config.toml",
        );
    }

    fn handle_city_key(&mut self, code: KeyCode) {
        let Some(search) = self.city_search.as_mut() else {
            return;
        };
        match code {
            KeyCode::Char(ch) => search.type_char(ch),
            KeyCode::Backspace => search.erase(),
            KeyCode::Up => search.select(-1),
            KeyCode::Down => search.select(1),
            KeyCode::Esc => self.city_search = None,
            KeyCode::Enter => self.submit_city_search(),
            _ => {}
        }
    }

    fn submit_city_search(&mut self) {
        let Some(search) = self.city_search.as_mut() else {
            return;
        };
        if let Some(place) = search.chosen() {
            let location = place.location.clone();
            self.city_search = None;
            self.auto_location = false;
            self.location_detection = None;
            if Config::save_city(&location).is_err() {
                self.toast_unsaved_setting();
            }
            self.state.set_location_display(LocationDisplay::City);
            self.move_to(location);
            return;
        }
        let Some(query) = search.query_to_search().map(str::to_string) else {
            return;
        };
        let language = self.city_name_language.clone();
        let (sender, receiver) = oneshot::channel();
        tokio::spawn(settings::send_unless_cancelled(sender, async move {
            geolocation::search_cities(&query, &language).await
        }));
        search.wait(receiver);
    }

    fn toggle_auto_location(&mut self) {
        if self.auto_location || self.location_detection.is_some() {
            self.auto_location = false;
            self.location_detection = None;
            let location = GeoLocation {
                latitude: self.state.location.latitude,
                longitude: self.state.location.longitude,
                city: self.state.city_name.clone(),
            };
            self.save_location(&location, false);
            return;
        }
        let language = self.city_name_language.clone();
        let (sender, receiver) = oneshot::channel();
        tokio::spawn(settings::send_unless_cancelled(sender, async move {
            geolocation::detect_location_with_city(&language).await
        }));
        self.location_detection = Some(receiver);
    }

    fn receive_city_search(&mut self) {
        if let Some(search) = self.city_search.as_mut() {
            search.receive();
        }
    }

    fn receive_location_detection(&mut self) {
        let Some(receiver) = self.location_detection.as_mut() else {
            return;
        };
        let result = match receiver.try_recv() {
            Ok(result) => result,
            Err(oneshot::error::TryRecvError::Empty) => return,
            Err(oneshot::error::TryRecvError::Closed) => {
                self.location_detection = None;
                return;
            }
        };
        self.location_detection = None;
        match result {
            Ok(DetectedLocation {
                location,
                is_from_device,
            }) => {
                self.auto_location = true;
                self.is_located_by_device |= is_from_device;
                self.save_location(&location, true);
                self.move_to(location);
            }
            Err(_) => push_toast(&mut self.toasts, "Pas de connexion"),
        }
    }

    fn save_location(&mut self, location: &GeoLocation, auto: bool) {
        if Config::save_location(location, auto).is_err() {
            self.toast_unsaved_setting();
        }
    }

    fn move_to(&mut self, location: GeoLocation) {
        let weather_location = WeatherLocation {
            latitude: location.latitude,
            longitude: location.longitude,
            elevation: None,
        };
        if self.fetches_weather {
            let (sender, receiver) = mpsc::channel(1);
            spawn_weather(sender, weather_location, self.state.units);
            self.weather_receiver = receiver;
        }
        if self.rain_receiver.is_some() {
            self.rain_receiver = Some(rain_alert::spawn(location.latitude, location.longitude));
            self.rain_alert = None;
        }
        self.state.set_location(weather_location, location.city);
    }

    fn toggle_album(&mut self) {
        self.overlay = match self.overlay {
            Some(Overlay::Album { .. }) => None,
            _ => Some(Overlay::Album {
                page: 0,
                selected: 0,
            }),
        };
    }

    fn browse(&mut self, step: isize) {
        match self.overlay {
            Some(Overlay::Album { page, selected }) => {
                let last = self.album_pages(selected).len().saturating_sub(1);
                self.overlay = Some(Overlay::Album {
                    page: page.saturating_add_signed(step).min(last),
                    selected: 0,
                });
            }
            Some(Overlay::Letter { .. }) => self.browse_letters(step),
            Some(Overlay::Index { .. }) => self.switch_tab(step),
            Some(Overlay::Settings(_)) => self.change_setting(step),
            Some(Overlay::Calendar(selected)) => {
                self.overlay = Some(Overlay::Calendar(advent_view::step(selected, step)));
            }
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
            self.open_holiday_box();
            self.open_pending_badge();
            self.bring_chocolate();
            self.update_santa_visit();
            self.update_welcome(term_width, term_height);
            self.update_toasts(term_width);
            self.refresh_rain_alert();
            self.receive_city_search();
            self.receive_location_detection();
            self.state.update_clock();
            if self.state.take_weather_info_update() {
                self.weather_bar = WeatherBar::new(&self.state);
            }

            if !self.hide_hud {
                let toolbar = toolbar::line(
                    &self.state,
                    overlay::is_blink_lit(),
                    hud::inner_width(term_width),
                );
                for placed in hud::lay_out(&self.weather_bar, toolbar, term_width, term_height) {
                    render_placed(renderer, &placed)?;
                }
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
                    Event::Key(key_event)
                        if self.city_search.is_some()
                            && !key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        self.handle_city_key(key_event.code);
                    }
                    Event::Key(key_event)
                        if matches!(self.overlay, Some(Overlay::Reindeer))
                            && !key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        self.handle_reindeer_key(key_event.code);
                    }
                    Event::Key(key_event)
                        if self.diet_editor.is_some()
                            && !key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        self.handle_diet_key(key_event.code);
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
                            _ if matches!(self.overlay, Some(Overlay::Backup { .. })) => {
                                self.press_backup_key(key_event.code);
                            }
                            _ if matches!(self.overlay, Some(Overlay::NeighboursNote)) => {
                                self.press_neighbours_note_key(key_event.code);
                            }
                            _ if matches!(self.overlay, Some(Overlay::SantaNote)) => {
                                self.overlay = santa_note_after_key(key_event.code);
                            }
                            KeyCode::Char('c') | KeyCode::Char('C') => self.press_mail(),
                            KeyCode::Char('b') | KeyCode::Char('B') => self.toggle_badges(),
                            KeyCode::Char('v') | KeyCode::Char('V') => self.toggle_calendar(),
                            KeyCode::Char(key @ '1'..='8') => self.press_number(key),
                            KeyCode::Char('a') | KeyCode::Char('A') => self.toggle_album(),
                            KeyCode::Char('h') | KeyCode::Char('H') => self.toggle_help(),
                            KeyCode::Char('s') | KeyCode::Char('S') => self.toggle_settings(),
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
    use crate::animation::trophies::TROPHIES;
    use crate::config::LocationDisplay;
    use crate::progress::DATE_FORMAT;
    use crate::story::punishment::Punishment;
    use crate::weather::WeatherUnits;
    use chrono::{NaiveDate, NaiveDateTime};

    fn card_id(overlay: Option<Overlay>) -> Option<&'static str> {
        badge_card(overlay).map(|award| award.badge.id)
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

    #[test]
    fn test_earned_badges_stay_unseen_until_the_badges_view_opens() {
        let mut earned = BTreeSet::new();
        let mut unseen = BTreeSet::new();
        let awards = earn(&mut earned, vec!["rain", "patience"]);
        mark_unseen(&mut unseen, &awards);
        let mut pending: VecDeque<_> = awards.into();
        while next_overlay(None, &mut pending).is_some() {}
        assert_eq!(
            unseen,
            BTreeSet::from(["patience".to_string(), "rain".to_string()])
        );

        let revealed = reveal_badges(None, &mut unseen);
        assert_eq!(
            revealed,
            Some(BTreeSet::from(["patience".to_string(), "rain".to_string()]))
        );
        assert!(unseen.is_empty());
    }

    #[test]
    fn test_reopening_the_badges_view_reveals_nothing() {
        let mut unseen = BTreeSet::from(["rain".to_string()]);
        reveal_badges(None, &mut unseen);
        assert_eq!(reveal_badges(Some(Overlay::Badges), &mut unseen), None);
        assert_eq!(reveal_badges(None, &mut unseen), Some(BTreeSet::new()));
    }

    #[test]
    fn test_already_earned_badge_is_not_unseen_again() {
        let mut earned = BTreeSet::from(["rain".to_string()]);
        let mut unseen = BTreeSet::new();
        mark_unseen(&mut unseen, &earn(&mut earned, vec!["rain"]));
        assert!(unseen.is_empty());
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

        let letter = Overlay::Letter {
            index: 3,
            from: None,
        };
        let overlay = next_overlay(Some(letter), &mut pending);
        assert!(matches!(overlay, Some(Overlay::Letter { index: 3, .. })));
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
    fn test_letter_reread_from_the_courrier_goes_back_to_it() {
        let reread = Some(Overlay::Letter {
            index: 4,
            from: Some((Tab::Recipes, 1)),
        });
        let from_mail = Some(Overlay::Letter {
            index: 4,
            from: None,
        });

        assert!(matches!(
            closed_overlay(reread),
            Some(Overlay::Index {
                tab: Tab::Recipes,
                selected: 1
            })
        ));
        assert!(closed_overlay(from_mail).is_none());
    }

    #[test]
    fn test_only_a_letter_opened_from_the_courrier_ignores_the_mail_key() {
        assert!(is_reread(Some(Overlay::Letter {
            index: 2,
            from: Some((Tab::Letters, 1)),
        })));
        assert!(!is_reread(Some(Overlay::Letter {
            index: 2,
            from: None,
        })));
        assert!(!is_reread(Some(Overlay::Index {
            tab: Tab::Letters,
            selected: 1,
        })));
        assert!(!is_reread(None));
    }

    #[test]
    fn test_courrier_selection_follows_the_last_browsed_entry() {
        let read = [
            (1, Entry::Main(0)),
            (2, Entry::Gazette(0)),
            (3, Entry::Main(1)),
            (4, Entry::Main(2)),
            (9, Entry::Main(3)),
        ];
        let (selected, _) = tabs::entry_beside(&read, Tab::Letters, 1, 1).unwrap();
        let (selected, index) = tabs::entry_beside(&read, Tab::Letters, selected, 1).unwrap();
        assert_eq!(index, 4);

        assert!(matches!(
            closed_overlay(Some(Overlay::Letter {
                index,
                from: Some((Tab::Letters, selected)),
            })),
            Some(Overlay::Index {
                tab: Tab::Letters,
                selected: 3
            })
        ));
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
            opening_overlay(false, true, notes_overlay(true)),
            Some(Overlay::Welcome(Step::Arriving))
        ));
        assert!(matches!(
            opening_overlay(false, false, notes_overlay(true)),
            Some(Overlay::Notes(0))
        ));
        assert!(opening_overlay(false, false, notes_overlay(false)).is_none());

        let mut pending: VecDeque<_> = earn(&mut BTreeSet::new(), vec!["rain"]).into();
        let overlay = next_overlay(opening_overlay(false, true, None), &mut pending);
        assert!(matches!(overlay, Some(Overlay::Welcome(_))));
        assert_eq!(pending.len(), 1);
    }

    #[test]
    fn test_chocolutin_comes_once_on_the_first_launch_of_an_advent_day() {
        let mut progress = Progress::default();
        assert_eq!(
            chocolate_due_on_launch(false, None, &progress, date(2026, 12, 1)),
            Some(1)
        );
        progress.receive_advent_day(2026, 1);
        assert_eq!(
            chocolate_due_on_launch(false, None, &progress, date(2026, 12, 1)),
            None
        );
        assert_eq!(
            chocolate_due_on_launch(false, None, &progress, date(2026, 12, 24)),
            Some(24)
        );
        for (month, day) in [(11, 30), (12, 25), (12, 26)] {
            assert_eq!(
                chocolate_due_on_launch(false, None, &progress, date(2026, month, day)),
                None
            );
        }
    }

    #[test]
    fn test_chocolutin_stays_home_in_simulations() {
        let progress = Progress::default();
        assert_eq!(
            chocolate_due_on_launch(true, None, &progress, date(2026, 12, 2)),
            None
        );
        assert_eq!(
            chocolate_due_on_launch(false, NonZeroU16::new(3), &progress, date(2026, 12, 2)),
            None
        );
    }

    #[test]
    fn test_chocolutin_leaves_a_note_for_the_day_the_game_stayed_closed() {
        let mut progress = Progress::default();
        progress.receive_advent_day(2026, 1);
        let today = date(2026, 12, 3);
        assert_eq!(chocolutin_notes(true, &progress, today), vec![2]);
        assert_eq!(
            chocolate_due_on_launch(false, None, &progress, today),
            Some(3)
        );
        progress.receive_advent_day(2026, 3);
        assert_eq!(chocolutin_notes(true, &progress, today), vec![2]);
    }

    #[test]
    fn test_reading_a_note_gives_its_chocolate_to_the_calendar() {
        let mut progress = Progress::default();
        let today = date(2026, 12, 5);
        assert_eq!(chocolutin_notes(true, &progress, today), vec![1, 2, 3, 4]);
        progress.receive_advent_day(2026, 2);
        assert_eq!(chocolutin_notes(true, &progress, today), vec![1, 3, 4]);
        let cells = advent::cells(&progress.advent_days(2026), today);
        assert!(matches!(cells[1], advent::Cell::Received(_)));
        assert!(matches!(cells[0], advent::Cell::Missed));
    }

    #[test]
    fn test_chocolutin_leaves_no_note_in_simulations() {
        assert!(chocolutin_notes(false, &Progress::default(), date(2026, 12, 5)).is_empty());
    }

    #[test]
    fn test_chocolutin_waits_for_the_launch_cards_then_gives_the_chocolate() {
        let holiday_box = Holiday::christmas().holiday_box().map(Overlay::HolidayBox);
        assert_eq!(
            chocolutin_move(Some(24), holiday_box, false, false),
            ChocolutinMove::Wait
        );
        assert_eq!(
            chocolutin_move(Some(24), Some(Overlay::Notes(0)), false, false),
            ChocolutinMove::Wait
        );
        assert_eq!(
            chocolutin_move(Some(24), None, false, false),
            ChocolutinMove::Visit
        );
        assert_eq!(
            chocolutin_move(Some(24), None, true, false),
            ChocolutinMove::Wait
        );
        assert_eq!(
            chocolutin_move(Some(24), holiday_box, true, true),
            ChocolutinMove::Wait
        );
        assert_eq!(
            chocolutin_move(Some(24), None, true, true),
            ChocolutinMove::Give(24)
        );
        assert_eq!(
            chocolutin_move(None, Some(Overlay::Chocolate(24)), true, true),
            ChocolutinMove::Wait
        );
        assert_eq!(
            chocolutin_move(None, Some(Overlay::ChocolutinIntroduction(24)), true, true),
            ChocolutinMove::Wait
        );
        assert_eq!(
            chocolutin_move(None, None, true, true),
            ChocolutinMove::Leave
        );
    }

    #[test]
    fn test_chocolutin_introduces_himself_once_before_the_chocolate() {
        let first = chocolate_overlay(3, false);
        assert!(matches!(first, Overlay::ChocolutinIntroduction(3)));
        assert!(matches!(
            closed_overlay(Some(first)),
            Some(Overlay::Chocolate(3))
        ));
        assert!(matches!(chocolate_overlay(4, true), Overlay::Chocolate(4)));
        assert!(closed_overlay(Some(Overlay::Chocolate(4))).is_none());
    }

    #[test]
    fn test_calendar_key_works_only_in_december() {
        assert!(toggled_calendar(None, false, date(2026, 11, 30)).is_none());
        assert!(matches!(
            toggled_calendar(Some(Overlay::Help), false, date(2026, 1, 4)),
            Some(Overlay::Help)
        ));
        assert!(matches!(
            toggled_calendar(None, true, date(2026, 12, 9)),
            Some(Overlay::Calendar(9))
        ));
        assert!(matches!(
            toggled_calendar(Some(Overlay::Help), true, date(2026, 12, 31)),
            Some(Overlay::Calendar(24))
        ));
        assert!(toggled_calendar(Some(Overlay::Calendar(3)), true, date(2026, 12, 9)).is_none());
    }

    #[test]
    fn test_santa_comes_on_a_real_christmas_launch_only() {
        let christmas = NaiveDate::from_ymd_opt(2026, 12, 25).unwrap();
        let progress = Progress::default();
        assert!(santa_due_on_launch(true, &progress, christmas));
        assert!(!santa_due_on_launch(false, &progress, christmas));
        assert!(!santa_due_on_launch(
            true,
            &progress,
            christmas.succ_opt().unwrap()
        ));
    }

    #[test]
    fn test_santa_note_opens_once_the_sleigh_has_left_until_the_reindeer_is_named() {
        let christmas = NaiveDate::from_ymd_opt(2026, 12, 25).unwrap();
        let mut progress = Progress::default();
        progress.welcome_reindeer(christmas);
        let awaits = progress.awaits_reindeer_name(christmas);
        assert!(santa_visit_overlay(true, true, awaits).is_none());
        assert!(santa_visit_overlay(false, false, awaits).is_none());
        assert!(matches!(
            santa_visit_overlay(true, false, awaits),
            Some(Overlay::SantaNote)
        ));
        progress.name_reindeer("Comète");
        let awaits = progress.awaits_reindeer_name(christmas);
        assert!(santa_visit_overlay(true, false, awaits).is_none());
        let next_day = christmas.succ_opt().unwrap();
        let mut unnamed = Progress::default();
        unnamed.welcome_reindeer(christmas);
        assert!(santa_visit_overlay(true, false, unnamed.awaits_reindeer_name(next_day)).is_none());
    }

    #[test]
    fn test_santa_note_leads_to_the_naming_card_only() {
        for code in [KeyCode::Enter, KeyCode::Esc] {
            assert!(matches!(
                santa_note_after_key(code),
                Some(Overlay::Reindeer)
            ));
        }
        for code in [KeyCode::Char('c'), KeyCode::Char('a'), KeyCode::Left] {
            assert!(matches!(
                santa_note_after_key(code),
                Some(Overlay::SantaNote)
            ));
        }
    }

    fn holiday_box_title(overlay: Option<Overlay>) -> Option<&'static str> {
        match overlay {
            Some(Overlay::HolidayBox(holiday_box)) => Some(holiday_box.title),
            _ => None,
        }
    }

    #[test]
    fn test_holiday_box_opens_after_the_launch_windows_and_before_badge_cards() {
        let mut holiday_box = Holiday::music_day().holiday_box();
        let mut badges: VecDeque<_> = earn(&mut BTreeSet::new(), vec!["music_day"]).into();

        let backup = holiday_box_after_launch(
            opening_overlay(true, true, notes_overlay(true)),
            &mut holiday_box,
        );
        assert!(matches!(
            backup,
            Some(Overlay::Backup {
                shows_welcome: true
            })
        ));
        let welcome = holiday_box_after_launch(
            backup_after_key(KeyCode::Esc, true, notes_overlay(true)),
            &mut holiday_box,
        );
        assert!(matches!(welcome, Some(Overlay::Welcome(Step::Arriving))));
        let notes = holiday_box_after_launch(notes_overlay(true), &mut holiday_box);
        assert!(matches!(notes, Some(Overlay::Notes(0))));
        assert!(holiday_box.is_some());

        let shown = next_overlay(
            holiday_box_after_launch(notes_overlay(false), &mut holiday_box),
            &mut badges,
        );
        assert_eq!(holiday_box_title(shown), Some("Fête de la musique"));
        assert_eq!(badges.len(), 1);

        let closed = holiday_box_after_launch(closed_overlay(shown), &mut holiday_box);
        assert!(closed.is_none());
        assert_eq!(
            card_id(next_overlay(closed, &mut badges)),
            Some("music_day")
        );
    }

    #[test]
    fn test_backup_box_opens_before_the_welcome_and_the_notes() {
        let mut pending: VecDeque<_> = earn(&mut BTreeSet::new(), vec!["rain"]).into();
        let overlay = next_overlay(
            opening_overlay(true, false, notes_overlay(true)),
            &mut pending,
        );
        assert!(matches!(
            overlay,
            Some(Overlay::Backup {
                shows_welcome: false
            })
        ));
        assert_eq!(pending.len(), 1);

        assert!(matches!(
            backup_after_key(KeyCode::Char('h'), false, notes_overlay(true)),
            Some(Overlay::Backup { .. })
        ));
        for code in [KeyCode::Esc, KeyCode::Enter] {
            assert!(matches!(
                backup_after_key(code, false, notes_overlay(true)),
                Some(Overlay::Notes(0))
            ));
        }
        assert!(matches!(
            backup_after_key(KeyCode::Esc, true, notes_overlay(true)),
            Some(Overlay::Welcome(Step::Arriving))
        ));
        assert!(backup_after_key(KeyCode::Enter, false, None).is_none());
    }

    #[test]
    fn test_neighbours_note_opens_after_backup_and_welcome_before_the_notes() {
        assert!(matches!(
            after_welcome(true, true),
            Some(Overlay::NeighboursNote)
        ));
        assert!(matches!(
            after_welcome(false, true),
            Some(Overlay::Notes(0))
        ));
        assert!(after_welcome(false, false).is_none());

        assert!(matches!(
            opening_overlay(false, false, after_welcome(true, true)),
            Some(Overlay::NeighboursNote)
        ));
        assert!(matches!(
            opening_overlay(false, true, after_welcome(true, true)),
            Some(Overlay::Welcome(_))
        ));
        assert!(matches!(
            backup_after_key(KeyCode::Enter, false, after_welcome(true, true)),
            Some(Overlay::NeighboursNote)
        ));

        assert!(matches!(
            neighbours_note_after_key(KeyCode::Char('c'), true),
            Some(Overlay::NeighboursNote)
        ));
        for code in [KeyCode::Esc, KeyCode::Enter] {
            assert!(matches!(
                neighbours_note_after_key(code, true),
                Some(Overlay::Notes(0))
            ));
            assert!(neighbours_note_after_key(code, false).is_none());
        }
    }

    fn cheater(cheats: u32) -> Progress {
        let mut progress = Progress::default();
        progress.started_on = Some("2026-10-01".to_string());
        progress.resyncs = cheats;
        progress
    }

    #[test]
    fn test_neighbours_note_announces_each_new_cheat_once() {
        let mut progress = cheater(1);
        let mut clock = StoryClock::for_progress(&progress, 9);
        let mut rng = rand::rng();
        let mut launch = |progress: &mut Progress, clock: &mut StoryClock, today| {
            neighbours_note_on_launch(None, progress, clock, today, &mut rng)
        };

        assert_eq!(
            launch(&mut progress, &mut clock, date(2026, 10, 10)),
            Some(NeighboursNote::Announcement(Punishment::Cheater))
        );
        assert_eq!(launch(&mut progress, &mut clock, date(2026, 10, 10)), None);
        assert!(progress.badges.contains(badges::CHEATER));

        progress.resyncs = 2;
        assert_eq!(
            launch(&mut progress, &mut clock, date(2026, 10, 11)),
            Some(NeighboursNote::Announcement(Punishment::BareVillage))
        );
        assert_eq!(launch(&mut progress, &mut clock, date(2026, 10, 12)), None);
    }

    #[test]
    fn test_neighbours_note_comes_back_disappointed_until_the_mail_returns() {
        let mut progress = cheater(3);
        progress.punished_cheats = 2;
        let mut clock = StoryClock::for_progress(&progress, 9);
        let mut rng = rand::rng();
        let mut launch = |progress: &mut Progress, clock: &mut StoryClock, today| {
            neighbours_note_on_launch(None, progress, clock, today, &mut rng)
        };

        assert_eq!(
            launch(&mut progress, &mut clock, date(2026, 10, 10)),
            Some(NeighboursNote::Announcement(Punishment::MailBan(3)))
        );
        for day in [10, 11, 13] {
            assert!(matches!(
                launch(&mut progress, &mut clock, date(2026, 10, day)),
                Some(NeighboursNote::Disappointed(_))
            ));
        }
        assert_eq!(launch(&mut progress, &mut clock, date(2026, 10, 14)), None);

        let relaunched = StoryClock::for_progress(&progress, 9);
        assert_eq!(relaunched.day_of(date(2026, 10, 13)), 10);
        assert_eq!(relaunched.day_of(date(2026, 10, 14)), 11);
    }

    #[test]
    fn test_forced_day_never_punishes() {
        let mut progress = cheater(3);
        let mut clock = StoryClock::for_progress(&progress, 9);
        let note = neighbours_note_on_launch(
            NonZeroU16::new(43),
            &mut progress,
            &mut clock,
            date(2026, 10, 10),
            &mut rand::rng(),
        );
        assert_eq!(note, None);
        assert_eq!(progress.punished_cheats, 0);
        assert!(progress.badges.is_empty());
        assert!(progress.paused_days.is_empty());
        assert!(clock.pauses().is_empty());
        assert!(!punishment::is_village_bare(
            &progress,
            10,
            date(2026, 10, 10)
        ));
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

    const ARRIVAL_DAYS: [(Thing, usize); 30] = [
        (Thing::Cat, 3),
        (Thing::Bench, 4),
        (Thing::CatBow, 6),
        (Thing::Footprints, 8),
        (Thing::Dog, 9),
        (Thing::Owl, 12),
        (Thing::Well, 13),
        (Thing::Bone, 15),
        (Thing::Key, 17),
        (Thing::Duck, 18),
        (Thing::Snail, 21),
        (Thing::Treehouse, 22),
        (Thing::OwlFriend, 24),
        (Thing::BlueLantern, 26),
        (Thing::Squirrel, 27),
        (Thing::Frog, 30),
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
            Thing::Cat => unlocks.cat,
            Thing::Bench => unlocks.bench,
            Thing::CatBow => unlocks.cat_bow,
            Thing::Footprints => unlocks.footprints,
            Thing::Dog => unlocks.dog,
            Thing::Owl => unlocks.owl,
            Thing::Well => unlocks.well,
            Thing::Bone => unlocks.bone,
            Thing::Key => unlocks.key,
            Thing::Duck => unlocks.duck,
            Thing::Snail => unlocks.snail,
            Thing::Treehouse => unlocks.treehouse,
            Thing::OwlFriend => unlocks.owl_friend,
            Thing::BlueLantern => unlocks.blue_lantern,
            Thing::Squirrel => unlocks.squirrel,
            Thing::Frog => unlocks.frog,
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
        Timeline::for_village(letters::defaults().len(), &clock(), day, Recipes::default())
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

    const HABITANTS: [Thing; 7] = [
        Thing::Cat,
        Thing::Dog,
        Thing::Owl,
        Thing::Duck,
        Thing::Snail,
        Thing::Squirrel,
        Thing::Frog,
    ];

    #[test]
    fn test_first_letters_match_the_default_story() {
        let senders = [
            (Neighbour::Rose, "Rose, la boulangère"),
            (Neighbour::Paulo, "Paulo, le pêcheur"),
            (Neighbour::Odile, "Odile, la maire"),
            (Neighbour::Leo, "Léo, le petit voisin"),
            (Neighbour::Gaston, "Gaston, le facteur"),
        ];
        let defaults = letters::defaults();
        for (neighbour, sender) in senders {
            let first = defaults
                .iter()
                .position(|letter| letter.sender == sender)
                .map(|index| index + 1);
            let expected = NEIGHBOUR_FIRST_LETTERS
                .iter()
                .find(|(written, _)| *written == neighbour)
                .map(|(_, number)| *number);
            assert_eq!(first, expected, "{neighbour:?}");
        }
    }

    #[test]
    fn test_guests_come_once_their_first_letter_is_delivered() {
        let guests = |day| guests_on(&timeline_on(day), day);
        assert_eq!(guests(1), [Neighbour::Rose]);
        assert_eq!(guests(3), [Neighbour::Rose]);
        assert_eq!(
            guests(15),
            [
                Neighbour::Rose,
                Neighbour::Paulo,
                Neighbour::Odile,
                Neighbour::Gaston
            ]
        );
        let everyone = guests(16);
        assert_eq!(everyone.len(), Neighbour::ALL.len());
        assert!(Neighbour::ALL.iter().all(|guest| everyone.contains(guest)));
    }

    const FOOTPRINT_MAIL: [(Entry, &str); 3] = [
        (Entry::Main(7), "Regarde le sol"),
        (Entry::Main(10), "Ces traces sont petites"),
        (Entry::Main(15), "Pardon pour la boue"),
    ];

    const WET_FOOTPRINT_MAIL: [(Entry, &str); 2] = [
        (Entry::Creepy(2), "Les traces vont du puits à ta porte"),
        (Entry::Marcel(2), "Des pas vers ta maison"),
    ];

    fn mail_text(entry: Entry) -> String {
        match entry {
            Entry::Main(index) => letters::defaults()[index].body.clone(),
            Entry::Creepy(index) => letters::CREEPY_LETTERS[index].to_string(),
            Entry::Marcel(index) => chronicles::MARCEL_LETTERS[index].to_string(),
            other => panic!("no footprint mail in {other:?}"),
        }
    }

    fn delivery_days(mail: &[(Entry, &str)]) -> Vec<usize> {
        let timeline = timeline_on(120);
        mail.iter()
            .map(|(entry, _)| {
                timeline
                    .mail()
                    .find(|(_, delivered)| delivered == entry)
                    .map(|(day, _)| day)
                    .unwrap_or_else(|| panic!("{entry:?} never delivered"))
            })
            .collect()
    }

    #[test]
    fn test_footprint_mail_talks_about_footprints() {
        for (entry, quote) in FOOTPRINT_MAIL.iter().chain(&WET_FOOTPRINT_MAIL) {
            assert!(mail_text(*entry).contains(quote), "{entry:?}");
        }
    }

    #[test]
    fn test_footprints_show_only_on_the_days_their_mail_arrives() {
        for (thing, mail) in [
            (Thing::Footprints, &FOOTPRINT_MAIL[..]),
            (Thing::WetFootprints, &WET_FOOTPRINT_MAIL[..]),
        ] {
            let days = delivery_days(mail);
            for day in 1..=120 {
                assert_eq!(
                    shows(&unlocks_on_day(day), thing),
                    days.contains(&day),
                    "{thing:?} on day {day}"
                );
            }
        }
    }

    #[test]
    fn test_arrivals_ignore_reading() {
        let unlocks = unlocks_on(&timeline_on(30), 30, &BTreeMap::new());
        assert!(unlocks.bench && unlocks.well && unlocks.treehouse);

        let unlocks = unlocks_on(&timeline_on(10), 10, &BTreeMap::new());
        assert!(unlocks.cat && unlocks.dog);
    }

    #[test]
    fn test_no_habitant_on_first_day() {
        let unlocks = unlocks_on_day(1);
        assert!(!HABITANTS.iter().any(|&habitant| shows(&unlocks, habitant)));
    }

    #[test]
    fn test_late_player_finds_every_habitant() {
        let unlocks = unlocks_on_day(41);
        assert!(HABITANTS.iter().all(|&habitant| shows(&unlocks, habitant)));
    }

    #[test]
    fn test_forced_mode_shows_every_habitant_unless_day_is_set() {
        assert!(shows_every_habitant(true, None));
        assert!(!shows_every_habitant(true, NonZeroU16::new(5)));
        assert!(!shows_every_habitant(false, None));
    }

    #[test]
    fn test_every_habitant_leaves_objects_locked() {
        let unlocks = unlocks_on_day(1).with_every_habitant();
        assert!(HABITANTS.iter().all(|&habitant| shows(&unlocks, habitant)));
        assert!(!unlocks.bench);
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
            let bar = WeatherBar::new(&state);
            assert_eq!(bar.right[1][0].text, format!("Jour {day}"));
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
            first.iter().map(|award| award.badge.id).collect::<Vec<_>>(),
            ["summer_holidays"]
        );
        assert!(earn(&mut earned, vec!["summer_holidays"]).is_empty());
    }

    #[test]
    fn test_frozen_clock_never_fills_the_box() {
        let frozen = StoryClock::frozen(date(2026, 10, 26), 9, NonZeroU16::new(7).unwrap());
        let timeline =
            Timeline::for_village(letters::defaults().len(), &frozen, 7, Recipes::default());
        assert!(!is_live_box_full(&frozen, &timeline, 7, 0));
    }

    #[test]
    fn test_fresh_progress_starts_on_day_one() {
        let mut progress = Progress::default();
        progress.started_on = Some(clock::now().format(DATE_FORMAT).to_string());
        for mail_hour in [0, 23] {
            assert_eq!(StoryClock::for_progress(&progress, mail_hour).today(), 1);
        }
    }

    #[test]
    fn test_relaunch_keeps_the_saved_previous_mail_hour() {
        let mut progress = Progress::default();
        progress.started_on = Some("2026-10-01".to_string());
        progress.set_previous_mail_hour(Some((date(2026, 10, 9), 9)));
        let clock = StoryClock::for_progress(&progress, 11);
        let at = |day, hour, minute| date(2026, 10, day).and_hms_opt(hour, minute, 0).unwrap();
        assert_eq!(clock.day_on(at(8, 10, 30)), 8);
        assert_eq!(clock.day_on(at(9, 10, 59)), 8);
        assert_eq!(clock.day_on(at(9, 11, 0)), 9);
    }

    #[test]
    fn test_identical_toast_is_queued_once() {
        let mut toasts = VecDeque::new();
        push_toast(&mut toasts, "A");
        push_toast(&mut toasts, "B");
        push_toast(&mut toasts, "A");
        assert_eq!(toasts, ["A", "B"]);
    }

    fn detected(city: &str, latitude: f64, longitude: f64) -> GeoLocation {
        GeoLocation {
            latitude,
            longitude,
            city: Some(city.to_string()),
        }
    }

    fn album(page: usize, selected: usize) -> Option<Overlay> {
        Some(Overlay::Album { page, selected })
    }

    fn album_position(overlay: Option<Overlay>) -> Option<(usize, usize)> {
        match overlay {
            Some(Overlay::Album { page, selected }) => Some((page, selected)),
            _ => None,
        }
    }

    #[test]
    fn test_launch_detection_records_the_city() {
        let mut progress = Progress::default();
        let lyon = detected("Lyon", 45.76, 4.84);
        assert!(record_trip(&mut progress, Some(&lyon), None).is_new);
        assert!(!record_trip(&mut progress, Some(&lyon), None).is_new);
        assert_eq!(progress.visited_cities.len(), 1);
    }

    #[test]
    fn test_no_launch_detection_records_nothing() {
        let mut progress = Progress::default();
        assert!(!record_trip(&mut progress, None, None).is_new);
        assert!(progress.visited_cities.is_empty());
    }

    #[test]
    fn test_day_flag_records_nothing() {
        let mut progress = Progress::default();
        let lyon = detected("Lyon", 45.76, 4.84);
        assert!(!record_trip(&mut progress, Some(&lyon), NonZeroU16::new(3)).is_new);
        assert!(progress.visited_cities.is_empty());
    }

    fn launch(
        progress: &mut Progress,
        launch_location: Option<&GeoLocation>,
        day: Option<NonZeroU16>,
    ) -> Vec<Award> {
        record_trip(progress, launch_location, day).tier_cards
    }

    fn lisbon() -> GeoLocation {
        detected("Lisbon", 38.7223, -9.1393)
    }

    #[test]
    fn test_first_launch_in_a_sam_city_earns_chocolate() {
        let mut progress = Progress::default();
        let cards = launch(&mut progress, Some(&lisbon()), None);
        assert_eq!(cards.len(), 1);
        assert!(!cards[0].is_new_tier());
        assert_eq!(cards[0].badge.id, SAM_TRAIL.id);
        assert_eq!(cards[0].name(), "Sur les traces de Sam · chocolat");
        assert!(progress.badges.contains(SAM_TRAIL.id));
    }

    #[test]
    fn test_same_sam_city_again_opens_no_card() {
        let mut progress = Progress::default();
        launch(&mut progress, Some(&lisbon()), None);
        assert!(launch(&mut progress, Some(&lisbon()), None).is_empty());
        assert_eq!(progress.sam_cities.len(), 1);
    }

    #[test]
    fn test_far_from_every_sam_city_changes_nothing() {
        let mut progress = Progress::default();
        let lyon = detected("Lyon", 45.76, 4.84);
        assert!(launch(&mut progress, Some(&lyon), None).is_empty());
        assert!(progress.sam_cities.is_empty());
        assert!(!progress.badges.contains(SAM_TRAIL.id));
    }

    #[test]
    fn test_lisbon_not_detected_at_launch_or_on_day_flag_changes_nothing() {
        let mut progress = Progress::default();
        assert!(launch(&mut progress, None, None).is_empty());
        assert!(launch(&mut progress, Some(&lisbon()), NonZeroU16::new(3)).is_empty());
        assert!(progress.sam_cities.is_empty());
        assert!(progress.visited_cities.is_empty());
        assert!(progress.badges.is_empty());
    }

    #[test]
    fn test_each_tier_opens_one_new_tier_card() {
        let mut progress = Progress::default();
        let mut cards = Vec::new();
        for (index, city) in CITIES.iter().enumerate() {
            let location = detected(city.name, city.latitude, city.longitude);
            for card in launch(&mut progress, Some(&location), None) {
                cards.push((index + 1, card.is_new_tier(), card.name(), card.hint()));
            }
        }
        let expected = [
            (1, false, "chocolat", "visité la même ville que Sam"),
            (5, true, "bronze", "5 villes de Sam visitées"),
            (15, true, "argent", "15 villes de Sam visitées"),
            (30, true, "or", "30 villes de Sam visitées"),
            (40, true, "platine", "40 villes de Sam visitées"),
        ]
        .map(|(count, is_new_tier, tier, hint)| {
            (
                count,
                is_new_tier,
                format!("Sur les traces de Sam · {tier}"),
                Some(hint.to_string()),
            )
        });
        assert_eq!(cards, expected);
    }

    #[test]
    fn test_new_tier_marks_sam_trail_unseen() {
        let mut progress = Progress::default();
        launch(&mut progress, Some(&lisbon()), None);
        assert_eq!(
            progress.unseen_badges,
            BTreeSet::from([SAM_TRAIL.id.to_string()])
        );
    }

    #[test]
    fn test_several_tiers_crossed_at_once_open_one_card_each() {
        let mut earned = BTreeSet::new();
        let cards = earn_tiers(&mut earned, 0, 5);
        let names: Vec<_> = cards.iter().map(Award::name).collect();
        assert_eq!(
            names,
            [
                "Sur les traces de Sam · chocolat",
                "Sur les traces de Sam · bronze"
            ]
        );
        assert_eq!(earned, BTreeSet::from([SAM_TRAIL.id.to_string()]));
    }

    #[test]
    fn test_album_selection_stays_within_answered_questions() {
        let answers = BTreeMap::from([("bow".to_string(), 1), ("well".to_string(), 2)]);
        let page = views::ANSWERS_PAGE;
        let no_badge = BTreeSet::new();
        let scroll =
            |overlay, step| album_position(album_after_scroll(overlay, step, &answers, &no_badge));
        assert_eq!(scroll(album(page, 0), -1), Some((page, 0)));
        assert_eq!(scroll(album(page, 0), 1), Some((page, 1)));
        assert_eq!(scroll(album(page, 1), 1), Some((page, 1)));
        assert_eq!(scroll(album(page, 1), -1), Some((page, 0)));
        assert_eq!(scroll(album(0, 0), 1), Some((0, 0)));

        let nothing_answered = BTreeMap::new();
        for step in [-1, 1] {
            assert_eq!(
                album_position(album_after_scroll(
                    album(page, 0),
                    step,
                    &nothing_answered,
                    &no_badge
                )),
                Some((page, 0))
            );
        }
    }

    #[test]
    fn test_option_keys_change_the_selected_answer_on_the_answers_page_only() {
        let timeline = timeline_on(60);
        let mut answers = BTreeMap::from([("bow".to_string(), 1), ("well".to_string(), 2)]);
        let selected = album(views::ANSWERS_PAGE, 0);
        for (option, is_bow_blue) in [(2, true), (1, false), (2, true)] {
            assert!(answer_option(selected, &mut answers, None, option));
            assert_eq!(
                unlocks_on(&timeline, 60, &answers).choices.bow_blue,
                is_bow_blue
            );
        }
        assert_eq!(answers["bow"], 2);
        assert_eq!(answers["well"], 2);
        assert_eq!(
            album_choice(album(views::ANSWERS_PAGE, 1), &answers).map(|choice| choice.id),
            Some("well")
        );

        let before = answers.clone();
        assert!(!answer_option(album(0, 0), &mut answers, None, 1));
        assert!(!answer_option(
            album(views::TROPHIES_PAGE, 0),
            &mut answers,
            None,
            1
        ));
        assert!(!answer_option(
            None,
            &mut answers,
            letters::choice_for(3),
            1
        ));
        assert!(!answer_option(
            Some(Overlay::Letter {
                index: 2,
                from: None,
            }),
            &mut answers,
            letters::choice_for(3),
            1
        ));
        assert_eq!(answers, before);

        let mut nothing_answered = BTreeMap::new();
        assert!(!answer_option(
            album(views::ANSWERS_PAGE, 0),
            &mut nothing_answered,
            None,
            1
        ));
        assert!(answer_option(
            Some(Overlay::Letter {
                index: 2,
                from: None,
            }),
            &mut nothing_answered,
            letters::choice_for(3),
            2
        ));
        assert_eq!(nothing_answered, BTreeMap::from([("bow".to_string(), 2)]));
    }

    #[test]
    fn test_trophy_page_selection_stays_within_won_trophies() {
        let page = views::TROPHIES_PAGE;
        let answers = BTreeMap::new();
        let scroll = |badges: &BTreeSet<String>, selected, step| {
            album_position(album_after_scroll(
                album(page, selected),
                step,
                &answers,
                badges,
            ))
        };
        let both = BTreeSet::from(["streak_60".to_string(), "streak_100".to_string()]);
        assert_eq!(scroll(&both, 0, 1), Some((page, 1)));
        assert_eq!(scroll(&both, 1, 1), Some((page, 1)));
        assert_eq!(scroll(&both, 1, -1), Some((page, 0)));

        let gnome_only = BTreeSet::from(["streak_60".to_string()]);
        assert_eq!(scroll(&gnome_only, 0, 1), Some((page, 0)));
        assert_eq!(scroll(&BTreeSet::new(), 0, 1), Some((page, 0)));
    }

    #[test]
    fn test_option_keys_put_the_selected_trophy_away_and_back() {
        let badges = BTreeSet::from([
            "streak_60".to_string(),
            "streak_100".to_string(),
            "streak_365".to_string(),
        ]);
        let mut hidden = BTreeSet::new();
        let shown = |hidden: &BTreeSet<String>| Trophy::shown(&badges, hidden);

        assert!(place_trophy(
            album(views::TROPHIES_PAGE, 0),
            &badges,
            &mut hidden,
            2
        ));
        assert_eq!(shown(&hidden), [Trophy::Statue, Trophy::Sundial]);
        assert!(place_trophy(
            album(views::TROPHIES_PAGE, 1),
            &badges,
            &mut hidden,
            2
        ));
        assert_eq!(shown(&hidden), [Trophy::Sundial]);
        assert!(place_trophy(
            album(views::TROPHIES_PAGE, 2),
            &badges,
            &mut hidden,
            2
        ));
        assert!(shown(&hidden).is_empty());
        assert!(place_trophy(
            album(views::TROPHIES_PAGE, 2),
            &badges,
            &mut hidden,
            1
        ));
        assert_eq!(shown(&hidden), [Trophy::Sundial]);
        assert!(place_trophy(
            album(views::TROPHIES_PAGE, 0),
            &badges,
            &mut hidden,
            1
        ));
        assert_eq!(shown(&hidden), [Trophy::GardenGnome, Trophy::Sundial]);
        assert!(place_trophy(
            album(views::TROPHIES_PAGE, 1),
            &badges,
            &mut hidden,
            1
        ));
        assert_eq!(shown(&hidden), TROPHIES);

        assert!(!place_trophy(
            album(views::ANSWERS_PAGE, 0),
            &badges,
            &mut hidden,
            2
        ));
        assert!(!place_trophy(None, &badges, &mut hidden, 2));
        let gnome_only = BTreeSet::from(["streak_60".to_string()]);
        assert!(!place_trophy(
            album(views::TROPHIES_PAGE, 1),
            &gnome_only,
            &mut hidden,
            2
        ));
        assert!(hidden.is_empty());
    }

    #[test]
    fn test_old_save_without_trophy_choice_shows_won_trophies() {
        let progress: Progress = serde_json::from_str(
            r#"{"badges":["streak_30","streak_60","streak_100","streak_365"]}"#,
        )
        .unwrap();
        assert!(progress.hidden_trophies.is_empty());
        assert_eq!(
            Trophy::shown(&progress.badges, &progress.hidden_trophies),
            TROPHIES
        );
    }

    #[test]
    fn test_trophy_choice_survives_a_save_and_reload() {
        let mut progress = Progress::default();
        progress.badges.insert("streak_60".to_string());
        progress
            .hidden_trophies
            .insert(Trophy::GardenGnome.id().to_string());
        let reloaded: Progress =
            serde_json::from_str(&serde_json::to_string(&progress).unwrap()).unwrap();
        assert!(Trophy::shown(&reloaded.badges, &reloaded.hidden_trophies).is_empty());
    }
}
