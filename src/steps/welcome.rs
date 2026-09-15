use ansi_to_tui::IntoText;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Paragraph, Widget},
};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    steps::StepTrait,
};

pub struct Welcome;

impl StepTrait for Welcome {
    fn on_key(&self, _state: &mut AppState, key: KeyEvent) -> Transition {
        if key.code == KeyCode::Enter {
            Transition::Next
        } else {
            Transition::None
        }
    }

    fn render(&self, _state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new("Installer").render(area, buf);

        let [logo_slot, footer] =
            Layout::vertical([Constraint::Min(10), Constraint::Length(1)]).areas(inner);

        let logo = include_str!("../assets/logo.txt")
            .into_text()
            .unwrap_or_default();
        let logo_width = logo.width() as u16;
        let logo_height = logo.height() as u16;

        let [centered] = Layout::horizontal([Constraint::Length(logo_width.min(logo_slot.width))])
            .flex(Flex::Center)
            .areas(logo_slot);
        let [centered] = Layout::vertical([Constraint::Length(logo_height.min(logo_slot.height))])
            .flex(Flex::Center)
            .areas(centered);

        Paragraph::new(logo).render(centered, buf);

        Text::new()
            .content("Made by WookiesRpeople2".to_string())
            .render(footer, buf);
    }
}
