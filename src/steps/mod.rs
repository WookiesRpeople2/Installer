pub mod boot_manager;
pub mod confirm;
pub mod disk;
pub mod done;
pub mod hostname;
pub mod installing;
pub mod keymap;
pub mod locale;
pub mod password;
pub mod step_manager;
pub mod swap;
pub mod timezone;
pub mod username;
pub mod welcome;

pub enum Transition {
    None,
    Next,
    Back,
    Quit,
}

pub trait StepTrait {
    fn on_key(
        &self,
        state: &mut step_manager::AppState,
        key: crossterm::event::KeyEvent,
    ) -> Transition;
    fn render(
        &self,
        state: &mut step_manager::AppState,
        area: ratatui::layout::Rect,
        buf: &mut ratatui::buffer::Buffer,
    );
}
