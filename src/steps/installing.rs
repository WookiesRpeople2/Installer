use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, gauge::Gauge, text::Text},
    layout::verical::Vertical,
    steps::StepTrait,
};

pub struct Installing;

impl StepTrait for Installing {
    fn on_key(&self, _state: &mut AppState, _key: KeyEvent) -> Transition {
        Transition::None
    }

    fn render(&self, state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new("Installing")
            .footer("Please wait…")
            .render(area, buf);
        let areas = Vertical::new().field_height(3).split(inner, 1);

        Text::new()
            .content("Installing Modular Linux".into())
            .render(areas.intro, buf);

        Gauge::new(state.install_ratio, state.install_label.as_str()).render(areas.fields[0], buf);

        let footer = if let Some(err) = &state.install_error {
            err.clone()
        } else {
            state.install_label.clone()
        };
        Text::new().content(footer).render(areas.footer, buf);
    }
}
