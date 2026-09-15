use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::StepTrait,
    utils::key::handle_on_key_menu,
};
use std::path::PathBuf;

pub struct Swap;

impl StepTrait for Swap {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu(&mut app_state.state.swaps, key, |swaps| {
            if let Some(line) = swaps.selected()
                && swaps.selected() != "None".into()
            {
                let dev = line.split_whitespace().next().unwrap_or(line);
                app_state.state.swap = PathBuf::from(dev);
            }
        })
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new("Swap Partition")
            .footer("WARNING: selected disk will be erased")
            .render(area, buf);
        let areas = Vertical::new().field_height(12).split(inner, 2);

        Text::new()
            .content("Select install target".into())
            .render(areas.intro, buf);
        app_state.state.swaps.render(areas.fields[0], buf);
        Text::new()
            .content("Select None if you do not want to use the swap partition".into())
            .render(areas.footer, buf);
    }
}
