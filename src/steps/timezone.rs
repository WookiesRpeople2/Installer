use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::{
        StepTrait,
        constants::{SCROLL_LIST_CARD_FOOTER, TZ_C_NAME, TZ_F_C, TZ_F_H, TZ_T_FOOTER, TZ_T_INTRO},
    },
    utils::key::handle_on_key_menu_scroll_list,
};

pub struct Timezone;

impl StepTrait for Timezone {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu_scroll_list(&mut app_state.state.timezone, key)
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(TZ_C_NAME)
            .footer(SCROLL_LIST_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new().field_height(TZ_F_H).split(inner, TZ_F_C);

        Text::new().content(TZ_T_INTRO).render(areas.intro, buf);
        app_state.state.timezone.render(areas.fields[0], buf);
        Text::new().content(TZ_T_FOOTER).render(areas.footer, buf);
    }
}
