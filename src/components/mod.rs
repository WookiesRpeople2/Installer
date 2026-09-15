pub mod background;
pub mod card;
mod constants;
pub mod gauge;
pub mod prompt;
pub mod scroll_list;
pub mod summary;
pub mod text;

pub trait InputComponent {
    fn on_key(&mut self, key: crossterm::event::KeyEvent) -> bool;
}
