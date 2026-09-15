use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Widget},
};

use crate::components::constants::{BG, FG};

#[derive(Debug, Clone, Copy, Default)]
pub struct Background;

impl Widget for Background {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        Block::default()
            .style(Style::default().bg(BG).fg(FG))
            .render(area, buf);
    }
}
