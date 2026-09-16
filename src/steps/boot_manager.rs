use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::{
        StepTrait,
        constants::{
            BMGR_C_NAME, BMGR_F_C, BMGR_F_H, BMGR_T_FOOTER, BMGR_T_INTRO, SCROLL_LIST_CARD_FOOTER,
        },
    },
    utils::key::handle_on_key_menu_scroll_list,
};

pub struct BootManger;

impl StepTrait for BootManger {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu_scroll_list(&mut app_state.state.boot_managers, key)
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(BMGR_C_NAME)
            .footer(SCROLL_LIST_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new()
            .field_height(BMGR_F_H)
            .split(inner, BMGR_F_C);

        Text::new().content(BMGR_T_INTRO).render(areas.intro, buf);
        app_state.state.boot_managers.render(areas.fields[0], buf);
        Text::new().content(BMGR_T_FOOTER).render(areas.footer, buf);
    }
}
