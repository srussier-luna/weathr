use crate::animation::lore::Unlocks;
use crate::badges::Badge;
use crate::overlay::{self, Frame, Line, View};
use crate::progress::Progress;
use crate::release_notes::Section;
use crate::story::Entry;
use crate::story::content::chronicles::{
    BOTTLE_MESSAGES, CHEST_SOUVENIRS, DRAWINGS, OBJECTS, RECIPES,
};
use crate::story::content::letters::CHOICES;
use crate::story::content::{self, MONTHS, ReadContext, postcard_lines, postcard_title, text};
use chrono::Datelike;

const INDEX_FOOTER: &str = "[↑] [↓] choisir  [Entrée] relire  [l] ou [Echap] fermer";
const ALBUM_FOOTER: &str = "[<] [>] pages  [a] ou [Echap] fermer";
const BOTTLE_FOOTER: &str = "[Echap] fermer";
const BADGE_CARD_FOOTER: &str = "[Entrée] ou [Echap] fermer";
const HELP_FOOTER: &str = "[h] ou [Echap] fermer";
const NOTES_FOOTER: &str = "[Entrée] ou [Echap] fermer";
const SCROLL_HINT: &str = "[↑] [↓] défiler";
const HELP_KEY_WIDTH: usize = 10;
const NOTES_BULLET: &str = "- ";
const NOTES_INDENT: &str = "  ";
pub const NOTES_FRAME: Frame = Frame::Plain;

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
        frame: Frame::Plain,
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
        NOTES_FOOTER,
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
    use crate::story::content::save_golden::{SavedGame, courrier_fixture, render};

    #[test]
    fn test_save_lists_in_courrier_as_before_refactor() {
        let save = SavedGame::load();
        let opened = save.progress.letters_read.min(save.available - 1);
        let read = save
            .timeline
            .first_mail(save.progress.letters_read.min(save.available));

        let mut out = format!("opens: {opened}\n");
        out.push_str(&render(&index_view(read, 0, 200, &save.context(12))));
        assert_eq!(out, courrier_fixture());
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
        assert_eq!(view.footer, NOTES_FOOTER);
    }

    #[test]
    fn test_long_notes_wrap_in_the_box_and_scroll_to_the_last_line() {
        let sections = crate::release_notes::parse(
            "## 2026-10-26\n- Les Perséides font pleuvoir les étoiles filantes la nuit\n",
        );
        let (height, width) = (3, 20);

        assert_eq!(notes_view(&sections, 0, 20, width).footer, NOTES_FOOTER);
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
