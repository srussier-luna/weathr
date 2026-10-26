use crate::holiday::{Holiday, MeteorShower};
use crate::weather::WeatherCondition;
use chrono::{Datelike, Local};
use std::time::{Duration, Instant};

pub const SCENE_DURATION: Duration = Duration::from_secs(1);
const CONDITIONS: [WeatherCondition; 14] = [
    WeatherCondition::Clear,
    WeatherCondition::PartlyCloudy,
    WeatherCondition::Cloudy,
    WeatherCondition::Overcast,
    WeatherCondition::Fog,
    WeatherCondition::Drizzle,
    WeatherCondition::Rain,
    WeatherCondition::FreezingRain,
    WeatherCondition::RainShowers,
    WeatherCondition::Snow,
    WeatherCondition::SnowGrains,
    WeatherCondition::SnowShowers,
    WeatherCondition::Thunderstorm,
    WeatherCondition::ThunderstormHail,
];

#[derive(Clone, Copy)]
pub struct DemoScene {
    pub condition: WeatherCondition,
    pub is_day: bool,
    pub holiday: Holiday,
    pub rainbow: bool,
}

pub struct Demo {
    scenes: Vec<DemoScene>,
    index: usize,
    shown_since: Instant,
}

impl Demo {
    pub fn new() -> Self {
        let regular = |is_day| {
            CONDITIONS.into_iter().map(move |condition| DemoScene {
                condition,
                is_day,
                holiday: Holiday::default(),
                rainbow: false,
            })
        };
        let scene = |condition, is_day, holiday| DemoScene {
            condition,
            is_day,
            holiday,
            rainbow: false,
        };
        let holidays = [
            scene(WeatherCondition::Snow, true, Holiday::christmas()),
            scene(WeatherCondition::Clear, false, Holiday::national_day()),
            scene(WeatherCondition::Clear, false, Holiday::halloween()),
            scene(
                WeatherCondition::Clear,
                false,
                Holiday::new_year(Local::now().year() + 1),
            ),
            scene(WeatherCondition::Clear, true, Holiday::easter()),
            scene(WeatherCondition::Clear, false, Holiday::perseids()),
            scene(WeatherCondition::Clear, false, Holiday::geminids()),
            DemoScene {
                rainbow: true,
                ..scene(WeatherCondition::PartlyCloudy, true, Holiday::default())
            },
        ];

        Self {
            scenes: regular(true)
                .chain(regular(false))
                .chain(holidays)
                .collect(),
            index: 0,
            shown_since: Instant::now(),
        }
    }

    pub fn current(&self) -> &DemoScene {
        &self.scenes[self.index]
    }

    pub fn advance_if_due(&mut self) -> bool {
        if self.shown_since.elapsed() < SCENE_DURATION {
            return false;
        }

        self.index = (self.index + 1) % self.scenes.len();
        self.shown_since = Instant::now();
        true
    }

    pub fn label(&self) -> String {
        let scene = self.current();
        let suffix = if scene.holiday.is_christmas {
            " Christmas"
        } else if scene.holiday.is_national_day {
            " July 14th"
        } else if scene.holiday.is_halloween {
            " Halloween"
        } else if scene.holiday.new_year.is_some() {
            " New Year"
        } else if scene.holiday.is_easter {
            " Spring"
        } else if let Some(meteor_shower) = scene.holiday.meteor_shower {
            match meteor_shower {
                MeteorShower::Perseids => " Perséides",
                MeteorShower::Geminids => " Géminides",
            }
        } else if scene.rainbow {
            " Rainbow"
        } else if !scene.is_day {
            " Night"
        } else {
            ""
        };

        format!("DEMO {}/{}{}", self.index + 1, self.scenes.len(), suffix)
    }
}
