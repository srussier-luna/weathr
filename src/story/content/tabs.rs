use super::{Series, series};
use crate::story::Entry;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tab {
    Letters,
    Postcards,
    Archives,
    Recipes,
    Gazettes,
}

const TABS: [Tab; 5] = [
    Tab::Letters,
    Tab::Postcards,
    Tab::Archives,
    Tab::Recipes,
    Tab::Gazettes,
];

impl Tab {
    pub fn label(self) -> &'static str {
        match self {
            Tab::Letters => "Lettres",
            Tab::Postcards => "Cartes postales",
            Tab::Archives => "Archives",
            Tab::Recipes => "Recettes",
            Tab::Gazettes => "Gazettes",
        }
    }

    fn holds(self, entry: Entry) -> bool {
        let tab = match series(entry) {
            Some(Series::Letter) => Tab::Letters,
            Some(Series::Postcard) => Tab::Postcards,
            Some(Series::Archive) => Tab::Archives,
            Some(Series::Recipe) => Tab::Recipes,
            None => Tab::Gazettes,
        };
        tab == self
    }
}

pub fn visible_tabs(read: &[(usize, Entry)]) -> Vec<Tab> {
    TABS.into_iter()
        .filter(|tab| read.iter().any(|&(_, entry)| tab.holds(entry)))
        .collect()
}

pub fn entries_in(read: &[(usize, Entry)], tab: Tab) -> Vec<usize> {
    read.iter()
        .enumerate()
        .filter(|&(_, &(_, entry))| tab.holds(entry))
        .map(|(index, _)| index)
        .collect()
}

pub fn opening_tab(read: &[(usize, Entry)]) -> Tab {
    visible_tabs(read).first().copied().unwrap_or(Tab::Letters)
}

pub fn switch_tab(read: &[(usize, Entry)], tab: Tab, step: isize) -> Tab {
    let visible = visible_tabs(read);
    match visible.iter().position(|&shown| shown == tab) {
        Some(position) => visible[position.saturating_add_signed(step).min(visible.len() - 1)],
        None => opening_tab(read),
    }
}

pub fn has_neighbours(read: &[(usize, Entry)], tab: Tab) -> (bool, bool) {
    let visible = visible_tabs(read);
    match visible.iter().position(|&shown| shown == tab) {
        Some(position) => (position > 0, position + 1 < visible.len()),
        None => (false, false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read() -> Vec<(usize, Entry)> {
        vec![
            (1, Entry::Main(0)),
            (2, Entry::Recipe(0)),
            (2, Entry::Sam(0)),
            (3, Entry::Gazette(0)),
            (4, Entry::DietLetter),
            (5, Entry::Recipe(1)),
        ]
    }

    #[test]
    fn test_entries_group_per_tab() {
        let read = read();

        assert_eq!(entries_in(&read, Tab::Letters), vec![0, 2, 4]);
        assert_eq!(entries_in(&read, Tab::Recipes), vec![1, 5]);
        assert_eq!(entries_in(&read, Tab::Gazettes), vec![3]);
        assert!(entries_in(&read, Tab::Postcards).is_empty());
    }

    #[test]
    fn test_tabs_without_a_read_entry_are_hidden() {
        assert_eq!(
            visible_tabs(&read()),
            vec![Tab::Letters, Tab::Recipes, Tab::Gazettes]
        );
        assert!(visible_tabs(&[]).is_empty());
    }

    #[test]
    fn test_courrier_opens_on_letters_or_the_first_shown_tab() {
        assert_eq!(opening_tab(&read()), Tab::Letters);
        assert_eq!(opening_tab(&[(3, Entry::Gazette(0))]), Tab::Gazettes);
        assert_eq!(opening_tab(&[]), Tab::Letters);
    }

    #[test]
    fn test_tab_switch_skips_hidden_tabs_and_clamps() {
        let read = read();

        assert_eq!(switch_tab(&read, Tab::Letters, 1), Tab::Recipes);
        assert_eq!(switch_tab(&read, Tab::Recipes, 1), Tab::Gazettes);
        assert_eq!(switch_tab(&read, Tab::Gazettes, 1), Tab::Gazettes);
        assert_eq!(switch_tab(&read, Tab::Recipes, -1), Tab::Letters);
        assert_eq!(switch_tab(&read, Tab::Letters, -1), Tab::Letters);
        assert_eq!(switch_tab(&[], Tab::Letters, 1), Tab::Letters);
    }

    #[test]
    fn test_selection_in_a_tab_maps_to_its_global_index() {
        let read = read();

        assert_eq!(entries_in(&read, Tab::Recipes).get(1), Some(&5));
        assert_eq!(entries_in(&read, Tab::Letters).get(1), Some(&2));
        assert_eq!(entries_in(&read, Tab::Letters).get(3), None);
    }

    #[test]
    fn test_neighbours_follow_the_shown_tabs() {
        let read = read();

        assert_eq!(has_neighbours(&read, Tab::Letters), (false, true));
        assert_eq!(has_neighbours(&read, Tab::Recipes), (true, true));
        assert_eq!(has_neighbours(&read, Tab::Gazettes), (true, false));
        assert_eq!(has_neighbours(&read, Tab::Postcards), (false, false));
        assert_eq!(has_neighbours(&[], Tab::Letters), (false, false));
    }
}
