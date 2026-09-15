use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, summary::Summary, text::Text},
    layout::verical::Vertical,
    steps::StepTrait,
    utils::install::{InstallEvent, install},
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub struct Confirm;

impl StepTrait for Confirm {
    fn on_key(&self, app_state: &mut AppState, key: KeyEvent) -> Transition {
        match key.code {
            KeyCode::BackTab => Transition::Back,
            KeyCode::Enter => {
                let state = app_state.state.clone();
                let (tx, rx): (Sender<InstallEvent>, Receiver<InstallEvent>) = mpsc::channel();
                thread::spawn(move || {
                    let result = install(&state, |ratio, label| {
                        let _ = tx.send(InstallEvent::Progress {
                            ratio,
                            label: label.to_string(),
                        });
                    });
                    match result {
                        Ok(_) => {
                            let _ = tx.send(InstallEvent::Done);
                        }
                        Err(e) => {
                            let _ = tx.send(InstallEvent::Failed(e));
                        }
                    }
                });
                app_state.install_rx = Some(rx);
                Transition::Next
            }
            _ => Transition::None,
        }
    }

    fn render(&self, app_state: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new("Confirm").render(area, buf);
        let areas = Vertical::new().field_height(12).split(inner, 1);

        Text::new()
            .content("Review your choices".into())
            .render(areas.intro, buf);

        let user = format!("User:     {}", app_state.state.username.value());
        let host = format!("Host:     {}", app_state.state.hostname.value());
        let boot_manager = format!(
            "Boot:     {}",
            app_state.state.boot_managers.selected().unwrap_or("-")
        );
        let disk = format!("Disk:     {}", app_state.state.disk.display());
        let swap = format!("Swap:     {}", app_state.state.swap.display());
        let timezone = format!(
            "Timezone: {}",
            app_state.state.timezone.selected().unwrap_or("-")
        );
        let locale = format!(
            "Locale:   {}",
            app_state.state.locale.selected().unwrap_or("-")
        );
        let keymap = format!(
            "Keymap:   {}",
            app_state.state.keymap.selected().unwrap_or("-")
        );
        let summary = Summary::new(vec![
            &user,
            &host,
            &boot_manager,
            &disk,
            &swap,
            &timezone,
            &locale,
            &keymap,
        ]);
        summary.render(areas.fields[0], buf);

        Text::new()
            .content("Enter to install · Shift-Tab back".into())
            .render(areas.footer, buf);
    }
}
