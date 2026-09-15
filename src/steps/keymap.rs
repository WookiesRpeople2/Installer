use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::StepTrait,
    utils::key::handle_on_key_menu_scroll_list,
};

pub struct Keymap;

impl StepTrait for Keymap {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu_scroll_list(&mut app_state.state.keymap, key)
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new("Keyboard")
            .footer("↑↓ scroll · Enter select")
            .render(area, buf);
        let areas = Vertical::new().field_height(10).split(inner, 1);

        Text::new()
            .content("Select keyboard layout".into())
            .render(areas.intro, buf);
        app_state.state.keymap.render(areas.fields[0], buf);
        Text::new()
            .content("Written to /etc/vconsole.conf".into())
            .render(areas.footer, buf);
    }
}
