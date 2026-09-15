use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::StepTrait,
    utils::key::handle_on_key_menu_scroll_list,
};

pub struct Timezone;

impl StepTrait for Timezone {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu_scroll_list(&mut app_state.state.timezone, key)
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new("Timezone")
            .footer("↑↓ scroll · Enter select")
            .render(area, buf);
        let areas = Vertical::new().field_height(10).split(inner, 1);

        Text::new()
            .content("Select your timezone".into())
            .render(areas.intro, buf);
        app_state.state.timezone.render(areas.fields[0], buf);
        Text::new()
            .content("PgUp/PgDn jump faster".into())
            .render(areas.footer, buf);
    }
}
