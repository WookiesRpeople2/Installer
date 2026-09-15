use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::StepTrait,
    utils::key::handle_on_key_menu_prompt,
};

pub struct Password;

impl StepTrait for Password {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu_prompt(
            [
                &mut app_state.state.user_password,
                &mut app_state.state.root_password,
            ]
            .as_mut_slice(),
            key,
        )
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new("Password").render(area, buf);
        let areas = Vertical::new().field_height(4).split(inner, 3);

        Text::new()
            .content("Set the user and root password".into())
            .render(areas.intro, buf);
        app_state.state.user_password.render(areas.fields[0], buf);
        app_state.state.root_password.render(areas.fields[2], buf);

        Text::new()
            .content("Hidden while typig · Enter next".into())
            .render(areas.footer, buf);
    }
}
