use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

use crate::components::{InputComponent, constants};

#[derive(Debug, Clone)]
pub struct ScrollList {
    label: String,
    items: Vec<String>,
    state: ListState,
}

impl ScrollList {
    pub fn new(label: impl Into<String>, items: Vec<String>) -> Self {
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        Self {
            label: label.into(),
            items,
            state,
        }
    }

    pub fn selected(&self) -> Option<&str> {
        self.state
            .selected()
            .and_then(|i| self.items.get(i).map(String::as_str))
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn preferred_width(&self) -> u16 {
        let longest = self
            .items
            .iter()
            .map(|item| item.chars().count())
            .chain(std::iter::once(self.label.chars().count()))
            .max()
            .unwrap_or(0);

        u16::try_from(longest.saturating_add(6)).unwrap_or(u16::MAX)
    }

    fn preferred_list_height(&self) -> u16 {
        u16::try_from(self.items.len().saturating_add(2)).unwrap_or(u16::MAX)
    }
}

impl InputComponent for ScrollList {
    fn on_key(&mut self, key: KeyEvent) -> bool {
        if self.items.is_empty() {
            return false;
        }
        let len = self.items.len();
        let i = self.state.selected().unwrap_or(0);

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.state
                    .select(Some(if i == 0 { len - 1 } else { i - 1 }));
                false
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.state.select(Some((i + 1) % len));
                false
            }
            KeyCode::PageUp => {
                self.state.select(Some(i.saturating_sub(5)));
                false
            }
            KeyCode::PageDown => {
                self.state.select(Some((i + 5).min(len - 1)));
                false
            }
            KeyCode::Enter => true,
            _ => false,
        }
    }
}

impl Widget for &mut ScrollList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let list_height = self.preferred_list_height();
        let content_height = list_height.saturating_add(1 + 1);

        let [column] = Layout::horizontal([Constraint::Length(self.preferred_width())])
            .flex(Flex::Center)
            .areas(area);

        let [content] = Layout::vertical([Constraint::Length(content_height)])
            .flex(Flex::Center)
            .areas(column);

        let [label_area, list_area] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)])
            .spacing(1)
            .areas(content);

        Paragraph::new(self.label.as_str())
            .fg(constants::MUTED)
            .centered()
            .render(label_area, buf);

        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let line = if self.state.selected() == Some(i) {
                    Line::from(vec![
                        Span::styled("› ", Style::default().fg(constants::ACCENT)),
                        Span::raw(item),
                    ])
                } else {
                    Line::from(item.as_str())
                };

                ListItem::new(line.centered())
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::bordered()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(constants::BORDER)),
            )
            .highlight_style(
                Style::default()
                    .fg(constants::ACCENT)
                    .add_modifier(Modifier::BOLD),
            );

        StatefulWidget::render(list, list_area, buf, &mut self.state);
    }
}
