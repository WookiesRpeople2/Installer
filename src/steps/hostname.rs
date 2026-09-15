use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::StepTrait,
    utils::key::handle_on_key_menu_prompt,
};

pub struct Hostname;

impl StepTrait for Hostname {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu_prompt([&mut app_state.state.hostname].as_mut_slice(), key)
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new("Hostname").render(area, buf);
        let areas = Vertical::new().field_height(4).split(inner, 1);

        Text::new()
            .content("Name this computer".into())
            .render(areas.intro, buf);
        app_state.state.hostname.render(areas.fields[0], buf);
        Text::new()
            .content("Letters, numbers, hyphens only".into())
            .render(areas.footer, buf);
    }
}
