mod archives;
mod correspondents;
mod creepy;
mod gazettes;
mod postcards;
mod recipes;
mod village;

use super::clock::StoryClock;
use super::event::Beat;

/// Schedules one kind of story event on the Timeline.
pub trait StorySource {
    /// Beats of this source; open-ended rhythms stop at Jour de jeu `today`.
    fn beats(&self, clock: &StoryClock, today: usize) -> Vec<Beat>;
}

/// Every source of the story, `village_count` being the number of village Lettres.
pub fn registered(village_count: usize) -> Vec<Box<dyn StorySource>> {
    vec![
        Box::new(village::Village::new(village_count)),
        Box::new(correspondents::SAM),
        Box::new(correspondents::NEIGHBOURS),
        Box::new(correspondents::MARCEL),
        Box::new(archives::Archives),
        Box::new(creepy::Creepy),
        Box::new(gazettes::Gazettes),
        Box::new(recipes::Recipes),
        Box::new(postcards::Postcards),
    ]
}
