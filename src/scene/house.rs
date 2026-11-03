use crate::render::TerminalRenderer;
use crate::scene::tree::Tree;
use chrono::{Local, Timelike};
use crossterm::style::Color;
use std::io;

const WOOD_COLOR: Color = Color::Rgb {
    r: 210,
    g: 180,
    b: 140,
};
const DOOR_COLOR: Color = Color::Rgb {
    r: 139,
    g: 69,
    b: 19,
};
const EVENING_START_MINUTES: u32 = 18 * 60;
const LIGHTS_OUT_AFTER_EVENING: u32 = 5 * 60;
const LIGHTS_OUT_SPREAD: u32 = 90;
const MORNING_AFTER_EVENING: u32 = 12 * 60 + 30;
const LARGE_MIN_WIDTH: u16 = 170;
const LARGE_MIN_HEIGHT: u16 = 32;
const LEFT_SHIFT_DIVISOR: u16 = 20;

const SMALL_ASCII: [&str; 13] = [
    "                             ",
    "                             ",
    "                             ",
    "            _   _._          ",
    "           |_|-'_~_`-._      ",
    "        _.-'-_~_-~_-~-_`-._  ",
    "    _.-'_~-_~-_-~-_~_~-_~-_`-._",
    "   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
    "     |  []  []   []   []  [] |",
    "     |           __    ___   |",
    "   ._|  []  []  | .|  [___]  |_._._._._._._._._._._._._._._._._.",
    "   |=|________()|__|()_______|=|=|=|=|=|=|=|=|=|=|=|=|=|=|=|=|=|",
    " ^^^^^^^^^^^^^^^ === ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^",
];

const LARGE_ASCII: [&str; 15] = [
    "",
    "                         _._",
    "                    _.-'_~-_~`-._",
    "             _  _.-'_~-_~-_-~-_~_`-._",
    "            |_|-'_~-_~-_-~-_~_~-_~-_~`-._",
    "        _.-'_~-_~-_-~-_~_~-_~-_~_-~-_~-_~`-._",
    "    _.-'_~-_~-_-~-_~_~-_~-_~_-~-_~-_~_~-_-~-_`-._",
    "   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
    "     |  []   []   []   []   []   []   []   []  |",
    "     |                                         |",
    "     |-----------------------------------------|",
    "     |  []   []   []     __     []    ___  []  |",
    "   ._|  []   []   []    | .|    []   [___] []  |_._._._._._._._._._._._._._._._._._._.",
    "   |=|________________()|__|()_________________|=|=|=|=|=|=|=|=|=|=|=|=|=|=|=|=|=|=|=|",
    " ^^^^^^^^^^^^^^^^^^^^^^^ === ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^",
];

#[derive(Clone, Copy)]
enum RowKind {
    Smoke,
    Roof,
    Eave,
    Wall,
    Base,
    Grass,
}

const SMALL_ROWS: [RowKind; 13] = [
    RowKind::Smoke,
    RowKind::Smoke,
    RowKind::Smoke,
    RowKind::Smoke,
    RowKind::Roof,
    RowKind::Roof,
    RowKind::Roof,
    RowKind::Eave,
    RowKind::Wall,
    RowKind::Wall,
    RowKind::Wall,
    RowKind::Base,
    RowKind::Grass,
];

const LARGE_ROWS: [RowKind; 15] = [
    RowKind::Roof,
    RowKind::Roof,
    RowKind::Roof,
    RowKind::Roof,
    RowKind::Roof,
    RowKind::Roof,
    RowKind::Roof,
    RowKind::Eave,
    RowKind::Wall,
    RowKind::Wall,
    RowKind::Wall,
    RowKind::Wall,
    RowKind::Wall,
    RowKind::Base,
    RowKind::Grass,
];

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum HouseSize {
    #[default]
    Small,
    Large,
}

#[derive(Clone, Copy, Default)]
pub struct House {
    size: HouseSize,
}

impl House {
    pub fn for_terminal(width: u16, height: u16) -> Self {
        let size = if width >= LARGE_MIN_WIDTH && height >= LARGE_MIN_HEIGHT {
            HouseSize::Large
        } else {
            HouseSize::Small
        };
        Self { size }
    }

    pub fn x_on(&self, terminal_width: u16, horizon_y: u16) -> u16 {
        let centered = (terminal_width / 2).saturating_sub(self.width() / 2);
        let room = Tree::fit(centered, horizon_y).map_or(0, |tree| tree.x - 1);
        centered - (terminal_width / LEFT_SHIFT_DIVISOR).min(room)
    }

    pub fn height(&self) -> u16 {
        self.get_ascii().len() as u16
    }

    pub fn width(&self) -> u16 {
        match self.size {
            HouseSize::Small => 64,
            HouseSize::Large => 86,
        }
    }

    pub fn chimney_offset(&self) -> (u16, u16) {
        match self.size {
            HouseSize::Small => (12, 2),
            HouseSize::Large => (13, 2),
        }
    }

    pub fn roof_perch(&self) -> (u16, u16) {
        match self.size {
            HouseSize::Small => (23, 3),
            HouseSize::Large => (41, 3),
        }
    }

    pub fn big_window(&self) -> (u16, u16) {
        match self.size {
            HouseSize::Small => (23, 9),
            HouseSize::Large => (38, 11),
        }
    }

    pub fn eave_row(&self) -> u16 {
        self.row_kinds()
            .iter()
            .position(|kind| matches!(kind, RowKind::Eave))
            .unwrap_or_default() as u16
    }

    pub fn windows(&self) -> Vec<(u16, u16)> {
        self.get_ascii()
            .iter()
            .zip(self.row_kinds())
            .enumerate()
            .filter(|(_, (_, kind))| matches!(kind, RowKind::Wall))
            .flat_map(|(row, (line, _))| {
                line.match_indices('[').filter_map(move |(start, _)| {
                    let end = start + line[start..].find(']')?;
                    Some((((start + end) / 2) as u16, row as u16))
                })
            })
            .collect()
    }

    pub fn peak(&self) -> (u16, u16) {
        match self.size {
            HouseSize::Small => (17, 3),
            HouseSize::Large => (26, 1),
        }
    }

    pub fn porch_rail_row(&self) -> u16 {
        match self.size {
            HouseSize::Small => 10,
            HouseSize::Large => 12,
        }
    }

    pub fn porch_width(&self) -> u16 {
        match self.size {
            HouseSize::Small => 34,
            HouseSize::Large => 38,
        }
    }

    pub fn door_x(&self) -> u16 {
        match self.size {
            HouseSize::Small => 16,
            HouseSize::Large => 24,
        }
    }

    pub fn porch_x(&self) -> u16 {
        match self.size {
            HouseSize::Small => 30,
            HouseSize::Large => 48,
        }
    }

    pub fn get_ascii(&self) -> &'static [&'static str] {
        match self.size {
            HouseSize::Small => &SMALL_ASCII,
            HouseSize::Large => &LARGE_ASCII,
        }
    }

    fn row_kinds(&self) -> &'static [RowKind] {
        match self.size {
            HouseSize::Small => &SMALL_ROWS,
            HouseSize::Large => &LARGE_ROWS,
        }
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        x: u16,
        y: u16,
        is_day: bool,
    ) -> io::Result<()> {
        let wood_color = if is_day {
            WOOD_COLOR
        } else {
            Color::Rgb {
                r: 100,
                g: 70,
                b: 50,
            }
        };
        let roof_color = if is_day {
            Color::DarkRed
        } else {
            Color::DarkMagenta
        };
        let now = Local::now();
        let minutes_since_evening =
            (now.hour() * 60 + now.minute() + 24 * 60 - EVENING_START_MINUTES) % (24 * 60);
        let grass_color = if is_day {
            Color::Green
        } else {
            Color::DarkGreen
        };

        for (i, (line, kind)) in self.get_ascii().iter().zip(self.row_kinds()).enumerate() {
            let row = y + i as u16;

            if let RowKind::Eave = kind {
                renderer.render_line_colored(x, row, line, roof_color)?;
                continue;
            }

            let is_solid = matches!(kind, RowKind::Wall | RowKind::Base);
            let first = line.find(|ch| ch != ' ').unwrap_or(line.len());
            let last = line.rfind(|ch| ch != ' ').unwrap_or(0);
            let mut window_start = 0;
            for (j, ch) in line.chars().enumerate() {
                if ch == '[' {
                    window_start = j;
                }
                if ch == ' ' {
                    if is_solid && (first..=last).contains(&j) {
                        renderer.render_char(x + j as u16, row, ' ', Color::Reset)?;
                    }
                    continue;
                }
                let color = match kind {
                    RowKind::Smoke => match ch {
                        '(' | ')' | '_' => Color::DarkGrey,
                        _ => Color::Grey,
                    },
                    RowKind::Roof => match ch {
                        '(' | ')' => Color::DarkGrey,
                        _ => roof_color,
                    },
                    RowKind::Eave => roof_color,
                    RowKind::Wall => match ch {
                        '[' | ']' => window_color(is_day, i, window_start, minutes_since_evening),
                        '(' | ')' => DOOR_COLOR,
                        '=' => Color::DarkGrey,
                        _ => wood_color,
                    },
                    RowKind::Base => match ch {
                        '=' | '|' => Color::DarkGrey,
                        '(' | ')' => DOOR_COLOR,
                        _ => wood_color,
                    },
                    RowKind::Grass => match ch {
                        '^' => grass_color,
                        '=' => Color::DarkGrey,
                        _ => Color::Reset,
                    },
                };
                renderer.render_char(x + j as u16, row, ch, color)?;
            }
        }
        Ok(())
    }
}

fn window_color(is_day: bool, row: usize, column: usize, minutes_since_evening: u32) -> Color {
    if is_day {
        return Color::Cyan;
    }

    let lights_out =
        LIGHTS_OUT_AFTER_EVENING + (row as u32 * 37 + column as u32 * 53) % LIGHTS_OUT_SPREAD;
    if minutes_since_evening < lights_out || minutes_since_evening >= MORNING_AFTER_EVENING {
        Color::Yellow
    } else {
        Color::DarkGrey
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_house_windows() {
        let house = House::for_terminal(80, 24);
        assert_eq!(house.eave_row(), 7);
        assert_eq!(
            house.windows(),
            vec![
                (8, 8),
                (12, 8),
                (17, 8),
                (22, 8),
                (26, 8),
                (8, 10),
                (12, 10),
                (24, 10),
            ]
        );
    }

    #[test]
    fn test_large_house_windows_sit_in_the_walls() {
        let house = House::for_terminal(200, 40);
        let ascii = house.get_ascii();
        assert_eq!(house.eave_row(), 7);
        assert_eq!(house.windows().len(), 19);
        for (x, y) in house.windows() {
            assert!(y > house.eave_row());
            let row = ascii[y as usize];
            assert!(matches!(&row[x as usize..=x as usize], "[" | "_"));
        }
    }
}
