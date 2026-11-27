use crate::animation::lore::Unlocks;
use crate::animation::trophies::Trophy;
use crate::badges::{self, Award};
use crate::holiday::HolidayBox;
use crate::overlay::{self, Frame, Line, View};
use crate::progress::Progress;
use crate::release_notes::Section;
use crate::settings::{AutoLocation, CitySearch, Row, SearchStatus, Settings};
use crate::story::Entry;
use crate::story::content::chronicles::{BOTTLE_MESSAGES, CHEST_SOUVENIRS, DRAWINGS, OBJECTS};
use crate::story::content::letters::answered_choices;
use crate::story::content::neighbours_note::{self, NeighboursNote};
use crate::story::content::recipes::{Diet, RECIPES, diet_labels};
use crate::story::content::tabs::{self, Tab};
use crate::story::content::{
    self, MONTHS, ReadContext, diet_checklist, postcard_lines, postcard_title, text,
};
use crate::story::punishment::Punishment;
use crate::weather::units::{precipitation_symbol, temperature_symbol, wind_speed_symbol};
use chrono::{Datelike, NaiveDate};
use std::collections::BTreeSet;
use std::path::Path;

const INDEX_FOOTER: &str =
    "[←] [→] onglets  [↑] [↓] choisir  [Entrée] relire  [l] ou [Echap] fermer";
const ALBUM_FOOTER: &str = "[<] [>] pages  [a] ou [Echap] fermer";
const ANSWERS_FOOTER: &str =
    "[↑] [↓] choisir  [1] [2] changer  [<] [>] pages  [a] ou [Echap] fermer";
const BOTTLE_FOOTER: &str = "[Echap] fermer";
const HOLIDAY_BOX_FOOTER: &str = "[Echap] fermer";
const BADGE_CARD_FOOTER: &str = "[Entrée] ou [Echap] fermer";
const HELP_FOOTER: &str = "[h] ou [Echap] fermer";
const SETTINGS_FOOTER: &str = "[↑] [↓] choisir  [<] [>] changer  [s] ou [Echap] fermer";
const SETTINGS_LABEL_WIDTH: usize = 18;
const CITY_SEARCH_FOOTER: &str = "[Entrée] chercher  [Echap] retour";
const CITY_RESULTS_FOOTER: &str = "[↑] [↓] choisir  [Entrée] valider  [Echap] retour";
const DIET_FOOTER: &str = "[1]…[8] cocher  [Entrée] valider  [Echap] retour";
const LISTED_DIETS: usize = 2;
const CLOSE_FOOTER: &str = "[Entrée] ou [Echap] fermer";
const POSTMAN_NOTE_FOOTER: &str = "[c] suivante  [Echap] fermer";
const SCROLL_HINT: &str = "[↑] [↓] défiler";
const HELP_KEY_WIDTH: usize = 10;
const NOTES_BULLET: &str = "- ";
const NOTES_INDENT: &str = "  ";
pub const NOTES_FRAME: Frame = Frame::Plain;
pub const INDEX_FRAME: Frame = Frame::Plain;
pub const ANSWERS_PAGE: usize = 2;
pub const TROPHIES_PAGE: usize = ANSWERS_PAGE + 1;
pub const POSTMAN_SIGNATURE: &str = "— Le Facteur";
const FULL_BOX_NOTE: &str =
    "Pendant ton absence, tout le village en a profité pour prendre des vacances\u{202F}!";

pub fn index_view(
    read_entries: &[(usize, Entry)],
    tab: Tab,
    selected: usize,
    rows: usize,
    context: &ReadContext,
) -> View {
    let shown = tabs::entries_in(read_entries, tab);
    let mut lines = Vec::new();
    if !shown.is_empty() {
        lines.push(Line::Preformatted(tab_bar(read_entries, tab)));
        lines.push(Line::Blank);
    }
    let rows = rows.saturating_sub(lines.len()).max(1);
    let first = selected
        .saturating_sub(rows / 2)
        .min(shown.len().saturating_sub(rows));
    lines.extend(
        shown
            .iter()
            .enumerate()
            .skip(first)
            .take(rows)
            .map(|(position, &index)| {
                let (day, entry) = read_entries[index];
                let marker = if position == selected { '>' } else { ' ' };
                Line::Preformatted(format!(
                    "{marker} {:>3}. {}  {}",
                    position + 1,
                    context.clock.date_of(day).format("%d/%m"),
                    content::label(day, entry, context)
                ))
            }),
    );
    if lines.is_empty() {
        lines.push(text("Aucune lettre lue pour l'instant."));
    }
    View {
        title: format!("Lettres reçues ({})", read_entries.len()),
        lines,
        footer: INDEX_FOOTER.to_string(),
        frame: INDEX_FRAME,
    }
}

fn tab_bar(read_entries: &[(usize, Entry)], current: Tab) -> String {
    tabs::visible_tabs(read_entries)
        .into_iter()
        .map(|tab| {
            if tab == current {
                format!("[{}]", tab.label())
            } else {
                tab.label().to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("  ")
}

fn day_and_month(date: NaiveDate) -> String {
    let day = match date.day() {
        1 => "1er".to_string(),
        day => day.to_string(),
    };
    format!("{day} {}", MONTHS[date.month0() as usize])
}

fn full_box_note(back_to_school: NaiveDate) -> Vec<Line> {
    vec![
        text(FULL_BOX_NOTE),
        text(format!(
            "La rentrée, c'est le {}.",
            day_and_month(back_to_school)
        )),
        Line::Blank,
        text(POSTMAN_SIGNATURE),
    ]
}

pub fn postman_note_view(back_to_school: NaiveDate) -> View {
    View {
        title: String::new(),
        lines: full_box_note(back_to_school),
        footer: POSTMAN_NOTE_FOOTER.to_string(),
        frame: Frame::Notice,
    }
}

fn backup_box(line: String) -> View {
    View {
        title: "Sauvegarde".to_string(),
        lines: vec![text(line)],
        footer: CLOSE_FOOTER.to_string(),
        frame: Frame::Floppy,
    }
}

pub fn backup_view(backup: &Path) -> View {
    backup_box(format!(
        "Ta progression n'a pas pu être lue. Une copie est gardée dans {}. Une nouvelle partie commence.",
        backup.display()
    ))
}

pub fn restored_view() -> View {
    backup_box(
        "Ta progression n'a pas pu être lue. Ta partie reprend à la dernière sauvegarde."
            .to_string(),
    )
}

pub fn neighbours_note_view(note: NeighboursNote) -> View {
    let mut lines = Vec::new();
    if note == NeighboursNote::Announcement(Punishment::Cheater)
        && let Some(cheater) = badges::find(badges::CHEATER)
    {
        lines.extend(medal(cheater.icon).into_iter().map(Line::Centered));
        lines.push(Line::Blank);
    }
    lines.extend([
        text(note.text()),
        Line::Blank,
        text(neighbours_note::SIGNATURE),
    ]);
    View {
        title: String::new(),
        lines,
        footer: CLOSE_FOOTER.to_string(),
        frame: Frame::Roofs,
    }
}

pub fn full_box_index_view(
    read_entries: &[(usize, Entry)],
    tab: Tab,
    selected: usize,
    rows: usize,
    width: usize,
    context: &ReadContext,
    back_to_school: NaiveDate,
) -> View {
    let mut note = full_box_note(back_to_school);
    note.push(Line::Rule);
    let note_rows = overlay::lay_out(&note, width).len();
    let mut view = index_view(
        read_entries,
        tab,
        selected,
        rows.saturating_sub(note_rows),
        context,
    );
    note.append(&mut view.lines);
    view.lines = note;
    view
}

pub fn bottle_view(index: usize) -> View {
    View {
        title: "Message dans une bouteille".to_string(),
        lines: vec![text(BOTTLE_MESSAGES[index])],
        footer: BOTTLE_FOOTER.to_string(),
        frame: Frame::Waves,
    }
}

pub fn holiday_box_view(holiday_box: &HolidayBox) -> View {
    View {
        title: holiday_box.title.to_string(),
        lines: holiday_box.lines.iter().copied().map(text).collect(),
        footer: HOLIDAY_BOX_FOOTER.to_string(),
        frame: Frame::Plain,
    }
}

fn medal([top, middle, bottom]: [&str; 3]) -> [String; 5] {
    [
        " .-\"\"\"-. ".to_string(),
        format!("/ {top} \\"),
        format!("| {middle} |"),
        format!("\\ {bottom} /"),
        " '-...-' ".to_string(),
    ]
}

pub fn badge_card_view(award: Award) -> View {
    let title = if award.is_new_tier() {
        "Nouveau palier\u{202F}!"
    } else {
        "Nouveau badge\u{202F}!"
    };
    let mut lines = vec![Line::Blank];
    lines.extend(medal(award.badge.icon).into_iter().map(Line::Centered));
    lines.extend([
        Line::Blank,
        Line::Centered(title.to_string()),
        Line::Centered(award.name()),
    ]);
    lines.extend(award.hint().map(Line::Centered));
    View {
        title: String::new(),
        lines,
        footer: BADGE_CARD_FOOTER.to_string(),
        frame: Frame::Stars,
    }
}

pub fn help_view(unlocks: &Unlocks, has_gazette: bool, is_december: bool) -> View {
    let pick_up = match (unlocks.found_objects, unlocks.bottles) {
        (true, true) => Some("ramasser un objet ou une bouteille"),
        (true, false) => Some("ramasser un objet"),
        (false, true) => Some("ramasser une bouteille"),
        (false, false) => None,
    };
    let commands = [
        Some(("[c]", "lire le courrier")),
        Some(("[<] [>]", "parcourir les lettres ou les pages")),
        Some(("[1] [2]", "répondre à une question")),
        has_gazette.then_some(("[e]", "répondre à l'énigme de la gazette")),
        pick_up.map(|description| ("[r]", description)),
        Some(("[l]", "relire les lettres reçues")),
        Some(("[a]", "album des souvenirs")),
        Some(("[b]", "badges")),
        is_december.then_some(("[v]", "calendrier de l'Avent")),
        Some(("[s]", "réglages")),
        Some(("[h]", "cette aide")),
        Some(("[Echap]", "fermer la fenêtre")),
        Some(("[q]", "quitter")),
    ];
    let mut lines: Vec<Line> = commands
        .into_iter()
        .flatten()
        .map(|(key, description)| {
            Line::Preformatted(format!(
                "{key:<width$}{description}",
                width = HELP_KEY_WIDTH
            ))
        })
        .collect();
    lines.push(Line::Blank);
    lines.push(text("Options de lancement : weathr --help"));
    View {
        title: "Commandes".to_string(),
        lines,
        footer: HELP_FOOTER.to_string(),
        frame: Frame::Plain,
    }
}

pub fn settings_view(settings: &Settings, selected: usize, is_hour_for_tomorrow: bool) -> View {
    let mut lines: Vec<Line> = settings
        .rows()
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let (label, value) = settings_row(settings, *row);
            let line = if index == selected {
                format!("> {label:<width$}< {value} >", width = SETTINGS_LABEL_WIDTH)
            } else {
                format!("  {label:<width$}  {value}", width = SETTINGS_LABEL_WIDTH)
            };
            Line::Preformatted(line)
        })
        .collect();
    if is_hour_for_tomorrow {
        lines.push(Line::Blank);
        lines.push(text("Nouvelle heure du Facteur dès demain."));
    }
    View {
        title: "Réglages".to_string(),
        lines,
        footer: SETTINGS_FOOTER.to_string(),
        frame: Frame::Plain,
    }
}

fn settings_row(settings: &Settings, row: Row) -> (&'static str, String) {
    match row {
        Row::Temperature => (
            "Température",
            temperature_symbol(settings.units.temperature).to_string(),
        ),
        Row::WindSpeed => (
            "Vent",
            wind_speed_symbol(settings.units.wind_speed).to_string(),
        ),
        Row::Precipitation => (
            "Pluie",
            precipitation_symbol(settings.units.precipitation).to_string(),
        ),
        Row::MailHour => ("Heure du Facteur", format!("{} h", settings.mail_hour)),
        Row::Hud => (
            "Barres",
            if settings.hide_hud {
                "masquée"
            } else {
                "affichée"
            }
            .to_string(),
        ),
        Row::City => ("Ville", settings.city.clone()),
        Row::AutoLocation => (
            "Position auto",
            match settings.auto_location {
                AutoLocation::Off => "non",
                AutoLocation::Locating => "…",
                AutoLocation::On => "oui",
            }
            .to_string(),
        ),
        Row::Diet => (
            "Régime",
            settings
                .diets
                .as_ref()
                .map(diet_summary)
                .unwrap_or_default(),
        ),
    }
}

fn diet_summary(diets: &BTreeSet<Diet>) -> String {
    match diets.len() {
        0 => "aucun".to_string(),
        count if count > LISTED_DIETS => format!("{count} Régimes"),
        _ => diet_labels(diets),
    }
}

pub fn diet_view(diets: &BTreeSet<Diet>) -> View {
    View {
        title: "Régime".to_string(),
        lines: diet_checklist(diets),
        footer: DIET_FOOTER.to_string(),
        frame: Frame::Plain,
    }
}

pub fn city_search_view(search: &CitySearch) -> View {
    let mut lines = vec![Line::Preformatted(format!("> {}_", search.query))];
    let answer: Vec<Line> = match &search.status {
        SearchStatus::Typing => Vec::new(),
        SearchStatus::Searching(_) => vec![text("Recherche…")],
        SearchStatus::Found(places, selected) => places
            .iter()
            .enumerate()
            .map(|(index, place)| {
                let pointer = if index == *selected { ">" } else { " " };
                Line::Preformatted(format!("{pointer} {}", place.label))
            })
            .collect(),
        SearchStatus::NotFound => vec![text("Aucune ville trouvée")],
        SearchStatus::NoConnection => vec![text("Pas de connexion")],
    };
    if !answer.is_empty() {
        lines.push(Line::Blank);
        lines.extend(answer);
    }
    let footer = match search.status {
        SearchStatus::Found(..) => CITY_RESULTS_FOOTER,
        _ => CITY_SEARCH_FOOTER,
    };
    View {
        title: "Ville".to_string(),
        lines,
        footer: footer.to_string(),
        frame: Frame::Plain,
    }
}

fn notes_lines(sections: &[Section], width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for section in sections {
        if !lines.is_empty() {
            lines.push(String::new());
        }
        lines.extend(overlay::wrap(
            &format!(
                "{} {} {}",
                section.date.day(),
                MONTHS[section.date.month0() as usize],
                section.date.year()
            ),
            width,
        ));
        for item in &section.items {
            let wrapped = overlay::wrap(item, width.saturating_sub(NOTES_BULLET.len()));
            lines.extend(wrapped.into_iter().enumerate().map(|(index, line)| {
                let prefix = if index == 0 {
                    NOTES_BULLET
                } else {
                    NOTES_INDENT
                };
                format!("{prefix}{line}")
            }));
        }
    }
    lines
}

pub fn scroll_limit(lines: &[String], rows: usize) -> usize {
    lines.len().saturating_sub(rows.max(1))
}

pub fn scrolled_view(
    title: &str,
    lines: Vec<String>,
    first: usize,
    rows: usize,
    footer: &str,
    frame: Frame,
) -> View {
    let rows = rows.max(1);
    let is_scrollable = lines.len() > rows;
    let first = first.min(scroll_limit(&lines, rows));
    View {
        title: title.to_string(),
        lines: lines
            .into_iter()
            .skip(first)
            .take(rows)
            .map(Line::Preformatted)
            .collect(),
        footer: if is_scrollable {
            format!("{SCROLL_HINT}  {footer}")
        } else {
            footer.to_string()
        },
        frame,
    }
}

pub fn notes_scroll_limit(sections: &[Section], rows: usize, width: usize) -> usize {
    scroll_limit(&notes_lines(sections, width), rows)
}

pub fn notes_view(sections: &[Section], first: usize, rows: usize, width: usize) -> View {
    scrolled_view(
        "Nouveautés",
        notes_lines(sections, width),
        first,
        rows,
        CLOSE_FOOTER,
        NOTES_FRAME,
    )
}

fn gifts(unlocks: &Unlocks) -> Vec<&'static str> {
    [
        (unlocks.bench, "Le banc de Paulo"),
        (unlocks.cat_bow, "Le nœud de Minouche"),
        (unlocks.well, "Le vieux puits réparé"),
        (unlocks.bone, "L'os de Biscuit"),
        (unlocks.key, "La petite clé"),
        (unlocks.treehouse, "La cabane de Léo"),
        (unlocks.owl_friend, "Le cousin grand-duc"),
        (unlocks.garland, "La guirlande du porche"),
        (unlocks.kittens, "Les chatons de Minouche"),
        (unlocks.balloon, "La montgolfière de la fête"),
        (unlocks.ducklings, "Les canetons de Coin-Coin"),
        (unlocks.weather_vane, "La girouette de Gaston"),
        (unlocks.bunting, "Les décorations de la fête"),
        (unlocks.chest, "Le coffre de Jeanne"),
    ]
    .into_iter()
    .filter(|(is_unlocked, _)| *is_unlocked)
    .map(|(_, name)| name)
    .collect()
}

fn list_page(title: &str, items: Vec<String>) -> View {
    let items = items.into_iter().map(|item| format!("- {item}")).collect();
    album_page(title, items, ALBUM_FOOTER)
}

fn album_page(title: &str, items: Vec<String>, footer: &str) -> View {
    let lines = if items.is_empty() {
        vec![text("Rien pour l'instant.")]
    } else {
        items.into_iter().map(text).collect()
    };
    View {
        title: title.to_string(),
        lines,
        footer: footer.to_string(),
        frame: Frame::Photo,
    }
}

fn answers_page(progress: &Progress, selected: usize) -> View {
    let items: Vec<String> = answered_choices(&progress.answers)
        .enumerate()
        .map(|(index, (choice, answer))| {
            choice_line(choice.prompt, &choice.options, answer, index == selected)
        })
        .collect();
    choices_page("Album : tes réponses", items)
}

fn trophies_page(progress: &Progress, selected: usize) -> View {
    let items: Vec<String> = Trophy::won(&progress.badges)
        .enumerate()
        .map(|(index, trophy)| {
            let place = usize::from(progress.hidden_trophies.contains(trophy.id()));
            choice_line(trophy.name(), &trophy.places(), place, index == selected)
        })
        .collect();
    choices_page("Album : tes trophées", items)
}

fn choices_page(title: &str, items: Vec<String>) -> View {
    let footer = if items.is_empty() {
        ALBUM_FOOTER
    } else {
        ANSWERS_FOOTER
    };
    album_page(title, items, footer)
}

fn choice_line(label: &str, options: &[&str], answer: usize, is_selected: bool) -> String {
    if is_selected {
        format!("> {label} {}", both_options(options, answer))
    } else {
        format!("- {label} {}", options[answer])
    }
}

fn both_options(options: &[&str], answer: usize) -> String {
    options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            let mark = if index == answer { "\u{a0}✓" } else { "" };
            format!("[{}]\u{a0}{option}{mark}", index + 1)
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn album_pages(
    read_entries: &[(usize, Entry)],
    progress: &Progress,
    unlocks: &Unlocks,
    selected: usize,
) -> Vec<View> {
    let postcards: Vec<usize> = read_entries
        .iter()
        .filter_map(|(_, entry)| match entry {
            Entry::Postcard(index) => Some(*index),
            _ => None,
        })
        .collect();
    let recipes: Vec<usize> = read_entries
        .iter()
        .filter_map(|(_, entry)| entry.recipe_index())
        .collect();
    let drawings: Vec<&[&str]> = DRAWINGS
        .iter()
        .filter(|(letter, _)| {
            read_entries
                .iter()
                .any(|(_, entry)| *entry == Entry::Main(letter - 1))
        })
        .map(|(_, drawing)| *drawing)
        .collect();
    let gifts = gifts(unlocks);

    let summary = vec![
        format!("Cadeaux du village : {}", gifts.len()),
        format!(
            "Objets trouvés : {}/{}",
            progress.found_objects.len(),
            OBJECTS.len()
        ),
        format!("Bouteilles repêchées : {}", progress.bottles.len()),
        format!("Cartes postales : {}", postcards.len()),
        format!("Recettes : {}", recipes.len()),
        format!("Dessins de Léo : {}", drawings.len()),
    ];
    let objects = progress
        .found_objects
        .iter()
        .map(|index| OBJECTS[*index].to_string())
        .collect();

    let mut pages = vec![
        list_page("Album : sommaire", summary),
        list_page(
            "Album : cadeaux du village",
            gifts.iter().map(|gift| gift.to_string()).collect(),
        ),
        answers_page(progress, selected),
        trophies_page(progress, selected),
        list_page("Album : objets trouvés", objects),
    ];
    if unlocks.chest {
        pages.push(list_page(
            "Album : souvenirs du coffre",
            CHEST_SOUVENIRS
                .iter()
                .map(|item| item.to_string())
                .collect(),
        ));
    }
    pages.extend(
        progress
            .bottles
            .iter()
            .enumerate()
            .map(|(rank, index)| View {
                title: format!("Album : bouteille n°{}", rank + 1),
                lines: vec![text(BOTTLE_MESSAGES[*index])],
                footer: ALBUM_FOOTER.to_string(),
                frame: Frame::Photo,
            }),
    );
    pages.extend(postcards.iter().map(|index| View {
        title: format!("Album : carte de {}", postcard_title(*index)),
        lines: postcard_lines(*index, progress),
        footer: ALBUM_FOOTER.to_string(),
        frame: Frame::Photo,
    }));
    pages.extend(recipes.iter().map(|index| {
        let recipe = &RECIPES[*index];
        View {
            title: format!("Album : recette, {}", recipe.name),
            lines: vec![text(recipe.body)],
            footer: ALBUM_FOOTER.to_string(),
            frame: Frame::Photo,
        }
    }));
    pages.extend(drawings.iter().map(|drawing| {
        View {
            title: "Album : dessin de Léo".to_string(),
            lines: drawing
                .iter()
                .map(|row| Line::Preformatted(row.to_string()))
                .collect(),
            footer: ALBUM_FOOTER.to_string(),
            frame: Frame::Photo,
        }
    }));

    let count = pages.len();
    for (index, page) in pages.iter_mut().enumerate() {
        page.title = format!("{} ({}/{count})", page.title, index + 1);
    }
    pages
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::badges::{self, Tier};
    use crate::holiday::Holiday;
    use crate::story::content::letters::CHOICES;
    use crate::story::content::save_golden::{SavedGame, courrier_fixture, render};
    use std::collections::BTreeMap;

    #[test]
    fn test_help_lists_the_calendar_only_in_december() {
        let has_calendar = |is_december| {
            help_view(&Unlocks::default(), false, is_december)
                .lines
                .iter()
                .any(|line| matches!(line, Line::Preformatted(text) if text.starts_with("[v]")))
        };
        assert!(has_calendar(true));
        assert!(!has_calendar(false));
    }

    #[test]
    fn test_save_lists_in_courrier_as_before_refactor() {
        let save = SavedGame::load();
        let opened = save.progress.letters_read.min(save.available - 1);
        let read = save
            .timeline
            .first_mail(save.progress.letters_read.min(save.available));

        let mut out = format!("opens: {opened}\n");
        out.push_str(&render(&index_view(
            read,
            Tab::Letters,
            0,
            200,
            &save.context(12),
        )));
        assert_eq!(out, courrier_fixture());
    }

    #[test]
    fn test_gazette_rows_are_named_after_their_date() {
        let save = SavedGame::load();
        let read = save
            .timeline
            .first_mail(save.progress.letters_read.min(save.available));
        let view = index_view(read, Tab::Gazettes, 0, 200, &save.context(12));
        let rows: Vec<String> = view
            .lines
            .iter()
            .filter_map(|line| match line {
                Line::Preformatted(row) if row.contains("Gazette du") => Some(row.clone()),
                _ => None,
            })
            .collect();

        assert!(!rows.is_empty());
        assert!(rows[0].ends_with("07/09  Gazette du lundi 7 septembre"));
        for row in &rows {
            let (date, title) = row[7..].split_once("  ").unwrap();
            let (day, month) = date.split_once('/').unwrap();
            let month = MONTHS[month.parse::<usize>().unwrap() - 1];
            let day: u32 = day.parse().unwrap();
            assert_eq!(title, format!("Gazette du lundi {day} {month}"));
        }
    }

    fn day(month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, month, day).unwrap()
    }

    const FULL_BOX_NOTE_TEXT: &str = "text: Pendant ton absence, tout le village en a profité pour prendre des vacances\u{202F}!\n";

    #[test]
    fn test_full_box_note_sits_on_top_of_the_courrier() {
        let save = SavedGame::load();
        let read = save.timeline.first_mail(10);
        let context = save.context(12);
        let plain = render(&index_view(read, Tab::Letters, 0, 20, &context));
        let full = render(&full_box_index_view(
            read,
            Tab::Letters,
            0,
            20,
            200,
            &context,
            day(10, 31),
        ));

        assert!(!plain.contains(FULL_BOX_NOTE_TEXT));
        assert!(full.contains(&format!(
            "{FULL_BOX_NOTE_TEXT}\
             text: La rentrée, c'est le 31 octobre.\n\
             blank\n\
             text: — Le Facteur\n\
             rule\n"
        )));
        assert!(
            full.contains(&plain[plain.find("pre: >").unwrap()..plain.find("footer").unwrap()])
        );
    }

    #[test]
    fn test_postman_note_has_its_own_frame() {
        let note = postman_note_view(day(10, 31));
        assert_eq!(note.frame, Frame::Notice);
        assert!(render(&note).contains(&format!(
            "{FULL_BOX_NOTE_TEXT}\
             text: La rentrée, c'est le 31 octobre.\n\
             blank\n\
             text: — Le Facteur\n"
        )));
    }

    #[test]
    fn test_neighbours_note_is_signed_in_its_own_frame() {
        let announcement =
            neighbours_note_view(NeighboursNote::Announcement(Punishment::MailBan(3)));
        let disappointed = neighbours_note_view(NeighboursNote::disappointed(&mut rand::rng()));
        for note in [&announcement, &disappointed] {
            assert_eq!(note.frame, Frame::Roofs);
            assert!(render(note).contains(&format!(
                "blank\ntext: {}\nfooter: {CLOSE_FOOTER}\n",
                neighbours_note::SIGNATURE
            )));
        }
        assert!(render(&announcement).contains("3 jours"));
    }

    #[test]
    fn test_cheater_announcement_shows_the_badge_medal() {
        let medal_lines = |note| {
            neighbours_note_view(note)
                .lines
                .iter()
                .filter(|line| matches!(line, Line::Centered(_)))
                .count()
        };
        assert_eq!(
            medal_lines(NeighboursNote::Announcement(Punishment::Cheater)),
            5
        );
        assert_eq!(
            medal_lines(NeighboursNote::Announcement(Punishment::BareVillage)),
            0
        );
    }

    #[test]
    fn test_backup_box_names_the_copy_in_the_floppy_frame() {
        let view = backup_view(Path::new("/weathr/progress.json.abime-2026-11-07-1432"));
        assert_eq!(
            render(&view),
            "title: Sauvegarde\n\
             text: Ta progression n'a pas pu être lue. Une copie est gardée dans \
             /weathr/progress.json.abime-2026-11-07-1432. Une nouvelle partie commence.\n\
             footer: [Entrée] ou [Echap] fermer\n\
             frame: Floppy\n"
        );
    }

    #[test]
    fn test_restored_box_says_the_game_resumes_in_the_floppy_frame() {
        assert_eq!(
            render(&restored_view()),
            "title: Sauvegarde\n\
             text: Ta progression n'a pas pu être lue. Ta partie reprend à la dernière sauvegarde.\n\
             footer: [Entrée] ou [Echap] fermer\n\
             frame: Floppy\n"
        );
    }

    #[test]
    fn test_back_to_school_says_1er_on_the_first() {
        let rentree = |date| render(&postman_note_view(date));
        assert!(rentree(day(11, 1)).contains("La rentrée, c'est le 1er novembre."));
        assert!(rentree(day(11, 2)).contains("La rentrée, c'est le 2 novembre."));
        assert!(
            rentree(NaiveDate::from_ymd_opt(2027, 1, 1).unwrap())
                .contains("La rentrée, c'est le 1er janvier.")
        );
    }

    #[test]
    fn test_wrapped_note_leaves_the_courrier_within_its_rows() {
        let save = SavedGame::load();
        let read = save.timeline.first_mail(10);
        let context = save.context(12);
        let (rows, width) = (12, 30);
        let view = full_box_index_view(read, Tab::Letters, 0, rows, width, &context, day(10, 31));

        assert!(overlay::lay_out(&view.lines, width).len() <= rows);
        assert!(render(&view).contains("pre: >"));
    }

    fn centered(view: &View) -> Vec<&str> {
        view.lines
            .iter()
            .filter_map(|line| match line {
                Line::Centered(text) => Some(text.as_str()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn test_medal_frames_the_badge_icon() {
        assert_eq!(
            medal(badges::find("rain").unwrap().icon),
            [
                " .-\"\"\"-. ",
                "/ (___) \\",
                "|  / /  |",
                "\\ / / / /",
                " '-...-' ",
            ]
        );
    }

    fn rows(view: &View) -> Vec<&str> {
        view.lines
            .iter()
            .filter_map(|line| match line {
                Line::Preformatted(text) => Some(text.as_str()),
                _ => None,
            })
            .collect()
    }

    fn sections() -> Vec<Section> {
        crate::release_notes::parse(
            "## 2026-10-12\n- Ancienne\n## 2026-10-26\n- Première\n- Deuxième\n",
        )
    }

    #[test]
    fn test_notes_list_each_section_under_its_date_newest_first() {
        let view = notes_view(&sections(), 0, 20, 60);
        assert_eq!(view.title, "Nouveautés");
        assert_eq!(
            rows(&view),
            [
                "26 octobre 2026",
                "- Première",
                "- Deuxième",
                "",
                "12 octobre 2026",
                "- Ancienne"
            ]
        );
        assert_eq!(view.footer, CLOSE_FOOTER);
    }

    #[test]
    fn test_long_notes_wrap_in_the_box_and_scroll_to_the_last_line() {
        let sections = crate::release_notes::parse(
            "## 2026-10-26\n- Les Perséides font pleuvoir les étoiles filantes la nuit\n",
        );
        let (height, width) = (3, 20);

        assert_eq!(notes_view(&sections, 0, 20, width).footer, CLOSE_FOOTER);
        let top = notes_view(&sections, 0, height, width);
        assert_eq!(
            rows(&top),
            ["26 octobre 2026", "- Les Perséides font", "  pleuvoir les"]
        );
        assert_eq!(top.footer, "[↑] [↓] défiler  [Entrée] ou [Echap] fermer");

        let last = notes_scroll_limit(&sections, height, width);
        assert_eq!(last, 2);
        let bottom = notes_view(&sections, last + 5, height, width);
        assert_eq!(
            rows(&bottom),
            ["  pleuvoir les", "  étoiles filantes", "  la nuit"]
        );
        assert!(rows(&bottom).iter().all(|row| row.chars().count() <= width));
    }

    #[test]
    fn test_badge_card_names_the_badge_and_its_secret_condition() {
        let rain = badge_card_view(badges::find("rain").unwrap().into());
        assert_eq!(rain.frame, Frame::Stars);
        assert_eq!(centered(&rain)[1], "/ (___) \\");
        assert_eq!(centered(&rain)[5..], ["Nouveau badge\u{202F}!", "Pluie"]);

        let patience = badge_card_view(badges::find("patience").unwrap().into());
        assert_eq!(
            centered(&patience)[5..],
            [
                "Nouveau badge\u{202F}!",
                "Patience",
                "weathr ouvert 1 h d'affilée"
            ]
        );
    }

    #[test]
    fn test_sam_trail_card_is_a_new_badge_then_a_new_tier() {
        let chocolate = badge_card_view(Award::sam_trail(Tier::Chocolate));
        assert_eq!(
            centered(&chocolate)[5..],
            [
                "Nouveau badge\u{202F}!",
                "Sur les traces de Sam · chocolat",
                "visité la même ville que Sam"
            ]
        );

        let bronze = badge_card_view(Award::sam_trail(Tier::Bronze));
        assert_eq!(centered(&bronze)[..5], centered(&chocolate)[..5]);
        assert_eq!(
            centered(&bronze)[5..],
            [
                "Nouveau palier\u{202F}!",
                "Sur les traces de Sam · bronze",
                "5 villes de Sam visitées"
            ]
        );
    }

    #[test]
    fn test_holiday_box_names_the_holiday_and_what_to_look_for() {
        let music = Holiday::music_day().holiday_box().unwrap();
        assert_eq!(
            render(&holiday_box_view(music)),
            "title: Fête de la musique\n\
             text: Ce soir, le village chante.\n\
             text: Ouvre l'œil\u{202F}: des notes s'échappent des fenêtres.\n\
             footer: [Echap] fermer\n\
             frame: Plain\n"
        );
    }

    #[test]
    fn test_settings_view_points_at_the_selected_row() {
        let mut settings = Settings {
            units: crate::weather::WeatherUnits::default(),
            mail_hour: 7,
            hide_hud: true,
            city: "Lyon".to_string(),
            auto_location: AutoLocation::Off,
            diets: None,
        };
        let view = settings_view(&settings, 3, false);
        let lines = overlay::lay_out(&view.lines, 60);
        assert_eq!(lines.len(), 7);
        assert!(lines[0].starts_with("  Température") && lines[0].ends_with("°C"));
        assert!(lines[3].starts_with("> Heure du Facteur") && lines[3].ends_with("< 7 h >"));
        assert!(lines[4].ends_with("masquée"));
        assert!(lines[5].starts_with("  Ville") && lines[5].ends_with("Lyon"));
        assert!(lines[6].starts_with("  Position auto") && lines[6].ends_with("non"));

        for (state, value) in [(AutoLocation::Locating, "…"), (AutoLocation::On, "oui")] {
            settings.auto_location = state;
            let view = settings_view(&settings, 0, false);
            assert!(overlay::lay_out(&view.lines, 60)[6].ends_with(value));
        }

        let view = settings_view(&settings, 0, true);
        let lines = overlay::lay_out(&view.lines, 60);
        assert_eq!(
            lines.last().unwrap(),
            "Nouvelle heure du Facteur dès demain."
        );
    }

    #[test]
    fn test_diet_row_sums_up_the_checked_diets() {
        let mut settings = Settings {
            units: crate::weather::WeatherUnits::default(),
            mail_hour: 9,
            hide_hud: false,
            city: "Lyon".to_string(),
            auto_location: AutoLocation::Off,
            diets: Some(BTreeSet::new()),
        };
        let row = |settings: &Settings| {
            overlay::lay_out(&settings_view(settings, 7, false).lines, 60)[7].clone()
        };
        assert!(row(&settings).starts_with("> Régime") && row(&settings).ends_with("< aucun >"));

        settings.diets = Some(BTreeSet::from([Diet::Vegetarian, Diet::NoPork]));
        assert!(row(&settings).ends_with("< végétarien, sans porc >"));

        settings.diets = Some(BTreeSet::from([Diet::Vegan, Diet::NoPork, Diet::NoFruit]));
        assert!(row(&settings).ends_with("< 3 Régimes >"));
    }

    #[test]
    fn test_diet_view_reuses_the_letter_checklist() {
        let diets = BTreeSet::from([Diet::NoFish]);
        let view = diet_view(&diets);
        assert_eq!(view.title, "Régime");
        assert_eq!(
            overlay::lay_out(&view.lines, 60)[2],
            "[5] ✓ sans poisson    [6]   sans porc"
        );
        assert_eq!(view.footer, DIET_FOOTER);
    }

    #[test]
    fn test_city_search_lists_results_under_the_query() {
        use crate::geolocation::{GeoLocation, Place};
        let place = |label: &str| Place {
            label: label.to_string(),
            location: GeoLocation {
                latitude: 45.76,
                longitude: 4.83,
                city: Some("Lyon".to_string()),
            },
        };
        let mut search = CitySearch {
            query: "Lyon".to_string(),
            status: SearchStatus::Found(
                vec![place("Lyon, Rhône, France"), place("Lyon, Mississippi")],
                1,
            ),
        };
        let view = city_search_view(&search);
        assert_eq!(
            overlay::lay_out(&view.lines, 60),
            [
                "> Lyon_",
                "",
                "  Lyon, Rhône, France",
                "> Lyon, Mississippi"
            ]
        );
        assert_eq!(view.footer, CITY_RESULTS_FOOTER);

        search.status = SearchStatus::NotFound;
        let view = city_search_view(&search);
        assert_eq!(
            overlay::lay_out(&view.lines, 60),
            ["> Lyon_", "", "Aucune ville trouvée"]
        );
        assert_eq!(view.footer, CITY_SEARCH_FOOTER);
    }

    #[test]
    fn test_answers_page_shows_both_options_of_the_selected_question() {
        let mut progress = Progress::default();
        progress.answers = BTreeMap::from([("bow".to_string(), 1), ("well".to_string(), 2)]);
        let [bow, well] = [&CHOICES[0], &CHOICES[1]];
        let pages = |selected| album_pages(&[], &progress, &Unlocks::default(), selected);

        let pages_on_first = pages(0);
        let view = &pages_on_first[ANSWERS_PAGE];
        assert!(view.title.starts_with("Album : tes réponses"));
        assert_eq!(view.footer, ANSWERS_FOOTER);
        assert_eq!(
            overlay::lay_out(&view.lines, 200),
            [
                format!("> {} [1]\u{a0}Rouge\u{a0}✓ [2]\u{a0}Bleu", bow.prompt),
                format!("- {} De la mousse verte", well.prompt),
            ]
        );
        assert!(
            pages_on_first
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != ANSWERS_PAGE)
                .all(|(_, page)| page.footer == ALBUM_FOOTER)
        );

        let view = &pages(1)[ANSWERS_PAGE];
        assert_eq!(
            overlay::lay_out(&view.lines, 200),
            [
                format!("- {} Rouge", bow.prompt),
                format!(
                    "> {} [1]\u{a0}Des tuiles rouges [2]\u{a0}De la mousse verte\u{a0}✓",
                    well.prompt
                ),
            ]
        );
        for width in 20..=60 {
            assert!(
                overlay::lay_out(&view.lines, width)
                    .iter()
                    .all(|line| !line.ends_with(']') && !line.starts_with('✓'))
            );
        }
    }

    #[test]
    fn test_album_opens_on_its_fixed_pages_in_order() {
        let pages = album_pages(&[], &Progress::default(), &Unlocks::default(), 0);
        let titles = [
            "Album : sommaire",
            "Album : cadeaux du village",
            "Album : tes réponses",
            "Album : tes trophées",
            "Album : objets trouvés",
        ];
        for (page, title) in pages.iter().zip(titles) {
            assert!(page.title.starts_with(title), "{}", page.title);
        }
        assert!(
            pages[ANSWERS_PAGE]
                .title
                .starts_with("Album : tes réponses")
        );
        assert!(
            pages[TROPHIES_PAGE]
                .title
                .starts_with("Album : tes trophées")
        );
    }

    #[test]
    fn test_trophies_page_lists_won_trophies_only() {
        let pages = album_pages(&[], &Progress::default(), &Unlocks::default(), 0);
        let view = &pages[TROPHIES_PAGE];
        assert_eq!(overlay::lay_out(&view.lines, 60), ["Rien pour l'instant."]);
        assert_eq!(view.footer, ALBUM_FOOTER);

        let mut progress = Progress::default();
        progress.badges.insert("streak_60".to_string());
        let pages = album_pages(&[], &progress, &Unlocks::default(), 0);
        let view = &pages[TROPHIES_PAGE];
        assert_eq!(
            overlay::lay_out(&view.lines, 200),
            ["> Nain de jardin [1]\u{a0}au jardin\u{a0}✓ [2]\u{a0}rangé"]
        );
        assert_eq!(view.footer, ANSWERS_FOOTER);
    }

    #[test]
    fn test_trophies_page_shows_where_each_trophy_is() {
        let mut progress = Progress::default();
        progress.badges = BTreeSet::from([
            "streak_60".to_string(),
            "streak_100".to_string(),
            "streak_365".to_string(),
        ]);
        progress.hidden_trophies.insert("statue".to_string());
        progress.hidden_trophies.insert("sundial".to_string());
        let pages = album_pages(&[], &progress, &Unlocks::default(), 1);
        assert_eq!(
            overlay::lay_out(&pages[TROPHIES_PAGE].lines, 200),
            [
                "- Nain de jardin au jardin",
                "> Statue [1]\u{a0}au jardin [2]\u{a0}rangée\u{a0}✓",
                "- Cadran solaire rangé",
            ]
        );
    }

    #[test]
    fn test_answers_page_stays_empty_until_a_question_is_answered() {
        let pages = album_pages(&[], &Progress::default(), &Unlocks::default(), 0);
        let view = &pages[ANSWERS_PAGE];
        assert!(view.title.starts_with("Album : tes réponses"));
        assert_eq!(overlay::lay_out(&view.lines, 60), ["Rien pour l'instant."]);
        assert_eq!(view.footer, ALBUM_FOOTER);
    }
}
