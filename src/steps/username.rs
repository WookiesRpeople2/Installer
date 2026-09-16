use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::{
        StepTrait,
        constants::{USER_C_NAME, USER_F_C, USER_F_H, USER_T_FOOTER, USER_T_INTRO},
    },
    utils::key::handle_on_key_menu_prompt,
};

pub struct Username;

impl StepTrait for Username {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu_prompt([&mut app_state.state.username].as_mut_slice(), key)
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(USER_C_NAME).render(area, buf);
        let areas = Vertical::new()
            .field_height(USER_F_H)
            .split(inner, USER_F_C);

        Text::new().content(USER_T_INTRO).render(areas.intro, buf);
        app_state.state.username.render(areas.fields[0], buf);
        Text::new().content(USER_T_FOOTER).render(areas.footer, buf);
    }
}
