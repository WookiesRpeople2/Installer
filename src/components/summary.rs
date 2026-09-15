use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    prelude::Stylize,
    style::Style,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::components::constants;

pub struct Summary<'a> {
    lines: Vec<&'a str>,
}

impl<'a> Summary<'a> {
    pub fn new(lines: Vec<&'a str>) -> Self {
        Self { lines }
    }

    fn preferred_width(&self) -> u16 {
        let longest = self
            .lines
            .iter()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(0);

        u16::try_from(longest.saturating_add(4)).unwrap_or(u16::MAX) // borders + a little padding
    }
}

impl Widget for Summary<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [column] = Layout::horizontal([Constraint::Length(self.preferred_width())])
            .flex(Flex::Center)
            .areas(area);

        Paragraph::new(self.lines.join("\n"))
            .block(
                Block::bordered()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(constants::BORDER)),
            )
            .fg(constants::FG)
            .left_aligned()
            .render(column, buf);
    }
}
