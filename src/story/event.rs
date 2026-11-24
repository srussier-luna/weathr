#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Entry {
    Main(usize),
    Sam(usize),
    Neighbours(usize),
    Marcel(usize),
    Archive(usize),
    Gazette(usize),
    Recipe(usize),
    Postcard(usize),
    Creepy(usize),
    DietLetter,
    DietRecipe(usize),
}

impl Entry {
    fn order(&self) -> u8 {
        match self {
            Self::Main(_) => 0,
            Self::Sam(_) => 1,
            Self::Neighbours(_) => 2,
            Self::Marcel(_) => 3,
            Self::Archive(_) => 4,
            Self::Gazette(_) => 5,
            Self::Recipe(_) | Self::DietRecipe(_) => 6,
            Self::Postcard(_) => 7,
            Self::Creepy(_) => 8,
            Self::DietLetter => 9,
        }
    }

    pub fn recipe_index(&self) -> Option<usize> {
        match self {
            Self::Recipe(index) | Self::DietRecipe(index) => Some(*index),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Thing {
    Cat,
    Bench,
    CatBow,
    Footprints,
    Dog,
    Owl,
    Well,
    Bone,
    Key,
    Duck,
    Snail,
    Treehouse,
    OwlFriend,
    BlueLantern,
    Squirrel,
    Frog,
    Garland,
    Kittens,
    XMark,
    Balloon,
    Ducklings,
    FoundObjects,
    WeatherVane,
    Bunting,
    Chest,
    WetFootprints,
    Silhouette,
    WhisperingWell,
    Staring,
    Bottles,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StoryEvent {
    Mail(Entry),
    Arrival(Thing),
}

impl StoryEvent {
    pub fn order(&self) -> u8 {
        match self {
            Self::Mail(entry) => entry.order(),
            Self::Arrival(_) => u8::MAX,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Beat {
    pub day: usize,
    pub event: StoryEvent,
}

impl Beat {
    pub fn mail(day: usize, entry: Entry) -> Self {
        Self {
            day,
            event: StoryEvent::Mail(entry),
        }
    }

    pub fn arrival(day: usize, thing: Thing) -> Self {
        Self {
            day,
            event: StoryEvent::Arrival(thing),
        }
    }
}
