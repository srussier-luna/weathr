use crate::holiday::Holiday;
use crate::render::TerminalRenderer;
use crate::scene::tree::Tree;
use crossterm::style::Color;
use std::io;

const MAILBOX_WIDTH: u16 = 7;
const MAILBOX_DAY: Color = Color::Rgb {
    r: 255,
    g: 204,
    b: 0,
};
const MAILBOX_NIGHT: Color = Color::Rgb {
    r: 150,
    g: 120,
    b: 0,
};

#[derive(Clone, Copy)]
pub struct GardenPlan {
    pub tree: Option<Tree>,
    pub mailbox_x: Option<u16>,
}

impl GardenPlan {
    pub fn new(house_x: u16, horizon_y: u16) -> Self {
        Self {
            tree: Tree::fit(house_x, horizon_y),
            mailbox_x: house_x.checked_sub(MAILBOX_WIDTH),
        }
    }
}

#[derive(Default)]
pub struct Decorations;

pub struct DecorationRenderConfig {
    pub horizon_y: u16,
    pub house_x: u16,
    pub house_width: u16,
    pub width: u16,
    pub is_day: bool,
    pub is_snowing: bool,
    pub holiday: Holiday,
}

impl Decorations {
    pub fn new() -> Self {
        Self
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        config: &DecorationRenderConfig,
    ) -> io::Result<()> {
        let plan = GardenPlan::new(config.house_x, config.horizon_y);

        // Render Tree (Left of house)
        if let Some(tree) = plan.tree {
            tree.render(renderer, config.is_day, &config.holiday, config.is_snowing)?;
        }

        // Render Fence (Right of house)
        let (fence_lines, fence_color) = self.get_fence(config.is_day);
        let fence_height = fence_lines.len() as u16;
        let fence_y = config.horizon_y.saturating_sub(fence_height); // Sitting on ground
        let fence_x = config.house_x + config.house_width + 2; // Slight gap

        if fence_x < config.width {
            for (i, line) in fence_lines.iter().enumerate() {
                for (j, ch) in line.chars().enumerate() {
                    if ch != ' ' {
                        renderer.render_char(
                            fence_x + j as u16,
                            fence_y + i as u16,
                            ch,
                            fence_color,
                        )?;
                    }
                }
            }
        }

        // Render Mailbox (On ground top level, left of tree)
        let (mailbox_lines, mailbox_color) = self.get_mailbox(config.is_day);
        let mailbox_height = mailbox_lines.len() as u16;
        let mailbox_y = config.horizon_y.saturating_sub(mailbox_height); // On ground top

        if let Some(mailbox_x) = plan.mailbox_x.filter(|x| *x < config.width) {
            for (i, line) in mailbox_lines.iter().enumerate() {
                for (j, ch) in line.chars().enumerate() {
                    if ch != ' ' {
                        renderer.render_char(
                            mailbox_x + j as u16,
                            mailbox_y + i as u16,
                            ch,
                            mailbox_color,
                        )?;
                    }
                }
            }
        }

        // Render Second Tree (Right of house, only if terminal is wide enough)
        if config.width > 120 {
            let (pine_lines, pine_color) = self.get_pine_tree(config.is_day);
            let pine_height = pine_lines.len() as u16;
            let pine_x = config.house_x + config.house_width + 18; // 18 chars right of house
            let pine_y = config.horizon_y.saturating_sub(pine_height);

            if pine_x + 10 < config.width {
                // Check if full tree fits
                for (i, line) in pine_lines.iter().enumerate() {
                    for (j, ch) in line.chars().enumerate() {
                        if ch != ' ' {
                            renderer.render_char(
                                pine_x + j as u16,
                                pine_y + i as u16,
                                ch,
                                pine_color,
                            )?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn get_fence(&self, is_day: bool) -> (Vec<&'static str>, Color) {
        (
            vec!["|--|--|--|--|", "|  |  |  |  |"],
            if is_day { Color::White } else { Color::Grey },
        )
    }

    fn get_mailbox(&self, is_day: bool) -> (Vec<&'static str>, Color) {
        (
            vec![" .--. ", "|____|", "  ||  "],
            if is_day { MAILBOX_DAY } else { MAILBOX_NIGHT },
        )
    }

    fn get_pine_tree(&self, is_day: bool) -> (Vec<&'static str>, Color) {
        (
            vec![
                "    *    ",
                "   ***   ",
                "  *****  ",
                " ******* ",
                "   |||   ",
            ],
            if is_day {
                Color::DarkGreen
            } else {
                Color::Rgb { r: 0, g: 50, b: 0 }
            },
        )
    }
}
