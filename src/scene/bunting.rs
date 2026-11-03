use crate::holiday::Holiday;
use crate::render::TerminalRenderer;
use crate::scene::house::House;
use crossterm::style::Color;
use std::io;

const STRING: char = '-';
const PENNANT: char = '▼';
const PENNANT_STEP: usize = 2;
const PENNANT_COLORS: [Color; 5] = [
    Color::Red,
    Color::Yellow,
    Color::Cyan,
    Color::Green,
    Color::Magenta,
];

struct Piece {
    x: u16,
    y: u16,
    ch: char,
    color: Color,
}

fn pieces(holiday: &Holiday, house: &House, house_x: u16, house_y: u16, width: u16) -> Vec<Piece> {
    if !holiday.is_neighbours_party {
        return Vec::new();
    }
    let (row, eave) = house.eave();
    let string_y = house_y + row;
    let roof_left = house_x + eave.start;
    let Some(last) = width.checked_sub(1).filter(|last| *last >= roof_left) else {
        return Vec::new();
    };
    let first_pennant = roof_left + (last - roof_left) % PENNANT_STEP as u16;

    let is_pennant = |x: u16| (x - first_pennant) % PENNANT_STEP as u16 == 0;

    let string = (house_x + eave.end..width)
        .filter(|x| !is_pennant(*x))
        .map(|x| Piece {
            x,
            y: string_y,
            ch: STRING,
            color: Color::Grey,
        });
    let pennants = (first_pennant..width)
        .step_by(PENNANT_STEP)
        .zip(PENNANT_COLORS.iter().cycle())
        .map(|(x, color)| Piece {
            x,
            y: string_y,
            ch: PENNANT,
            color: *color,
        });
    string.chain(pennants).collect()
}

pub fn render(
    renderer: &mut TerminalRenderer,
    holiday: &Holiday,
    house: &House,
    house_x: u16,
    house_y: u16,
    width: u16,
) -> io::Result<()> {
    for piece in pieces(holiday, house, house_x, house_y, width) {
        renderer.render_char(piece.x, piece.y, piece.ch, piece.color)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn party_on(month: u32, day: u32) -> Holiday {
        Holiday::on(NaiveDate::from_ymd_opt(2026, month, day).unwrap())
    }

    #[test]
    fn test_bunting_runs_from_the_roof_to_the_right_edge() {
        for (width, height) in [(80, 24), (121, 40), (200, 50)] {
            let house = House::for_terminal(width, height);
            let house_x = house.x_on(width, height - 7);
            let (row, eave) = house.eave();
            let pieces = pieces(&party_on(5, 29), &house, house_x, 10, width);

            let eave_line = house.get_ascii()[row as usize];
            let roof_left = house_x + eave.start;
            assert!(pieces.iter().all(|piece| piece.y == 10 + row));
            let pennants: Vec<&Piece> = pieces.iter().filter(|piece| piece.ch == PENNANT).collect();
            assert!(pennants[0].x - roof_left < PENNANT_STEP as u16);
            assert_eq!(pennants.last().unwrap().x, width - 1, "{width}x{height}");
            assert!(
                pennants
                    .windows(2)
                    .all(|pair| pair[1].x - pair[0].x == PENNANT_STEP as u16
                        && pair[1].color != pair[0].color)
            );
            for x in roof_left..width {
                let drawn = pieces
                    .iter()
                    .find(|piece| piece.x == x)
                    .map(|piece| piece.ch);
                let under = if x < house_x + eave.end {
                    eave_line.chars().nth((x - house_x) as usize)
                } else {
                    None
                };
                match drawn {
                    Some(PENNANT) => {}
                    Some(ch) => assert!(ch == STRING && x >= house_x + eave.end),
                    None => assert_eq!(under, Some('~'), "{width}x{height} at {x}"),
                }
            }
        }
    }

    #[test]
    fn test_no_bunting_around_the_party() {
        let house = House::for_terminal(121, 40);
        for day in [28, 30] {
            assert!(pieces(&party_on(5, day), &house, 20, 10, 121).is_empty());
        }
    }
}
