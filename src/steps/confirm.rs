use super::{Transition, step_manager::AppState};
use crate::{
    components::{card::Card, summary::Summary, text::Text},
    layout::verical::Vertical,
    steps::{
        StepTrait,
        constants::{
            CONF_C_NAME, CONF_F_C, CONF_F_H, CONF_T_BOOT, CONF_T_DISK, CONF_T_FOOTER, CONF_T_HOST,
            CONF_T_INTRO, CONF_T_KEY, CONF_T_LOCALE, CONF_T_SWAP, CONF_T_TZ, CONF_T_USER,
            PROMPT_CARD_FOOTER,
        },
    },
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
        let inner = Card::new(CONF_C_NAME)
            .footer(PROMPT_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new()
            .field_height(CONF_F_H)
            .split(inner, CONF_F_C);

        Text::new().content(CONF_T_INTRO).render(areas.intro, buf);

        let pairs: Vec<(&str, String)> = vec![
            (CONF_T_USER, app_state.state.username.value().to_string()),
            (CONF_T_HOST, app_state.state.hostname.value().to_string()),
            (
                CONF_T_BOOT,
                app_state
                    .state
                    .boot_managers
                    .selected()
                    .unwrap_or("-")
                    .to_string(),
            ),
            (CONF_T_DISK, app_state.state.disk.display().to_string()),
            (
                CONF_T_SWAP,
                if app_state.state.swap_on_install {
                    format!("{} (new partition)", app_state.state.disk.display())
                } else if app_state.state.swap.as_os_str().is_empty() {
                    "None".into()
                } else {
                    app_state.state.swap.display().to_string()
                },
            ),
            (
                CONF_T_TZ,
                app_state
                    .state
                    .timezone
                    .selected()
                    .unwrap_or("-")
                    .to_string(),
            ),
            (
                CONF_T_LOCALE,
                app_state.state.locale.selected().unwrap_or("-").to_string(),
            ),
            (
                CONF_T_KEY,
                app_state.state.keymap.selected().unwrap_or("-").to_string(),
            ),
        ];

        let label_width = pairs
            .iter()
            .map(|(l, _)| l.chars().count())
            .max()
            .unwrap_or(0);

        let lines: Vec<String> = pairs
            .iter()
            .map(|(label, value)| format!("{:<width$}   {}", label, value, width = label_width))
            .collect();

        let line_refs: Vec<&str> = lines.iter().map(String::as_str).collect();
        let summary = Summary::new(line_refs);
        summary.render(areas.fields[0], buf);

        Text::new().content(CONF_T_FOOTER).render(areas.footer, buf);
    }
}
