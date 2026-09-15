use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::StepTrait,
    utils::key::handle_on_key_menu_scroll_list,
};

pub struct BootManger;

impl StepTrait for BootManger {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu_scroll_list(&mut app_state.state.boot_managers, key)
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new("Boot Manager")
            .footer("↑↓ scroll · Enter select")
            .render(area, buf);
        let areas = Vertical::new().field_height(10).split(inner, 1);

        Text::new()
            .content("Select a boot magaer".into())
            .render(areas.intro, buf);
        app_state.state.boot_managers.render(areas.fields[0], buf);
        Text::new()
            .content("this will be used to boot the system".into())
            .render(areas.footer, buf);
    }
}
