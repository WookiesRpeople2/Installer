use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::StepTrait,
    utils::cmd::reboot,
};

pub struct Done;

impl StepTrait for Done {
    fn on_key(&self, _state: &mut AppState, key: KeyEvent) -> Transition {
        match key.code {
            KeyCode::Enter => {
                let _ = reboot();
                Transition::Quit
            }
            _ => Transition::None,
        }
    }

    fn render(&self, _state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new("Done").render(area, buf);
        let areas = Vertical::new().field_height(4).split(inner, 0);

        Text::new()
            .content("Install complete".into())
            .render(areas.intro, buf);
        Text::new()
            .content("Press Enter to reboot".into())
            .render(areas.footer, buf);
    }
}
