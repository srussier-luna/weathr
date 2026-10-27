use crate::animation::sun_path::day_progress;
use crate::animation::{
    AnimationController,
    airplanes::AirplaneSystem,
    birds::BirdSystem,
    cat::Cat,
    chimney::ChimneySmoke,
    cloud_bank::CloudBank,
    clouds::CloudSystem,
    constellation::StarGazing,
    creepy::Creepy,
    dog::Dog,
    duck::Duck,
    epiphany,
    finds::Finds,
    fireflies::FireflySystem,
    fireworks::Fireworks,
    fog::FogSystem,
    frog::Frog,
    garden::{self, GardenLayout},
    gust::Gust,
    halloween::Halloween,
    kid::Kid,
    leaves::FallingLeaves,
    lore::{Lore, Unlocks},
    moon::MoonSystem,
    new_year::NewYearBanner,
    owl::Owl,
    postman::Postman,
    rainbow::Rainbow,
    raindrops::RaindropSystem,
    snail::Snail,
    snow::SnowSystem,
    snow_cover::SnowCover,
    snowman::Snowman,
    spring::Spring,
    squirrel::Squirrel,
    stars::{ShootingStarSky, StarSystem},
    sunny::SunnyAnimation,
    thunderstorm::ThunderstormSystem,
};
use crate::app_state::AppState;
use crate::holiday::{Holiday, Season};
use crate::render::TerminalRenderer;
use crate::scene::WorldScene;
use crate::scene::house::House;
use crate::scene::river::River;
use crate::weather::{FogIntensity, RainIntensity, SnowIntensity, WeatherConditions};
use crossterm::style::Color;
use std::io;
use std::time::{Duration, Instant};

const FRAME_DELAY: Duration = Duration::from_millis(500);
const SUN_MARGIN: u16 = 2;
const SUN_LOWEST_ABOVE_HORIZON: u16 = 5;
const TWILIGHT: f64 = 0.12;
const TWILIGHT_SUN: Color = Color::Rgb {
    r: 255,
    g: 140,
    b: 0,
};

#[derive(Clone, Copy, Default)]
pub struct Sightings {
    pub shooting_star: bool,
    pub postman: bool,
    pub snowman: bool,
}

pub struct AnimationManager {
    raindrop_system: RaindropSystem,
    snow_system: SnowSystem,
    fog_system: FogSystem,
    thunderstorm_system: ThunderstormSystem,
    cloud_system: CloudSystem,
    cloud_bank: CloudBank,
    bird_system: BirdSystem,
    airplane_system: AirplaneSystem,
    star_system: StarSystem,
    moon_system: MoonSystem,
    chimney_smoke: ChimneySmoke,
    firefly_system: FireflySystem,
    fireworks: Fireworks,
    falling_leaves: FallingLeaves,
    sunny_animation: SunnyAnimation,
    cat: Cat,
    dog: Dog,
    duck: Duck,
    owl: Owl,
    snowman: Snowman,
    kid: Kid,
    frog: Frog,
    snail: Snail,
    squirrel: Squirrel,
    postman: Postman,
    halloween: Halloween,
    spring: Spring,
    new_year_banner: NewYearBanner,
    rainbow: Rainbow,
    snow_cover: SnowCover,
    lore: Lore,
    gust: Gust,
    creepy: Creepy,
    finds: Finds,
    star_gazing: StarGazing,
    season: Season,
    animation_controller: AnimationController,
    last_frame_time: Instant,
    show_leaves: bool,
}

impl AnimationManager {
    pub fn new(term_width: u16, term_height: u16, show_leaves: bool) -> Self {
        let mut raindrop_system =
            RaindropSystem::new(term_width, term_height, RainIntensity::Light);
        raindrop_system.set_spawn_y(CloudBank::bottom() as f32);
        let mut snow_system = SnowSystem::new(term_width, term_height, SnowIntensity::Light);
        snow_system.set_spawn_y(CloudBank::bottom() as f32);

        Self {
            raindrop_system,
            snow_system,
            fog_system: FogSystem::new(term_width, term_height, FogIntensity::Light),
            thunderstorm_system: ThunderstormSystem::new(term_width, term_height),
            cloud_system: CloudSystem::new(term_width, term_height),
            cloud_bank: CloudBank::new(),
            bird_system: BirdSystem::new(term_width, term_height),
            airplane_system: AirplaneSystem::new(term_width, term_height),
            star_system: StarSystem::new(term_width, term_height),
            moon_system: MoonSystem::new(term_width, term_height),
            chimney_smoke: ChimneySmoke::new(),
            firefly_system: FireflySystem::new(term_width, term_height),
            fireworks: Fireworks::new(),
            falling_leaves: FallingLeaves::new(term_width, term_height),
            sunny_animation: SunnyAnimation::new(),
            cat: Cat::new(),
            dog: Dog::new(),
            duck: Duck::new(),
            owl: Owl::new(),
            snowman: Snowman,
            kid: Kid::new(),
            frog: Frog::new(),
            snail: Snail::new(),
            squirrel: Squirrel::new(),
            postman: Postman::new(),
            halloween: Halloween::new(),
            spring: Spring::new(),
            new_year_banner: NewYearBanner::new(),
            rainbow: Rainbow,
            snow_cover: SnowCover::new(),
            lore: Lore::new(),
            gust: Gust::new(),
            creepy: Creepy::new(),
            finds: Finds::new(),
            star_gazing: StarGazing::new(),
            season: Season::default(),
            animation_controller: AnimationController::new(),
            last_frame_time: Instant::now(),
            show_leaves,
        }
    }

    pub fn update_rain_intensity(&mut self, intensity: RainIntensity) {
        self.raindrop_system.set_intensity(intensity);
    }

    pub fn update_snow_intensity(&mut self, intensity: SnowIntensity) {
        self.snow_system.set_intensity(intensity);
    }

    pub fn update_wind(&mut self, speed_kmh: f32, direction_deg: f32) {
        self.raindrop_system.set_wind(speed_kmh, direction_deg);
        self.snow_system.set_wind(speed_kmh, direction_deg);
        self.chimney_smoke.set_wind(speed_kmh, direction_deg);
        self.cloud_bank.set_wind(speed_kmh, direction_deg);
        self.lore.set_wind(direction_deg);
        self.gust.set_wind(speed_kmh, direction_deg);
    }

    pub fn set_season(&mut self, season: Season) {
        self.season = season;
    }

    pub fn set_unlocks(&mut self, unlocks: Unlocks) {
        self.lore.set_unlocks(unlocks);
        self.postman.set_bell(unlocks.choices.postman_bell);
        self.kid.set_avoids_puddles(unlocks.choices.avoids_puddles);
    }

    pub fn set_finds_remaining(&mut self, has_objects_left: bool, has_bottles_left: bool) {
        self.finds.set_remaining(has_objects_left, has_bottles_left);
    }

    pub fn visible_finds(&self) -> (bool, bool) {
        (self.finds.has_object(), self.finds.has_bottle())
    }

    pub fn take_object(&mut self) -> bool {
        self.finds.take_object(&mut rand::rng())
    }

    pub fn take_bottle(&mut self) -> bool {
        self.finds.take_bottle(&mut rand::rng())
    }

    pub fn set_mail_hour(&mut self, hour: u32) {
        self.postman.set_round_hour(hour);
    }

    pub fn set_mail_waiting(&mut self, is_waiting: bool) {
        self.postman.set_mail_waiting(is_waiting);
    }

    pub fn postman_visit(&mut self) {
        self.postman.visit();
    }

    pub fn postman_leave(&mut self) {
        self.postman.leave();
    }

    pub fn is_postman_arriving(&self, term_width: u16, term_height: u16) -> bool {
        self.postman
            .is_arriving(&GardenLayout::new(term_width, term_height))
    }

    pub fn postman_head(&self, term_width: u16, term_height: u16) -> Option<(u16, u16)> {
        self.postman
            .head(&GardenLayout::new(term_width, term_height))
    }

    pub fn sightings(&self, conditions: &WeatherConditions) -> Sightings {
        Sightings {
            shooting_star: !conditions.is_day && self.star_system.has_shooting_star(),
            postman: self.postman.is_delivering(),
            snowman: conditions.is_snowing && self.snow_cover.is_thick(),
        }
    }

    pub fn update_fog_intensity(&mut self, intensity: FogIntensity) {
        self.fog_system.set_intensity(intensity);
    }

    pub fn render_background(
        &mut self,
        renderer: &mut TerminalRenderer,
        conditions: &WeatherConditions,
        state: &AppState,
        term_width: u16,
        term_height: u16,
        mut rng: &mut impl rand::Rng,
    ) -> io::Result<()> {
        // Calculate horizon_y early so it's available for all systems
        let ground_height = WorldScene::GROUND_HEIGHT;
        let horizon_y = term_height.saturating_sub(ground_height);

        if !conditions.is_day {
            self.star_system.set_sky(if !conditions.is_clear_sky() {
                ShootingStarSky::Covered
            } else if state.holiday.meteor_shower.is_some() {
                ShootingStarSky::MeteorShower
            } else {
                ShootingStarSky::Clear
            });
            self.star_system.update(term_width, term_height, &mut rng);
            self.star_system.render(renderer)?;
            self.moon_system.update(term_width, term_height);
            self.moon_system.render(renderer)?;

            if state.should_show_fireflies() {
                self.firefly_system
                    .update(term_width, term_height, horizon_y, &mut rng);
                self.firefly_system.render(renderer)?;
            }

            if state.holiday.has_fireworks() && !conditions.is_thunderstorm {
                self.fireworks.update(term_width, horizon_y, &mut rng);
                self.fireworks.render(renderer)?;
            }
        }

        if !conditions.is_raining
            && !conditions.is_thunderstorm
            && !conditions.is_snowing
            && conditions.is_day
        {
            self.bird_system.update(term_width, term_height, &mut rng);
            self.bird_system.render(renderer)?;
        }

        if state.should_show_sun()
            && !conditions.is_raining
            && !conditions.is_thunderstorm
            && !conditions.is_snowing
        {
            self.render_sun(renderer, state, term_width, horizon_y)?;
        }

        if state.shows_rainbow() {
            self.rainbow.render(renderer, term_width, horizon_y)?;
        }

        if conditions.is_cloudy
            || (!conditions.is_raining && !conditions.is_thunderstorm && !conditions.is_snowing)
        {
            let (is_clear, cloud_color) = if let Some(weather) = &state.current_weather {
                match weather.condition {
                    crate::weather::WeatherCondition::Clear => (true, Color::White),
                    crate::weather::WeatherCondition::PartlyCloudy => (false, Color::Grey),
                    _ => (false, Color::DarkGrey),
                }
            } else {
                (false, Color::DarkGrey)
            };

            if conditions.is_cloudy || is_clear {
                self.cloud_system.set_cloud_color(is_clear);
                self.cloud_system
                    .update(term_width, term_height, is_clear, cloud_color, &mut rng);
                self.cloud_system.render(renderer)?;
            }
        }

        if !conditions.is_raining
            && !conditions.is_thunderstorm
            && !conditions.is_snowing
            && !conditions.is_foggy
        {
            self.airplane_system
                .update(term_width, term_height, &mut rng);
            self.airplane_system.render(renderer)?;
        }

        if conditions.is_raining || conditions.is_thunderstorm || conditions.is_snowing {
            self.cloud_bank.update();
            self.cloud_bank.render(renderer, term_width, conditions)?;
        }

        Ok(())
    }

    fn render_sun(
        &self,
        renderer: &mut TerminalRenderer,
        state: &AppState,
        term_width: u16,
        horizon_y: u16,
    ) -> io::Result<()> {
        let frame: Vec<&str> = self
            .animation_controller
            .current(&self.sunny_animation)
            .iter()
            .map(String::as_str)
            .collect();
        let sun_width = frame.iter().map(|line| line.len()).max().unwrap_or(0) as u16;
        let progress = day_progress(
            state.location.latitude,
            state.location.longitude,
            chrono::Utc::now(),
        );

        let Some(progress) = progress else {
            let animation_y = if horizon_y > 13 { 3 } else { 2 };
            return self.animation_controller.render_frame(
                renderer,
                &self.sunny_animation,
                animation_y,
            );
        };

        let top = CloudBank::TOP;
        let lowest = horizon_y.saturating_sub(SUN_LOWEST_ABOVE_HORIZON).max(top);
        let travel = term_width.saturating_sub(sun_width + 2 * SUN_MARGIN) as f64;
        let x = SUN_MARGIN + (progress * travel) as u16;
        let height = (std::f64::consts::PI * progress).sin();
        let y = top + ((1.0 - height) * (lowest - top) as f64) as u16;
        let color = if !(TWILIGHT..=1.0 - TWILIGHT).contains(&progress) {
            TWILIGHT_SUN
        } else {
            Color::Yellow
        };
        garden::draw(renderer, x, y, &frame, color)
    }

    pub fn render_chimney_smoke(
        &mut self,
        renderer: &mut TerminalRenderer,
        conditions: &WeatherConditions,
        term_width: u16,
        term_height: u16,
    ) -> io::Result<()> {
        if conditions.is_thunderstorm {
            return Ok(());
        }
        let ground_height = WorldScene::GROUND_HEIGHT;
        let horizon_y = term_height.saturating_sub(ground_height);
        let house = House::for_terminal(term_width, term_height);
        let house_x = house.x_on(term_width, horizon_y);
        let house_y = horizon_y.saturating_sub(house.height());
        let (chimney_offset_x, chimney_offset_y) = house.chimney_offset();
        let chimney_x = house_x + chimney_offset_x;
        let chimney_y = house_y + chimney_offset_y;

        let ceiling = if conditions.is_raining || conditions.is_snowing {
            CloudBank::bottom()
        } else {
            0
        };
        self.chimney_smoke.update(chimney_x, chimney_y, ceiling);
        self.chimney_smoke.render(renderer)?;

        Ok(())
    }

    pub fn render_garden(
        &mut self,
        renderer: &mut TerminalRenderer,
        conditions: &WeatherConditions,
        holiday: &Holiday,
        term_width: u16,
        term_height: u16,
        mut rng: &mut impl rand::Rng,
    ) -> io::Result<()> {
        let layout = GardenLayout::new(term_width, term_height);

        self.snow_cover.update(conditions.is_snowing);
        self.snow_cover.render(renderer, &layout)?;

        self.spring.update(&layout, holiday, conditions, &mut rng);
        self.spring.render_ground(renderer, &layout, holiday)?;
        let river = River::new(layout.width, layout.horizon_y);
        self.spring.render_lilies(renderer, &river, holiday)?;

        if holiday.is_halloween {
            self.halloween.update(&layout, conditions, &mut rng);
            self.halloween
                .render_pumpkins(renderer, &layout, conditions.is_day)?;
        }

        let unlocks = self.lore.unlocks();
        self.lore.update(&layout, conditions, &mut rng);
        self.lore.render_balloon(renderer)?;
        self.lore.render_scenery(renderer, &layout, conditions)?;

        if holiday.is_epiphany {
            epiphany::render_crown(renderer, &layout)?;
        }

        self.creepy.update(conditions, &unlocks, &mut rng);
        self.creepy
            .render(renderer, &layout, conditions, &unlocks)?;
        self.finds.update(
            &layout,
            conditions,
            &unlocks,
            &river,
            holiday.is_winter,
            &mut rng,
        );
        self.finds.render(renderer, &layout, &river)?;

        let is_staring = self.creepy.is_staring();
        self.cat.set_staring(is_staring);
        self.dog.set_staring(is_staring);
        self.owl.set_staring(is_staring);

        self.owl.update(conditions, &mut rng);
        self.owl.render(renderer, &layout, conditions, &unlocks)?;
        self.star_gazing
            .update(&layout, conditions, self.season, &mut rng);
        self.star_gazing.render(renderer, &layout)?;

        self.cat.update(&mut rng);
        self.cat.render(renderer, &layout, conditions, &unlocks)?;

        self.dog.update(&layout, conditions, &mut rng);
        self.dog.render(renderer, &layout, conditions)?;

        self.duck.update(conditions, &mut rng);
        self.duck.render(renderer, &layout, conditions, &unlocks)?;

        self.frog.update(conditions, &mut rng);
        self.frog.render(renderer, &layout, conditions, &unlocks)?;

        self.snail.update(&layout, conditions);
        self.snail.render(renderer, &layout)?;

        self.snowman
            .render(renderer, &layout, conditions, self.snow_cover.is_thick())?;

        self.squirrel.update(&layout, conditions, &mut rng);
        self.squirrel.render(renderer)?;

        self.postman.set_april_fools(holiday.is_april_fools);
        self.postman.update(&layout);
        self.postman.render(renderer, &layout)?;

        self.kid.update(&layout, conditions, &mut rng);
        self.kid.render(renderer, &layout, conditions)?;

        if holiday.is_halloween {
            self.halloween.render_spirits(renderer, &layout)?;
        }

        self.spring.render_butterflies(renderer)?;

        if let Some(year) = holiday.new_year {
            self.new_year_banner.update();
            self.new_year_banner.render(renderer, term_width, year)?;
        }

        Ok(())
    }

    pub fn render_foreground(
        &mut self,
        renderer: &mut TerminalRenderer,
        conditions: &WeatherConditions,
        term_width: u16,
        term_height: u16,
        mut rng: &mut impl rand::Rng,
    ) -> io::Result<()> {
        if conditions.is_thunderstorm {
            self.raindrop_system
                .update(term_width, term_height, &mut rng);
            self.raindrop_system.render(renderer)?;

            self.thunderstorm_system
                .update(term_width, term_height, &mut rng);
            self.thunderstorm_system.render(renderer)?;

            if self.thunderstorm_system.is_flashing() {
                renderer.flash_screen()?;
            }
        } else if conditions.is_raining {
            self.raindrop_system
                .update(term_width, term_height, &mut rng);
            self.raindrop_system.render(renderer)?;
        } else if conditions.is_snowing {
            self.snow_system.update(term_width, term_height, &mut rng);
            self.snow_system.render(renderer)?;
        }

        if conditions.is_foggy {
            self.fog_system.update(term_width, term_height, &mut rng);
            self.fog_system.render(renderer)?;
        }

        if (self.show_leaves || self.season == Season::Autumn)
            && !conditions.is_raining
            && !conditions.is_thunderstorm
            && !conditions.is_snowing
        {
            self.falling_leaves
                .update(term_width, term_height, &mut rng);
            self.falling_leaves.render(renderer)?;
        }

        let horizon_y = term_height.saturating_sub(WorldScene::GROUND_HEIGHT);
        self.gust
            .update(term_width, horizon_y, self.season, &mut rng);
        self.gust.render(renderer)?;

        Ok(())
    }

    pub fn update_sunny_animation(&mut self, conditions: &WeatherConditions) {
        if !conditions.is_raining
            && !conditions.is_thunderstorm
            && !conditions.is_snowing
            && self.last_frame_time.elapsed() >= FRAME_DELAY
        {
            self.animation_controller.next_frame(&self.sunny_animation);
            self.last_frame_time = Instant::now();
        }
    }
}
