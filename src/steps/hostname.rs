use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::{
        StepTrait,
        constants::{
            HNAME_C_NAME, HNAME_F_C, HNAME_F_H, HNAME_T_FOOTER, HNAME_T_INTRO, PROMPT_CARD_FOOTER,
        },
    },
    utils::key::handle_on_key_menu_prompt,
};

pub struct Hostname;

impl StepTrait for Hostname {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu_prompt([&mut app_state.state.hostname].as_mut_slice(), key)
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(HNAME_C_NAME)
            .footer(PROMPT_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new()
            .field_height(HNAME_F_H)
            .split(inner, HNAME_F_C);

        Text::new().content(HNAME_T_INTRO).render(areas.intro, buf);
        app_state.state.hostname.render(areas.fields[0], buf);
        Text::new()
            .content(HNAME_T_FOOTER)
            .render(areas.footer, buf);
    }
}
