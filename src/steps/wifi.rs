use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

use super::{Transition, step_manager::AppState};
use crate::{
    components::{InputComponent, card::Card, text::Text},
    layout::verical::Vertical,
    steps::{
        StepTrait,
        constants::{
            PROMPT_CARD_FOOTER, SCROLL_LIST_CARD_FOOTER, WIFI_C_NAME, WIFI_F_C, WIFI_F_H,
            WIFI_SKIP, WIFI_T_FOOTER_NET, WIFI_T_FOOTER_PASS, WIFI_T_INTRO_NET, WIFI_T_INTRO_PASS,
        },
    },
    utils::wifi::{wifi_connect, wifi_device},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiPhase {
    Networks,
    Password,
}

pub struct Wifi;

impl StepTrait for Wifi {
    fn on_key(&self, app: &mut AppState, key: KeyEvent) -> Transition {
        match app.wifi_phase {
            WifiPhase::Networks => Self::on_key_networks(app, key),
            WifiPhase::Password => Self::on_key_password(app, key),
        }
    }

    fn render(&self, app: &mut AppState, area: Rect, buf: &mut Buffer) {
        match app.wifi_phase {
            WifiPhase::Networks => Self::render_networks(app, area, buf),
            WifiPhase::Password => Self::render_password(app, area, buf),
        }
    }
}

impl Wifi {
    fn on_key_networks(app: &mut AppState, key: KeyEvent) -> Transition {
        match key.code {
            KeyCode::BackTab => Transition::Back,
            _ => {
                if !app.state.wifi_networks.on_key(key) {
                    return Transition::None;
                }
                let Some(selected) = app.state.wifi_networks.selected() else {
                    return Transition::None;
                };
                if selected == WIFI_SKIP {
                    app.state.wifi_ssid.clear();
                    return Transition::Next;
                }
                app.state.wifi_ssid = selected.to_string();
                app.state.wifi_password.set_focused(true);
                app.wifi_error = None;
                app.wifi_phase = WifiPhase::Password;
                Transition::None
            }
        }
    }

    fn on_key_password(app: &mut AppState, key: KeyEvent) -> Transition {
        match key.code {
            KeyCode::BackTab => {
                app.wifi_phase = WifiPhase::Networks;
                app.wifi_error = None;
                Transition::None
            }
            KeyCode::Enter => {
                let pass = app.state.wifi_password.value().to_string();
                let device = match wifi_device() {
                    Ok(d) => d,
                    Err(e) => {
                        app.wifi_error = Some(e);
                        return Transition::None;
                    }
                };
                match wifi_connect(&device, &app.state.wifi_ssid, &pass) {
                    Ok(_) => {
                        app.wifi_error = None;
                        Transition::Next
                    }
                    Err(e) => {
                        app.wifi_error = Some(e);
                        app.state.wifi_password.set_invalid(true);
                        Transition::None
                    }
                }
            }
            _ => {
                let _ = app.state.wifi_password.on_key(key);
                Transition::None
            }
        }
    }

    fn render_networks(app: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(WIFI_C_NAME)
            .footer(SCROLL_LIST_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new()
            .field_height(WIFI_F_H)
            .split(inner, WIFI_F_C);

        Text::new()
            .content(WIFI_T_INTRO_NET)
            .render(areas.intro, buf);
        app.state.wifi_networks.render(areas.fields[0], buf);
        Text::new()
            .content(WIFI_T_FOOTER_NET)
            .render(areas.footer, buf);
    }

    fn render_password(app: &mut AppState, area: Rect, buf: &mut Buffer) {
        let inner = Card::new(WIFI_C_NAME)
            .footer(PROMPT_CARD_FOOTER)
            .render(area, buf);
        let areas = Vertical::new().field_height(4).split(inner, 1);

        let intro = format!("{WIFI_T_INTRO_PASS} ({})", app.state.wifi_ssid);
        Text::new().content(&intro).render(areas.intro, buf);
        app.state.wifi_password.render(areas.fields[0], buf);

        let footer = app.wifi_error.as_deref().unwrap_or(WIFI_T_FOOTER_PASS);
        Text::new().content(footer).render(areas.footer, buf);
    }
}
