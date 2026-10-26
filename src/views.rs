use crate::animation::lore::Unlocks;
use crate::badges::Badge;
use crate::chronicles::{
    ADS, ARCHIVES, BOTTLE_MESSAGES, CHEST_SOUVENIRS, CITIES, DRAWINGS, MARCEL_LETTERS,
    MARCEL_SENDER, NEIGHBOURS_LETTERS, NEIGHBOURS_SENDER, OBJECTS, RECIPES, RIDDLES, RUMORS,
    SAM_LETTERS, SAM_SENDER,
};
use crate::letters::{self, CHOICES, Entry, Letter};
use crate::overlay::{Frame, Line, View};
use crate::progress::Progress;
use crate::release_notes::Section;
use chrono::{Datelike, NaiveDate};

const LETTER_FOOTER: &str = "[c] suivante  [<] [>] parcourir  [l] sommaire  [Echap] fermer";
const INDEX_FOOTER: &str = "[↑] [↓] choisir  [Entrée] relire  [l] ou [Echap] fermer";
const QUESTION_FOOTER: &str = "[1] [2] répondre  [c] suivante  [<] [>]  [Echap] fermer";
const RIDDLE_FOOTER: &str = "[e] répondre à l'énigme  [c] suivante  [<] [>]  [Echap]";
const INPUT_FOOTER: &str = "Tape ta réponse puis [Entrée]  [Echap] annuler";
const ALBUM_FOOTER: &str = "[<] [>] pages  [a] ou [Echap] fermer";
const BOTTLE_FOOTER: &str = "[Echap] fermer";
const BADGE_CARD_FOOTER: &str = "[Entrée] ou [Echap] fermer";
const HELP_FOOTER: &str = "[h] ou [Echap] fermer";
const NOTES_FOOTER: &str = "[Entrée] ou [Echap] fermer";
const NOTES_SCROLL_FOOTER: &str = "[↑] [↓] défiler  [Entrée] ou [Echap] fermer";
const HELP_KEY_WIDTH: usize = 10;
const MONTHS: [&str; 12] = [
    "janvier",
    "février",
    "mars",
    "avril",
    "mai",
    "juin",
    "juillet",
    "août",
    "septembre",
    "octobre",
    "novembre",
    "décembre",
];

pub struct MailContext<'a> {
    pub letters: &'a [Letter],
    pub progress: &'a Progress,
    pub start: NaiveDate,
    pub is_night: bool,
    pub riddle_input: Option<&'a str>,
}

fn text(content: impl Into<String>) -> Line {
    Line::Text(content.into())
}

fn signed(title: String, body: &str, sender: &str, frame: Frame) -> View {
    View {
        title,
        lines: vec![text(body), Line::Blank, text(format!("— {sender}"))],
        footer: LETTER_FOOTER.to_string(),
        frame,
    }
}

pub fn entry_view(entry: Entry, number: usize, total: usize, context: &MailContext) -> View {
    let counter = format!("{number}/{total}");
    match entry {
        Entry::Main(main) => village_view(main, &counter, context),
        Entry::Creepy(index) => {
            let letter = letters::creepy_letter(index);
            signed(
                format!("Lettre {counter} (glissée sous la porte)"),
                &letter.body,
                &letter.sender,
                Frame::Parchment,
            )
        }
        Entry::Sam(index) => signed(
            format!("Lettre {counter}"),
            SAM_LETTERS[index],
            SAM_SENDER,
            Frame::Parchment,
        ),
        Entry::Neighbours(index) => signed(
            format!("Lettre {counter}"),
            NEIGHBOURS_LETTERS[index],
            NEIGHBOURS_SENDER,
            Frame::Parchment,
        ),
        Entry::Marcel(index) => signed(
            format!("Lettre {counter}"),
            MARCEL_LETTERS[index],
            MARCEL_SENDER,
            Frame::Parchment,
        ),
        Entry::Postcard(index) => postcard_view(index, &counter, context.progress),
        Entry::Archive(index) => {
            let (source, body) = ARCHIVES[index];
            signed(
                format!("Archives de la mairie {counter} : {source}"),
                body,
                "Copie transmise par Odile",
                Frame::Folder,
            )
        }
        Entry::Gazette(week) => gazette_view(week, context),
        Entry::Recipe(month) => {
            let (name, body) = RECIPES[month];
            signed(
                format!("Recette du mois {counter} : {name}"),
                body,
                "Rose, la boulangère",
                Frame::Recipe,
            )
        }
    }
}

fn entry_label(entry: Entry, context: &MailContext) -> String {
    match entry {
        Entry::Main(main) => context.letters[main].sender.clone(),
        Entry::Sam(_) => SAM_SENDER.to_string(),
        Entry::Neighbours(_) => NEIGHBOURS_SENDER.to_string(),
        Entry::Marcel(_) => MARCEL_SENDER.to_string(),
        Entry::Archive(index) => format!("Archives : {}", ARCHIVES[index].0),
        Entry::Gazette(_) => "La Gazette du lundi".to_string(),
        Entry::Recipe(month) => format!("Recette du mois : {}", RECIPES[month].0),
        Entry::Postcard(index) => format!("Carte postale : {}", CITIES[index % CITIES.len()].name),
        Entry::Creepy(_) => letters::CREEPY_SENDER.to_string(),
    }
}

pub fn index_view(
    read_entries: &[(usize, Entry)],
    selected: usize,
    rows: usize,
    context: &MailContext,
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
            let date = context.start + chrono::Duration::days(*day as i64 - 1);
            Line::Preformatted(format!(
                "{marker} {:>3}. {}  {}",
                index + 1,
                date.format("%d/%m"),
                entry_label(*entry, context)
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
        frame: Frame::Plain,
    }
}

fn village_view(main: usize, counter: &str, context: &MailContext) -> View {
    let letter = &context.letters[main];
    let number = main + 1;
    let answers = &context.progress.answers;
    let mut lines: Vec<Line> = letter
        .body
        .lines()
        .map(|paragraph| {
            if paragraph.trim().is_empty() {
                Line::Blank
            } else {
                text(paragraph)
            }
        })
        .collect();

    if let Some((_, drawing)) = DRAWINGS.iter().find(|(letter, _)| *letter == number) {
        lines.push(Line::Blank);
        lines.extend(
            drawing
                .iter()
                .map(|row| Line::Preformatted(row.to_string())),
        );
    }
    if let Some(echo) = letters::echo_for(number, answers) {
        lines.extend([Line::Blank, text(echo)]);
    }
    if context.is_night {
        let postscript = letters::NIGHT_POSTSCRIPTS[main % letters::NIGHT_POSTSCRIPTS.len()];
        lines.extend([Line::Blank, text(postscript)]);
    }
    lines.extend([Line::Blank, text(format!("— {}", letter.sender))]);

    let mut footer = LETTER_FOOTER;
    if let Some(choice) = letters::choice_for(number) {
        lines.extend([Line::Blank, text(format!("Question : {}", choice.prompt))]);
        let answer = answers
            .get(choice.id)
            .and_then(|answer| choice.options.get(usize::from(*answer).checked_sub(1)?));
        match answer {
            Some(option) => lines.push(text(format!("Ta réponse : {option}"))),
            None => {
                lines.push(text(format!(
                    "[1] {}   [2] {}",
                    choice.options[0], choice.options[1]
                )));
                footer = QUESTION_FOOTER;
            }
        }
    }

    View {
        title: format!("Lettre {counter}"),
        lines,
        footer: footer.to_string(),
        frame: Frame::Parchment,
    }
}

fn postcard_lines(index: usize, progress: &Progress) -> Vec<Line> {
    let city = &CITIES[index % CITIES.len()];
    let weather = progress
        .postcard_weather
        .get(&index)
        .map_or("La météo s'est perdue en route.", String::as_str);
    vec![text(city.note), text(weather), Line::Blank, text("— Sam")]
}

fn postcard_title(index: usize) -> String {
    let city = &CITIES[index % CITIES.len()];
    format!("{}, {}", city.name, city.country)
}

fn postcard_view(index: usize, counter: &str, progress: &Progress) -> View {
    View {
        title: format!("Carte postale {counter} : {}", postcard_title(index)),
        lines: postcard_lines(index, progress),
        footer: LETTER_FOOTER.to_string(),
        frame: Frame::Postcard,
    }
}

fn gazette_view(week: usize, context: &MailContext) -> View {
    let date = letters::gazette_date(context.start, week);
    let riddle = &RIDDLES[week % RIDDLES.len()];
    let is_solved = context.progress.riddles_solved.contains(&week);
    let weather = context.progress.gazette_weather.get(&week).map_or(
        "Ta semaine : les relevés ne sont pas encore arrivés.",
        String::as_str,
    );

    let mut lines = vec![
        text(weather),
        Line::Blank,
        text(format!("Les échos : {}", RUMORS[week % RUMORS.len()])),
        Line::Blank,
        text(format!(
            "Petite annonce : {}",
            ADS[(week * 7 + 3) % ADS.len()]
        )),
        Line::Rule,
        text(format!("L'énigme de la semaine : {}", riddle.question)),
    ];

    let footer = if is_solved {
        lines.push(text("Bravo, énigme résolue !"));
        LETTER_FOOTER
    } else if let Some(input) = context.riddle_input {
        lines.push(text(format!("Ta réponse : {input}_")));
        INPUT_FOOTER
    } else {
        RIDDLE_FOOTER
    };

    if week > 0 {
        let previous = &RIDDLES[(week - 1) % RIDDLES.len()];
        lines.extend([
            Line::Blank,
            text(format!(
                "Réponse de la semaine dernière : {}",
                previous.answers[0]
            )),
        ]);
    }

    View {
        title: format!(
            "LA GAZETTE · lundi {} {}",
            date.day(),
            MONTHS[date.month0() as usize]
        ),
        lines,
        footer: footer.to_string(),
        frame: Frame::Newspaper,
    }
}

pub fn bottle_view(index: usize) -> View {
    View {
        title: "Message dans une bouteille".to_string(),
        lines: vec![text(BOTTLE_MESSAGES[index])],
        footer: BOTTLE_FOOTER.to_string(),
        frame: Frame::Waves,
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

pub fn badge_card_view(badge: &Badge) -> View {
    let mut lines = vec![Line::Blank];
    lines.extend(medal(badge.icon).into_iter().map(Line::Centered));
    lines.extend([
        Line::Blank,
        Line::Centered("Nouveau badge\u{202F}!".to_string()),
        Line::Centered(badge.name.to_string()),
    ]);
    lines.extend(badge.hint.map(|hint| Line::Centered(hint.to_string())));
    View {
        title: String::new(),
        lines,
        footer: BADGE_CARD_FOOTER.to_string(),
        frame: Frame::Stars,
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
        frame: Frame::Plain,
    }
}

fn notes_lines(sections: &[Section]) -> Vec<Line> {
    let mut lines = Vec::new();
    for section in sections {
        if !lines.is_empty() {
            lines.push(Line::Blank);
        }
        lines.push(text(format!(
            "{} {} {}",
            section.date.day(),
            MONTHS[section.date.month0() as usize],
            section.date.year()
        )));
        lines.extend(section.items.iter().map(|item| text(format!("- {item}"))));
    }
    lines
}

pub fn notes_scroll_limit(sections: &[Section], rows: usize) -> usize {
    notes_lines(sections).len().saturating_sub(rows.max(1))
}

pub fn notes_view(sections: &[Section], first: usize, rows: usize) -> View {
    let rows = rows.max(1);
    let lines = notes_lines(sections);
    let is_scrollable = lines.len() > rows;
    let first = first.min(lines.len().saturating_sub(rows));
    View {
        title: "Nouveautés".to_string(),
        lines: lines.into_iter().skip(first).take(rows).collect(),
        footer: if is_scrollable {
            NOTES_SCROLL_FOOTER
        } else {
            NOTES_FOOTER
        }
        .to_string(),
        frame: Frame::Plain,
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
        frame: Frame::Photo,
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
                frame: Frame::Photo,
            }),
    );
    pages.extend(postcards.iter().map(|index| View {
        title: format!("Album : carte de {}", postcard_title(*index)),
        lines: postcard_lines(*index, progress),
        footer: ALBUM_FOOTER.to_string(),
        frame: Frame::Photo,
    }));
    pages.extend(recipes.iter().map(|month| {
        let (name, body) = RECIPES[*month];
        View {
            title: format!("Album : recette, {name}"),
            lines: vec![text(body)],
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
    use crate::badges;

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

    fn texts(view: &View) -> Vec<&str> {
        view.lines
            .iter()
            .filter_map(|line| match line {
                Line::Text(text) => Some(text.as_str()),
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
        let view = notes_view(&sections(), 0, 20);
        assert_eq!(view.title, "Nouveautés");
        assert_eq!(
            texts(&view),
            [
                "26 octobre 2026",
                "- Première",
                "- Deuxième",
                "12 octobre 2026",
                "- Ancienne"
            ]
        );
        assert_eq!(view.footer, NOTES_FOOTER);
    }

    #[test]
    fn test_notes_taller_than_the_box_scroll_to_the_last_line() {
        let sections = sections();
        let rows = 3;
        let last = notes_scroll_limit(&sections, rows);
        assert_eq!(last, 3);

        let top = notes_view(&sections, 0, rows);
        assert_eq!(texts(&top), ["26 octobre 2026", "- Première", "- Deuxième"]);
        assert_eq!(top.footer, NOTES_SCROLL_FOOTER);

        let bottom = notes_view(&sections, last + 5, rows);
        assert_eq!(texts(&bottom), ["12 octobre 2026", "- Ancienne"]);
    }

    #[test]
    fn test_badge_card_names_the_badge_and_its_secret_condition() {
        let rain = badge_card_view(badges::find("rain").unwrap());
        assert_eq!(rain.frame, Frame::Stars);
        assert_eq!(centered(&rain)[1], "/ (___) \\");
        assert_eq!(centered(&rain)[5..], ["Nouveau badge\u{202F}!", "Pluie"]);

        let patience = badge_card_view(badges::find("patience").unwrap());
        assert_eq!(
            centered(&patience)[5..],
            [
                "Nouveau badge\u{202F}!",
                "Patience",
                "weathr ouvert 1 h d'affilée"
            ]
        );
    }
}
