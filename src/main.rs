use std::io;

use installer::steps::step_manager::AppState;

pub fn main() -> io::Result<()> {
    ratatui::run(|terminal| AppState::default().run(terminal))
}
