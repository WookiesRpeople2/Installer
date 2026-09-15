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

pub struct Disk;

impl StepTrait for Disk {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu(&mut app_state.state.disks, key, |disks| {
            if let Some(line) = disks.selected() {
                let dev = line.split_whitespace().next().unwrap_or(line);
                app_state.state.disk = PathBuf::from(dev);
            }
        })
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new("Disk")
            .footer("↑↓ scroll · Enter select")
            .render(area, buf);
        let areas = Vertical::new().field_height(12).split(inner, 2);

        Text::new()
            .content("Select install target".into())
            .render(areas.intro, buf);
        app_state.state.disks.render(areas.fields[0], buf);
        Text::new()
            .content("WARNING: All data on this disk will be destroyed".into())
            .render(areas.footer, buf);
    }
}
