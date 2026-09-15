use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::StepTrait,
    utils::key::handle_on_key_menu_scroll_list,
};

pub struct Locale;

impl StepTrait for Locale {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu_scroll_list(&mut app_state.state.locale, key)
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new("Locale")
            .footer("↑↓ scroll · Enter select")
            .render(area, buf);
        let areas = Vertical::new().field_height(12).split(inner, 1);

        Text::new()
            .content("Select system language / locale".into())
            .render(areas.intro, buf);
        app_state.state.locale.render(areas.fields[0], buf);
        Text::new()
            .content("Used for dates, numbers, messages".into())
            .render(areas.footer, buf);
    }
}
