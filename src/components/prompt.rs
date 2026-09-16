use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Flex;
use ratatui::prelude::Stylize;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::components::constants::{ERROR, PROMPT_DEFAULT_INVALID};
use crate::components::{
    InputComponent,
    constants::{
        ACCENT, FAINT, FG, MUTED, PROMPT_CURSOR, PROMPT_DEFAULT_FOCUSED, PROMPT_DEFAULT_MASK,
        PROMPT_DEFAULT_MAX_LEN, PROMPT_SATURATING, PROPMT_PADDING,
    },
};

#[derive(Debug, Clone)]
pub struct Prompt {
    label: String,
    placeholder: String,
    value: String,
    max_len: usize,
    mask: bool,
    focused: bool,
    invalid: bool,
    filter: fn(char) -> bool,
}

impl Prompt {
    pub fn new(label: impl Into<String>, placeholder: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            placeholder: placeholder.into(),
            value: String::new(),
            max_len: PROMPT_DEFAULT_MAX_LEN,
            mask: PROMPT_DEFAULT_MASK,
            focused: PROMPT_DEFAULT_FOCUSED,
            invalid: PROMPT_DEFAULT_INVALID,
            filter: |c| c.is_ascii_alphanumeric() || c == '-' || c == '_',
        }
    }

    pub fn max_len(mut self, max_len: usize) -> Self {
        self.max_len = max_len;
        self
    }

    pub fn mask(mut self, mask: bool) -> Self {
        self.mask = mask;
        if mask {
            self.filter = |_| true;
        }
        self
    }

    pub fn filter(mut self, filter: fn(char) -> bool) -> Self {
        self.filter = filter;
        self
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_empty(&self) -> bool {
        self.value.trim().is_empty()
    }

    pub fn take_trimmed(&mut self) -> String {
        self.value = self.value.trim().to_string();
        self.value.clone()
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn set_invalid(&mut self, invalid: bool) {
        self.invalid = invalid;
    }
    pub fn clear_invalid(&mut self) {
        self.invalid = false;
    }

    fn preferred_width(&self) -> u16 {
        let content = self
            .max_len
            .max(self.placeholder.chars().count())
            .max(self.label.chars().count());

        u16::try_from(content.saturating_add(PROMPT_SATURATING)).unwrap_or(u16::MAX) // borders + padding + cursor
    }
}

impl InputComponent for Prompt {
    fn on_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Enter => !self.is_empty(),
            KeyCode::Backspace => {
                self.value.pop();
                false
            }
            KeyCode::Char(c) if !c.is_control() => {
                if self.value.len() < self.max_len && (self.filter)(c) {
                    self.value.push(c);
                }
                false
            }
            _ => false,
        }
    }
}

impl Widget for &Prompt {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [column] = Layout::horizontal([Constraint::Length(self.preferred_width())])
            .flex(Flex::Center)
            .areas(area);

        let chunks = Layout::vertical([Constraint::Length(1), Constraint::Length(3)]).split(column);
        let cursor = Span::styled(
            if self.focused { PROMPT_CURSOR } else { " " },
            Style::default().fg(ACCENT),
        );
        let border_style = Style::default().fg(if self.invalid {
            ERROR
        } else if self.focused {
            ACCENT
        } else {
            MUTED
        });

        Paragraph::new(self.label.as_str())
            .fg(MUTED)
            .render(chunks[0], buf);

        let shown = if self.mask {
            "•".repeat(self.value.chars().count())
        } else {
            self.value.clone()
        };

        let input = if self.value.is_empty() {
            Line::from(vec![
                cursor,
                Span::styled(format!(" {}", self.placeholder), Style::default().fg(FAINT)),
            ])
        } else {
            Line::from(vec![Span::raw(shown), cursor])
        };

        Paragraph::new(input)
            .block(
                Block::bordered()
                    .border_style(border_style)
                    .padding(Padding::horizontal(PROPMT_PADDING)),
            )
            .fg(FG)
            .render(chunks[1], buf);
    }
}
