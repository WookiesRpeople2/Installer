use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{InputComponent, card::Card, text::Text},
    layout::verical::Vertical,
    steps::{
        StepTrait,
        constants::{
            PROMPT_CARD_FOOTER, SCROLL_LIST_CARD_FOOTER, SWAP_C_NAME, SWAP_CREATE_OTHER, SWAP_F_C,
            SWAP_F_H, SWAP_NONE, SWAP_ON_INSTALL, SWAP_T_FOOTER_DISK, SWAP_T_FOOTER_MODE,
            SWAP_T_FOOTER_SIZE, SWAP_T_INTRO_DISK, SWAP_T_INTRO_MODE, SWAP_T_INTRO_SIZE,
        },
    },
    utils::cmd::get_swap_disks,
};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapPhase {
    Mode,
    Disk,
    Size,
}

pub struct Swap;
impl StepTrait for Swap {
    fn on_key(&self, app: &mut AppState, key: KeyEvent) -> Transition {
        match app.swap_phase {
            SwapPhase::Mode => Self::on_key_mode(app, key),
            SwapPhase::Disk => Self::on_key_disk(app, key),
            SwapPhase::Size => Self::on_key_size(app, key),
        }
    }
    fn render(&self, app: &mut AppState, area: Rect, buf: &mut Buffer) {
        match app.swap_phase {
            SwapPhase::Mode => Self::render_mode(app, area, buf),
            SwapPhase::Disk => Self::render_disk(app, area, buf),
            SwapPhase::Size => Self::render_size(app, area, buf),
        }
    }
}
impl Swap {
    fn clear_create_flags(app: &mut AppState) {
        app.state.swap_on_install = false;
        app.state.swap_create_other = false;
        app.state.swap_disk.clear();
        app.state.swap.clear();
    }

    fn on_key_mode(app: &mut AppState, key: KeyEvent) -> Transition {
        match key.code {
            KeyCode::BackTab => {
                app.swap_phase = SwapPhase::Mode;
                Transition::Back
            }
            _ => {
                if !app.state.swaps.on_key(key) {
                    return Transition::None;
                }
                let selected = app.state.swaps.selected().map(|s| s.to_owned());
                match selected.as_deref() {
                    Some(SWAP_ON_INSTALL) => {
                        Self::clear_create_flags(app);
                        app.state.swap_on_install = true;
                        app.state.swap_size.set_focused(true);
                        app.swap_phase = SwapPhase::Size;
                        Transition::None
                    }
                    Some(SWAP_CREATE_OTHER) => {
                        Self::clear_create_flags(app);
                        app.state.swap_create_other = true;
                        app.state
                            .swap_disks
                            .set_items(get_swap_disks(&app.state.disk));
                        app.swap_phase = SwapPhase::Disk;
                        Transition::None
                    }
                    Some(SWAP_NONE) | None => {
                        Self::clear_create_flags(app);
                        app.state.swap_size_gb = 0;
                        Transition::Next
                    }
                    Some(line) => {
                        Self::clear_create_flags(app);
                        let dev = line.split_whitespace().next().unwrap_or(line);
                        app.state.swap = PathBuf::from(dev);
                        Transition::Next
                    }
                }
            }
        }
    }

    fn on_key_disk(app: &mut AppState, key: KeyEvent) -> Transition {
        match key.code {
            KeyCode::BackTab => {
                app.swap_phase = SwapPhase::Mode;
                Transition::None
            }
            _ => {
                if !app.state.swap_disks.on_key(key) {
                    return Transition::None;
                }
                if let Some(line) = app.state.swap_disks.selected() {
                    let dev = line.split_whitespace().next().unwrap_or(line);
                    app.state.swap_disk = PathBuf::from(dev);
                    app.state.swap_size.set_focused(true);
                    app.swap_phase = SwapPhase::Size;
                }
                Transition::None
            }
        }
    }

    fn on_key_size(app: &mut AppState, key: KeyEvent) -> Transition {
        match key.code {
            KeyCode::BackTab => {
                app.swap_phase = if app.state.swap_create_other {
                    SwapPhase::Disk
                } else {
                    SwapPhase::Mode
                };
                Transition::None
            }
            KeyCode::Enter => {
                let raw = app.state.swap_size.value().trim();
                let Ok(gb) = raw.parse::<u64>() else {
                    app.state.swap_size.set_invalid(true);
                    return Transition::None;
                };
                if gb == 0 {
                    app.state.swap_size.set_invalid(true);
                    return Transition::None;
                }
                app.state.swap_size_gb = gb;
                app.state.swap_size.clear_invalid();
                Transition::Next
            }
            _ => {
                let _ = app.state.swap_size.on_key(key);
                Transition::None
            }
        }
    }

    fn render_mode(app: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(SWAP_C_NAME)
            .footer(SCROLL_LIST_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new()
            .field_height(SWAP_F_H)
            .split(inner, SWAP_F_C);
        Text::new()
            .content(SWAP_T_INTRO_MODE)
            .render(areas.intro, buf);
        app.state.swaps.render(areas.fields[0], buf);
        Text::new()
            .content(SWAP_T_FOOTER_MODE)
            .render(areas.footer, buf);
    }

    fn render_disk(app: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(SWAP_C_NAME)
            .footer(SCROLL_LIST_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new()
            .field_height(SWAP_F_H)
            .split(inner, SWAP_F_C);
        Text::new()
            .content(SWAP_T_INTRO_DISK)
            .render(areas.intro, buf);
        app.state.swap_disks.render(areas.fields[0], buf);
        Text::new()
            .content(SWAP_T_FOOTER_DISK)
            .render(areas.footer, buf);
    }

    fn render_size(app: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(SWAP_C_NAME)
            .footer(PROMPT_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new().field_height(4).split(inner, 1);
        Text::new()
            .content(SWAP_T_INTRO_SIZE)
            .render(areas.intro, buf);
        app.state.swap_size.render(areas.fields[0], buf);
        Text::new()
            .content(SWAP_T_FOOTER_SIZE)
            .render(areas.footer, buf);
    }
}
