use crate::animation::lore::Unlocks;
use crate::overlay::{Line, View};
use crate::progress::Progress;
use crate::story::Entry;
use crate::story::content::chronicles::{
    BOTTLE_MESSAGES, CHEST_SOUVENIRS, DRAWINGS, OBJECTS, RECIPES,
};
use crate::story::content::letters::CHOICES;
use crate::story::content::{self, ReadContext, postcard_lines, postcard_title, text};

const INDEX_FOOTER: &str = "[↑] [↓] choisir  [Entrée] relire  [l] ou [Echap] fermer";
const ALBUM_FOOTER: &str = "[<] [>] pages  [a] ou [Echap] fermer";
const BOTTLE_FOOTER: &str = "[Echap] fermer";
const HELP_FOOTER: &str = "[h] ou [Echap] fermer";
const HELP_KEY_WIDTH: usize = 10;

pub fn index_view(
    read_entries: &[(usize, Entry)],
    selected: usize,
    rows: usize,
    context: &ReadContext,
) -> View {
    let rows = rows.max(1);
    let first = selected
        .saturating_sub(rows / 2)
        .min(read_entries.len().saturating_sub(rows));
    let mut lines: Vec<Line> = read_entries
        .iter()
        .enumerate()
        .skip(first)
        .take(rows)
        .map(|(index, (day, entry))| {
            let marker = if index == selected { '>' } else { ' ' };
            Line::Preformatted(format!(
                "{marker} {:>3}. {}  {}",
                index + 1,
                context.clock.date_of(*day).format("%d/%m"),
                content::label(*entry, context)
            ))
        })
        .collect();
    if lines.is_empty() {
        lines.push(text("Aucune lettre lue pour l'instant."));
    }
    View {
        title: format!("Lettres reçues ({})", read_entries.len()),
        lines,
        footer: INDEX_FOOTER.to_string(),
    }
}

pub fn bottle_view(index: usize) -> View {
    View {
        title: "Message dans une bouteille".to_string(),
        lines: vec![text(BOTTLE_MESSAGES[index])],
        footer: BOTTLE_FOOTER.to_string(),
    }
}

pub fn help_view(unlocks: &Unlocks, has_gazette: bool) -> View {
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
    }
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
    let lines = if items.is_empty() {
        vec![text("Rien pour l'instant.")]
    } else {
        items
            .into_iter()
            .map(|item| text(format!("- {item}")))
            .collect()
    };
    View {
        title: title.to_string(),
        lines,
        footer: ALBUM_FOOTER.to_string(),
    }
}

pub fn album_pages(
    read_entries: &[(usize, Entry)],
    progress: &Progress,
    unlocks: &Unlocks,
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
        .filter_map(|(_, entry)| match entry {
            Entry::Recipe(month) => Some(*month),
            _ => None,
        })
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
    let answers = CHOICES
        .iter()
        .filter_map(|choice| {
            let answer = *progress.answers.get(choice.id)?;
            let option = choice.options.get(usize::from(answer).checked_sub(1)?)?;
            Some(format!("{} {option}", choice.prompt))
        })
        .collect();
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
        list_page("Album : tes réponses", answers),
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
            }),
    );
    pages.extend(postcards.iter().map(|index| View {
        title: format!("Album : carte de {}", postcard_title(*index)),
        lines: postcard_lines(*index, progress),
        footer: ALBUM_FOOTER.to_string(),
    }));
    pages.extend(recipes.iter().map(|month| {
        let (name, body) = RECIPES[*month];
        View {
            title: format!("Album : recette, {name}"),
            lines: vec![text(body)],
            footer: ALBUM_FOOTER.to_string(),
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
    use crate::story::content::{entry_view, letters};
    use crate::story::{StoryClock, Timeline, sources};
    use chrono::NaiveDate;

    const SAVE: &str = r#"{
  "started_on": "2026-09-01",
  "letters_read": 37,
  "answers": { "bow": 2, "well": 1 },
  "riddles_solved": [1],
  "postcard_weather": { "0": "Il fait beau à Lisbonne." },
  "gazette_weather": { "0": "Ta semaine : soleil et 21 °C.", "3": "Ta semaine : pluie." }
}"#;

    fn render(view: &View) -> String {
        let mut out = format!("title: {}\n", view.title);
        for line in &view.lines {
            match line {
                Line::Text(text) => out.push_str(&format!("text: {text}\n")),
                Line::Preformatted(text) => out.push_str(&format!("pre: {text}\n")),
                Line::Blank => out.push_str("blank\n"),
            }
        }
        out.push_str(&format!("footer: {}\n", view.footer));
        out
    }

    #[test]
    fn test_save_opens_and_lists_as_before_refactor() {
        let letters = letters::defaults();
        let progress: Progress = serde_json::from_str(SAVE).unwrap();
        let clock = StoryClock::new(progress.start_date().unwrap(), 0);
        let today = clock.day_on(
            NaiveDate::from_ymd_opt(2026, 10, 26)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
        );
        let timeline = Timeline::new(&sources::registered(letters.len()), &clock, today);
        let available = timeline.delivered_by(today);
        let read: Vec<(usize, Entry)> = timeline
            .mail()
            .take(progress.letters_read.min(available))
            .collect();

        let mut out = format!("opens: {}\n", progress.letters_read.min(available - 1));
        for (is_night, reading_hour) in [(false, 12), (true, 23)] {
            let context = ReadContext {
                letters: &letters,
                progress: &progress,
                clock: &clock,
                reading_hour,
                riddle_input: None,
            };
            if !is_night {
                out.push_str(&render(&index_view(&read, 0, 200, &context)));
            }
            for (index, (day, entry)) in timeline.mail().take(available).enumerate() {
                out.push_str(&format!("== entry {index} night {is_night}\n"));
                out.push_str(&render(&entry_view(
                    day,
                    entry,
                    index + 1,
                    available,
                    &context,
                )));
            }
        }
        assert_eq!(out, include_str!("story/testdata/save_37_golden.txt"));
    }
}
