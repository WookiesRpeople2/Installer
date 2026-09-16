use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::{io, path::PathBuf};

use crate::components::prompt::Prompt;
use crate::components::scroll_list::ScrollList;
use crate::steps::boot_manager::BootManger;
use crate::steps::confirm::Confirm;
use crate::steps::disk::Disk;
use crate::steps::done::Done;
use crate::steps::hostname::Hostname;
use crate::steps::installing::Installing;
use crate::steps::keymap::Keymap;
use crate::steps::locale::Locale;
use crate::steps::password::Password;
use crate::steps::swap::Swap;
use crate::steps::timezone::Timezone;
use crate::steps::username::Username;
use crate::steps::welcome::Welcome;
use crate::steps::{StepTrait, Transition};
use crate::utils::cmd::{get_disks, reboot};
use crate::utils::install::InstallEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepId {
    Welcome,
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

    pub username: Prompt,
    pub hostname: Prompt,
    pub user_password: Prompt,
    pub root_password: Prompt,

    pub timezone: ScrollList,
    pub locale: ScrollList,
    pub keymap: ScrollList,
    pub disks: ScrollList,
    pub swaps: ScrollList,
    pub boot_managers: ScrollList,
}

pub struct AppState {
    pub state: State,
    pub install_ratio: f64,
    pub install_label: String,
    pub install_error: Option<String>,
    pub install_rx: Option<Receiver<InstallEvent>>,

    step: StepId,
    exit: bool,
}

impl Default for AppState {
    fn default() -> Self {
        let state = State {
            disk: PathBuf::new(),
            swap: PathBuf::new(),
            username: Prompt::new("Username", "type your name…").max_len(64),
            hostname: Prompt::new("Hostname", "modular").max_len(64),
            user_password: Prompt::new("User Password", "Set the user password")
                .mask(true)
                .max_len(64),
            root_password: Prompt::new("Root Password", "Set the root password")
                .mask(true)
                .max_len(64),
            timezone: ScrollList::new(
                "Timezone",
                vec![
                    "Europe/Paris".into(),
                    "Europe/London".into(),
                    "Europe/Berlin".into(),
                    "America/New_York".into(),
                    "America/Los_Angeles".into(),
                    "Asia/Tokyo".into(),
                    "UTC".into(),
                ],
            ),
            locale: ScrollList::new(
                "Locale",
                vec![
                    "en_US.UTF-8".into(),
                    "en_GB.UTF-8".into(),
                    "fr_FR.UTF-8".into(),
                    "de_DE.UTF-8".into(),
                    "es_ES.UTF-8".into(),
                    "it_IT.UTF-8".into(),
                    "pt_PT.UTF-8".into(),
                    "nl_NL.UTF-8".into(),
                    "pl_PL.UTF-8".into(),
                    "sv_SE.UTF-8".into(),
                    "ja_JP.UTF-8".into(),
                    "zh_CN.UTF-8".into(),
                ],
            ),
            keymap: ScrollList::new(
                "Keyboard",
                vec![
                    "us".into(),
                    "uk".into(),
                    "fr".into(),
                    "de".into(),
                    "es".into(),
                    "it".into(),
                    "pt".into(),
                    "nl".into(),
                    "pl".into(),
                    "sv".into(),
                    "dvorak".into(),
                    "colemak".into(),
                ],
            ),
            disks: ScrollList::new("Disks", get_disks()),
            swaps: ScrollList::new(
                "Swaps",
                get_disks()
                    .into_iter()
                    .chain(std::iter::once("None".to_string()))
                    .collect::<Vec<_>>(),
            ),
            boot_managers: ScrollList::new("Boot Managers", vec!["efi".into(), "grub".into()]),
        };

        Self {
            state,
            install_ratio: 0.0,
            install_label: "Starting…".into(),
            install_error: None,
            install_rx: None,

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
            Self::Welcome => Self::Username,
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
            Self::Username => Self::Welcome,
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
