/// A Lettre, identified by its kind and its index within that kind.
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

/// Something that happens in the story.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StoryEvent {
    Mail(Entry),
}

impl StoryEvent {
    /// Rank among events of the same Jour de jeu.
    pub fn order(&self) -> u8 {
        match self {
            Self::Mail(entry) => entry.order(),
        }
    }
}

/// A story event scheduled on a Jour de jeu.
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
}
