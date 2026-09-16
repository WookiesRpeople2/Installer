use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::{
        StepTrait,
        constants::{
            KEY_C_NAME, KEY_F_C, KEY_F_H, KEY_T_FOOTER, KEY_T_INTRO, SCROLL_LIST_CARD_FOOTER,
        },
    },
    utils::key::handle_on_key_menu_scroll_list,
};

pub struct Keymap;

impl StepTrait for Keymap {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu_scroll_list(&mut app_state.state.keymap, key)
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(KEY_C_NAME)
            .footer(SCROLL_LIST_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new().field_height(KEY_F_H).split(inner, KEY_F_C);

        Text::new().content(KEY_T_INTRO).render(areas.intro, buf);
        app_state.state.keymap.render(areas.fields[0], buf);
        Text::new().content(KEY_T_FOOTER).render(areas.footer, buf);
    }
}
