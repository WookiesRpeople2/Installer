use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::{
        StepTrait,
        constants::{
            PASS_C_NAME, PASS_F_C, PASS_F_H, PASS_T_FOOTER, PASS_T_INTRO, PROMPT_CARD_FOOTER,
        },
    },
    utils::key::handle_on_key_menu_prompt,
};

pub struct Password;

impl StepTrait for Password {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu_prompt(
            [
                &mut app_state.state.user_password,
                &mut app_state.state.root_password,
            ]
            .as_mut_slice(),
            key,
        )
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(PASS_C_NAME)
            .footer(PROMPT_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new()
            .field_height(PASS_F_H)
            .split(inner, PASS_F_C);

        Text::new().content(PASS_T_INTRO).render(areas.intro, buf);
        app_state.state.user_password.render(areas.fields[0], buf);
        app_state.state.root_password.render(areas.fields[1], buf);
        Text::new().content(PASS_T_FOOTER).render(areas.footer, buf);
    }
}
