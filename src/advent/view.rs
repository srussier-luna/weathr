use super::chocolates::DRAWING_WIDTH;
use super::{Cell, DAYS, chocolate};
use crate::overlay::{Frame, Line, View};

pub const COLUMNS: usize = 6;
const CELL_ROWS: usize = 3;
const DOOR: &str = "::::::::";
const GRID_INDENT: &str = "   ";
const CALENDAR_TITLE: &str = "Calendrier de l'Avent";
const CALENDAR_FOOTER: &str = "[←] [→] [↑] [↓] choisir  [v] ou [Echap] fermer";
const CARD_FOOTER: &str = "[Entrée] ou [Echap] fermer";

struct Border {
    corner: char,
    across: char,
    down: char,
}

const BORDER: Border = Border {
    corner: '+',
    across: '-',
    down: '|',
};
const SELECTED_BORDER: Border = Border {
    corner: '#',
    across: '=',
    down: '#',
};

pub fn step(selected: u32, delta: isize) -> u32 {
    let last = DAYS as isize;
    (selected as isize).saturating_add(delta).clamp(1, last) as u32
}

fn door(day: u32) -> [String; CELL_ROWS] {
    [DOOR.to_string(), format!("::{day:^4}::"), DOOR.to_string()]
}

fn content(cell: Cell) -> [String; CELL_ROWS] {
    match cell {
        Cell::Received(chocolate) => chocolate.drawing.map(str::to_string),
        Cell::Missed => Default::default(),
        Cell::Closed(day) => door(day),
    }
}

fn corner_of(day: u32) -> (usize, usize) {
    let index = day.saturating_sub(1) as usize;
    (
        (index % COLUMNS) * (DRAWING_WIDTH + 1),
        (index / COLUMNS) * (CELL_ROWS + 1),
    )
}

fn outline(canvas: &mut [Vec<char>], (x, y): (usize, usize), border: &Border) {
    let (right, bottom) = (x + DRAWING_WIDTH + 1, y + CELL_ROWS + 1);
    for line in [y, bottom] {
        canvas[line][x..=right].fill(border.across);
    }
    for row in canvas.iter_mut().take(bottom + 1).skip(y) {
        row[x] = border.down;
        row[right] = border.down;
    }
    for (column, row) in [(x, y), (right, y), (x, bottom), (right, bottom)] {
        canvas[row][column] = border.corner;
    }
}

pub fn grid(cells: &[Cell], selected: u32) -> Vec<String> {
    let rows = cells.len().div_ceil(COLUMNS);
    let mut canvas = vec![vec![' '; COLUMNS * (DRAWING_WIDTH + 1) + 1]; rows * (CELL_ROWS + 1) + 1];
    for (day, cell) in (1..).zip(cells) {
        let (x, y) = corner_of(day);
        outline(&mut canvas, (x, y), &BORDER);
        for (row, line) in content(*cell).iter().enumerate() {
            for (column, ch) in line.chars().take(DRAWING_WIDTH).enumerate() {
                canvas[y + 1 + row][x + 1 + column] = ch;
            }
        }
    }
    if (1..=cells.len()).contains(&(selected as usize)) {
        outline(&mut canvas, corner_of(selected), &SELECTED_BORDER);
    }
    canvas
        .into_iter()
        .map(|row| row.into_iter().collect())
        .collect()
}

pub fn calendar_view(cells: &[Cell], selected: u32) -> View {
    let mut lines: Vec<Line> = grid(cells, selected)
        .into_iter()
        .map(|line| Line::Preformatted(format!("{GRID_INDENT}{line}")))
        .collect();
    lines.push(Line::Blank);
    lines.push(match cells.get(selected.saturating_sub(1) as usize) {
        Some(Cell::Received(chocolate)) => Line::Centered(chocolate.name.to_string()),
        _ => Line::Blank,
    });
    View {
        title: CALENDAR_TITLE.to_string(),
        lines,
        footer: CALENDAR_FOOTER.to_string(),
        frame: Frame::Plain,
    }
}

pub fn chocolate_card_view(day: u32) -> Option<View> {
    let chocolate = chocolate(day)?;
    let mut lines = vec![Line::Blank];
    lines.extend(
        chocolate
            .drawing
            .iter()
            .map(|line| Line::Centered(line.to_string())),
    );
    lines.extend([
        Line::Blank,
        Line::Centered(format!("Chocolat n°{day}")),
        Line::Centered(chocolate.name.to_string()),
    ]);
    Some(View {
        title: String::new(),
        lines,
        footer: CARD_FOOTER.to_string(),
        frame: Frame::Stars,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent::cells;
    use crate::advent::chocolates::CHOCOLATES;
    use chrono::NaiveDate;
    use std::collections::BTreeSet;

    fn december(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 12, day).unwrap()
    }

    #[test]
    fn test_grid_draws_twenty_four_cells_in_six_columns() {
        let grid = grid(&cells(&BTreeSet::new(), december(1)), 1);
        assert_eq!(grid.len(), 4 * (CELL_ROWS + 1) + 1);
        assert!(grid.iter().all(|line| line.chars().count() == 55));
        assert!(grid.join("\n").contains(":: 24 ::"));
    }

    #[test]
    fn test_grid_shows_received_drawings_empty_past_days_and_closed_doors() {
        let grid = grid(&cells(&BTreeSet::from([1]), december(3)), 3);
        let middle = |day| door(day)[1].clone();
        assert_eq!(
            grid[2],
            format!(
                "|{}|        #{}#{}|{}|{}|",
                CHOCOLATES[0].drawing[1],
                middle(3),
                middle(4),
                middle(5),
                middle(6)
            )
        );
    }

    #[test]
    fn test_selected_cell_is_outlined() {
        let grid = grid(&cells(&BTreeSet::new(), december(1)), 8);
        assert!(grid[4].contains("#========#"));
        assert!(grid[8].contains("#========#"));
        assert_eq!(grid[0].matches('#').count(), 0);
    }

    #[test]
    fn test_step_stays_inside_the_calendar() {
        assert_eq!(step(1, -1), 1);
        assert_eq!(step(3, COLUMNS as isize), 9);
        assert_eq!(step(22, COLUMNS as isize), 24);
        assert_eq!(step(24, 1), 24);
    }

    #[test]
    fn test_calendar_names_the_selected_received_chocolate() {
        let cells = cells(&BTreeSet::from([2]), december(5));
        let view = calendar_view(&cells, 2);
        assert!(
            matches!(view.lines.last(), Some(Line::Centered(name)) if name == CHOCOLATES[1].name)
        );
        let view = calendar_view(&cells, 3);
        assert!(matches!(view.lines.last(), Some(Line::Blank)));
    }

    #[test]
    fn test_chocolate_card_shows_the_drawing_number_and_name() {
        let view = chocolate_card_view(1).unwrap();
        let texts: Vec<&str> = view
            .lines
            .iter()
            .filter_map(|line| match line {
                Line::Centered(text) => Some(text.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(texts[..3], CHOCOLATES[0].drawing);
        assert_eq!(texts[3..], ["Chocolat n°1", CHOCOLATES[0].name]);
        assert!(chocolate_card_view(25).is_none());
    }
}
