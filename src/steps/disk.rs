use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, text::Text},
    layout::verical::Vertical,
    steps::{
        StepTrait,
        constants::{
            DISK_C_NAME, DISK_F_C, DISK_F_H, DISK_T_FOOTER, DISK_T_INTRO, SCROLL_LIST_CARD_FOOTER,
            SWAP_NONE, SWAP_ON_INSTALL,
        },
    },
    utils::{cmd::get_swap_candidates, key::handle_on_key_menu},
};
use std::path::PathBuf;

pub struct Disk;

impl StepTrait for Disk {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        handle_on_key_menu(&mut app_state.state.disks, key, |disks| {
            if let Some(line) = disks.selected() {
                let dev = line.split_whitespace().next().unwrap_or(line);
                app_state.state.disk = PathBuf::from(dev);
                app_state.state.swaps.set_items({
                    let mut items = vec![SWAP_ON_INSTALL.to_string(), SWAP_NONE.to_string()];
                    items.extend(get_swap_candidates(&app_state.state.disk));
                    items
                });
                app_state.state.swap.clear();
                app_state.state.swap_on_install = false;
            }
        })
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(DISK_C_NAME)
            .footer(SCROLL_LIST_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new()
            .field_height(DISK_F_H)
            .split(inner, DISK_F_C);

        Text::new().content(DISK_T_INTRO).render(areas.intro, buf);
        app_state.state.disks.render(areas.fields[0], buf);
        Text::new().content(DISK_T_FOOTER).render(areas.footer, buf);
    }
}
