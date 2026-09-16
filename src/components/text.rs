use ratatui::{
    layout::Alignment,
    prelude::Stylize,
    style::Color,
    widgets::{Paragraph, Widget},
};

use crate::components::constants::FG;

pub struct Text {
    content: String,
    alignment: Alignment,
    color: Color,
}

impl Text {
    pub fn new() -> Self {
        Self {
            content: "".to_string(),
            alignment: Alignment::Center,
            color: FG,
        }
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = content.into();
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Widget for Text {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.content)
            .alignment(self.alignment)
            .fg(self.color)
            .render(area, buf);
    }
}
