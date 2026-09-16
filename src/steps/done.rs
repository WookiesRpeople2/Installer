use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::{
        StepTrait,
        constants::{
            DONE_C_NAME, DONE_F_C, DONE_F_H, DONE_T_FOOTER, DONE_T_INTRO, PROMPT_CARD_FOOTER,
        },
    },
    utils::cmd::reboot,
};

pub struct Done;

impl StepTrait for Done {
    fn on_key(&self, _state: &mut AppState, key: KeyEvent) -> Transition {
        match key.code {
            KeyCode::Enter => {
                let _ = reboot();
                Transition::Quit
            }
            _ => Transition::None,
        }
    }

    fn render(&self, _state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(DONE_C_NAME)
            .footer(PROMPT_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new()
            .field_height(DONE_F_H)
            .split(inner, DONE_F_C);

        Text::new().content(DONE_T_INTRO).render(areas.intro, buf);
        Text::new().content(DONE_T_FOOTER).render(areas.footer, buf);
    }
}
