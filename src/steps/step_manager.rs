use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::{io, path::PathBuf};

use crate::components::prompt::Prompt;
use crate::components::scroll_list::ScrollList;
use crate::steps::boot_manager::BootManger;
use crate::steps::confirm::Confirm;
use crate::steps::constants::{
    INSTALL_LABEL, INSTALL_RATIO, PASSWORD_MASK, SMGR_P_H_KEY, SMGR_P_H_MAX, SMGR_P_H_VALUE,
    SMGR_P_RP_KEY, SMGR_P_RP_MAX, SMGR_P_RP_VALUE, SMGR_P_SWAP_SIZE_KEY, SMGR_P_SWAP_SIZE_MAX,
    SMGR_P_SWAP_SIZE_VALUE, SMGR_P_U_KEY, SMGR_P_U_MAX, SMGR_P_U_VALUE, SMGR_P_UP_KEY,
    SMGR_P_UP_MAX, SMGR_P_UP_VALUE, SMGR_P_WP_KEY, SMGR_P_WP_MAX, SMGR_P_WP_VALUE, SMGR_S_BOOT_KEY,
    SMGR_S_BOOT_OPTS, SMGR_S_DISK_KEY, SMGR_S_KM_KEY, SMGR_S_KM_OPTS, SMGR_S_LOC_KEY,
    SMGR_S_LOC_OPTS, SMGR_S_SWAP_DISK_KEY, SMGR_S_SWAP_KEY, SMGR_S_TZ_KEY, SMGR_S_TZ_OPTS,
    SMGR_S_WIFI_KEY, WIFI_SKIP,
};
use crate::steps::disk::Disk;
use crate::steps::done::Done;
use crate::steps::hostname::Hostname;
use crate::steps::installing::Installing;
use crate::steps::keymap::Keymap;
use crate::steps::locale::Locale;
use crate::steps::password::Password;
use crate::steps::swap::{Swap, SwapPhase};
use crate::steps::timezone::Timezone;
use crate::steps::username::Username;
use crate::steps::welcome::Welcome;
use crate::steps::wifi::{Wifi, WifiPhase};
use crate::steps::{StepTrait, Transition};
use crate::utils::cmd::{get_disks, reboot};
use crate::utils::install::InstallEvent;
use crate::utils::wifi::{wifi_device, wifi_networks, wifi_scan};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepId {
    Welcome,
    Wifi,
    Username,
    Hostname,
    Password,
    Timezone,
    Locale,
    Keymap,
    BootManager,
    Disk,
    Swap,
    Confirm,
    Installing,
    Done,
}

#[derive(Clone)]
pub struct State {
    pub disk: PathBuf,
    pub swap: PathBuf,
    pub wifi_ssid: String,
    pub swap_disk: PathBuf,
    pub swap_on_install: bool,
    pub swap_create_other: bool,
    pub swap_size_gb: u64,

    pub username: Prompt,
    pub hostname: Prompt,
    pub swap_size: Prompt,
    pub user_password: Prompt,
    pub root_password: Prompt,
    pub wifi_password: Prompt,

    pub timezone: ScrollList,
    pub locale: ScrollList,
    pub keymap: ScrollList,
    pub disks: ScrollList,
    pub swaps: ScrollList,
    pub swap_disks: ScrollList,
    pub boot_managers: ScrollList,
    pub wifi_networks: ScrollList,
}

pub struct AppState {
    pub state: State,
    pub install_ratio: f64,
    pub install_label: String,
    pub install_error: Option<String>,
    pub install_rx: Option<Receiver<InstallEvent>>,
    pub wifi_phase: WifiPhase,
    pub wifi_error: Option<String>,
    pub swap_phase: SwapPhase,

    step: StepId,
    exit: bool,
}

impl Default for AppState {
    fn default() -> Self {
        let state = State {
            disk: PathBuf::new(),
            swap: PathBuf::new(),
            swap_disk: PathBuf::new(),
            swap_on_install: false,
            swap_create_other: false,
            swap_size_gb: 4,
            wifi_ssid: String::new(),
            username: Prompt::new(SMGR_P_U_KEY, SMGR_P_U_VALUE).max_len(SMGR_P_U_MAX),
            hostname: Prompt::new(SMGR_P_H_KEY, SMGR_P_H_VALUE).max_len(SMGR_P_H_MAX),
            swap_size: Prompt::new(SMGR_P_SWAP_SIZE_KEY, SMGR_P_SWAP_SIZE_VALUE)
                .max_len(SMGR_P_SWAP_SIZE_MAX)
                .filter(|c| c.is_ascii_digit()),
            user_password: Prompt::new(SMGR_P_UP_KEY, SMGR_P_UP_VALUE)
                .mask(PASSWORD_MASK)
                .max_len(SMGR_P_UP_MAX),
            root_password: Prompt::new(SMGR_P_RP_KEY, SMGR_P_RP_VALUE)
                .mask(PASSWORD_MASK)
                .max_len(SMGR_P_RP_MAX),
            wifi_password: Prompt::new(SMGR_P_WP_KEY, SMGR_P_WP_VALUE)
                .mask(PASSWORD_MASK)
                .max_len(SMGR_P_WP_MAX),
            timezone: ScrollList::new(SMGR_S_TZ_KEY, SMGR_S_TZ_OPTS.clone()),
            locale: ScrollList::new(SMGR_S_LOC_KEY, SMGR_S_LOC_OPTS.clone()),
            keymap: ScrollList::new(SMGR_S_KM_KEY, SMGR_S_KM_OPTS.clone()),
            disks: ScrollList::new(SMGR_S_DISK_KEY, get_disks()),
            swaps: ScrollList::new(SMGR_S_SWAP_KEY, vec![]),
            swap_disks: ScrollList::new(SMGR_S_SWAP_DISK_KEY, vec![]),
            boot_managers: ScrollList::new(SMGR_S_BOOT_KEY, SMGR_S_BOOT_OPTS.clone()),
            wifi_networks: ScrollList::new(SMGR_S_WIFI_KEY, {
                let mut items = vec![WIFI_SKIP.to_string()];
                if let Ok(dev) = wifi_device() {
                    let _ = wifi_scan(&dev);
                    items.extend(wifi_networks(&dev));
                }
                items
            }),
        };

        Self {
            state,
            install_ratio: INSTALL_RATIO,
            install_label: INSTALL_LABEL.into(),
            install_error: None,
            install_rx: None,
            wifi_phase: WifiPhase::Networks,
            wifi_error: None,
            swap_phase: SwapPhase::Mode,

            step: StepId::Welcome,
            exit: false,
        }
    }
}

impl AppState {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        self.poll_install();
        let timeout = if self.step == StepId::Installing {
            Duration::from_millis(100)
        } else {
            Duration::from_secs(60 * 60)
        };
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    let t = self.on_key(key);
                    self.apply(t);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn on_key(&mut self, key: event::KeyEvent) -> Transition {
        if key.code == KeyCode::Esc {
            return Transition::Quit;
        }

        self.step.widget().on_key(self, key)
    }

    fn apply(&mut self, t: Transition) {
        match t {
            Transition::None => {}
            Transition::Quit => {
                let _ = reboot();
                self.exit = true
            }
            Transition::Next => self.step = self.step.next(),
            Transition::Back => self.step = self.step.back(),
        }
    }

    fn poll_install(&mut self) {
        let Some(rx) = self.install_rx.as_ref() else {
            return;
        };
        let mut done = false;
        let mut failed: Option<String> = None;
        while let Ok(ev) = rx.try_recv() {
            match ev {
                InstallEvent::Progress { ratio, label } => {
                    self.install_ratio = ratio;
                    self.install_label = label;
                }
                InstallEvent::Done => done = true,
                InstallEvent::Failed(e) => failed = Some(e),
            }
        }
        if done {
            self.install_rx = None;
            self.step = StepId::Done;
        } else if let Some(e) = failed {
            self.install_error = Some(e);
            self.install_rx = None;
        }
    }
}

impl StepId {
    fn next(self) -> Self {
        match self {
            Self::Welcome => Self::Wifi,
            Self::Wifi => Self::Username,
            Self::Username => Self::Hostname,
            Self::Hostname => Self::Password,
            Self::Password => Self::Timezone,
            Self::Timezone => Self::Locale,
            Self::Locale => Self::Keymap,
            Self::Keymap => Self::BootManager,
            Self::BootManager => Self::Disk,
            Self::Disk => Self::Swap,
            Self::Swap => Self::Confirm,
            Self::Confirm => Self::Installing,
            Self::Installing => Self::Done,
            Self::Done => Self::Done,
        }
    }

    fn back(self) -> Self {
        match self {
            Self::Welcome => Self::Welcome,
            Self::Wifi => Self::Welcome,
            Self::Username => Self::Wifi,
            Self::Hostname => Self::Username,
            Self::Password => Self::Hostname,
            Self::Timezone => Self::Password,
            Self::Locale => Self::Timezone,
            Self::Keymap => Self::Locale,
            Self::BootManager => Self::Keymap,
            Self::Disk => Self::BootManager,
            Self::Swap => Self::Disk,
            Self::Confirm => Self::Swap,
            Self::Installing => Self::Installing,
            Self::Done => Self::Confirm,
        }
    }

    fn widget(&self) -> &'static dyn StepTrait {
        match self {
            StepId::Welcome => &Welcome,
            StepId::Wifi => &Wifi,
            StepId::Username => &Username,
            StepId::Hostname => &Hostname,
            StepId::Password => &Password,
            StepId::Timezone => &Timezone,
            StepId::Locale => &Locale,
            StepId::Keymap => &Keymap,
            StepId::BootManager => &BootManger,
            StepId::Disk => &Disk,
            StepId::Swap => &Swap,
            StepId::Confirm => &Confirm,
            StepId::Installing => &Installing,
            StepId::Done => &Done,
        }
    }
}

impl Widget for &mut AppState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.step.widget().render(self, area, buf);
    }
}
