use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::{
        StepTrait,
        constants::{
            LOCAL_C_NAME, LOCAL_F_C, LOCAL_F_H, LOCAL_T_FOOTER, LOCAL_T_INTRO,
            SCROLL_LIST_CARD_FOOTER,
        },
    },
    utils::key::handle_on_key_menu_scroll_list,
};

pub struct Locale;

impl StepTrait for Locale {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu_scroll_list(&mut app_state.state.locale, key)
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(LOCAL_C_NAME)
            .footer(SCROLL_LIST_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new()
            .field_height(LOCAL_F_H)
            .split(inner, LOCAL_F_C);

        Text::new().content(LOCAL_T_INTRO).render(areas.intro, buf);
        app_state.state.locale.render(areas.fields[0], buf);
        Text::new()
            .content(LOCAL_T_FOOTER)
            .render(areas.footer, buf);
    }
}
