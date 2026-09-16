use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Padding, Widget},
};

use crate::components::{
    background::Background,
    constants::{
        self, CARD_DEFAULT_BRAND, CARD_DEFAULT_FOOTER, CARD_MARGIN_BOTH_DIRECTIONS, CARD_MARGIN_X,
        CARD_MARGIN_Y, CARD_MAX_H, CARD_MAX_W,
    },
};

pub struct Card {
    brand: String,
    section: String,
    footer: String,
}

impl Card {
    pub fn new(section: impl Into<String>) -> Self {
        Self {
            brand: CARD_DEFAULT_BRAND.to_string(),
            section: section.into(),
            footer: CARD_DEFAULT_FOOTER.to_string(),
        }
    }

    pub fn brand(mut self, brand: impl Into<String>) -> Self {
        self.brand = brand.into();
        self
    }

    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = footer.into();
        self
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) -> Rect {
        Background.render(area, buf);

        let width = area
            .width
            .saturating_sub(CARD_MARGIN_X * CARD_MARGIN_BOTH_DIRECTIONS)
            .max(CARD_MAX_W);
        let height = area
            .height
            .saturating_sub(CARD_MARGIN_Y * CARD_MARGIN_BOTH_DIRECTIONS)
            .max(CARD_MAX_H);
        let card_area = centered_rect(area, width, height);
        Clear.render(card_area, buf);

        let block = Block::bordered()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(constants::BORDER))
            .padding(Padding::uniform(1))
            .title(Line::from(vec![
                Span::styled(
                    format!(" {} ", self.brand),
                    Style::default().fg(constants::ACCENT).bold(),
                ),
                Span::styled(self.section.as_str(), Style::default().fg(constants::MUTED)),
            ]))
            .title_bottom(
                Line::from(format!(" {} ", self.footer))
                    .fg(constants::FAINT)
                    .centered(),
            );

        let inner = block.inner(card_area);
        block.render(card_area, buf);
        inner
    }
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let [area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}
