use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, gauge::Gauge, text::Text},
    layout::verical::Vertical,
    steps::{
        StepTrait,
        constants::{GAUGE_CARD_FOOTER, INST_C_NAME, INST_F_C, INST_F_H, INST_T_INTRO},
    },
};

pub struct Installing;

impl StepTrait for Installing {
    fn on_key(&self, _state: &mut AppState, _key: KeyEvent) -> Transition {
        Transition::None
    }

    fn render(&self, state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(INST_C_NAME)
            .footer(GAUGE_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new()
            .field_height(INST_F_H)
            .split(inner, INST_F_C);

        Text::new()
            .content(INST_T_INTRO.into())
            .render(areas.intro, buf);

        Gauge::new(state.install_ratio, state.install_label.as_str()).render(areas.fields[0], buf);

        Text::new()
            .content(
                &state
                    .install_error
                    .clone()
                    .unwrap_or_else(|| state.install_label.clone()),
            )
            .render(areas.footer, buf);
    }
}
