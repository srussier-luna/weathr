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
            Self::Recipe(_) => 6,
            Self::Postcard(_) => 7,
            Self::Creepy(_) => 8,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Thing {
    Bench,
    CatBow,
    Footprints,
    Well,
    Bone,
    Key,
    Treehouse,
    OwlFriend,
    BlueLantern,
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
