use std::io;

use modular_installer::steps::step_manager::AppState;

pub fn main() -> io::Result<()> {
    ratatui::run(|terminal| AppState::default().run(terminal))
}
