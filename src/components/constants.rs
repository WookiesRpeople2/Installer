use ratatui::style::Color;

pub const BG: Color = Color::Rgb(12, 12, 14);
pub const FG: Color = Color::Rgb(220, 220, 220);
pub const BORDER: Color = Color::Rgb(80, 80, 90);
pub const ACCENT: Color = Color::Cyan;
pub const MUTED: Color = Color::Gray;
pub const FAINT: Color = Color::DarkGray;
pub const ERROR: Color = Color::Rgb(220, 80, 80);

pub const CARD_DEFAULT_BRAND: &str = "Modular";
pub const CARD_DEFAULT_FOOTER: &str = "Enter confirm · Esc quit";
pub const CARD_MARGIN_X: u16 = 4;
pub const CARD_MARGIN_Y: u16 = 2;
pub const CARD_MARGIN_BOTH_DIRECTIONS: u16 = 2;
pub const CARD_MAX_W: u16 = 40;
pub const CARD_MAX_H: u16 = 12;

pub const GAUGE_MIN_RATIO: f64 = 0.0;
pub const GAUGE_MAX_RATIO: f64 = 1.0;
pub const GAUGE_PERCENTAGE: f64 = 100.0;

pub const PROMPT_DEFAULT_MAX_LEN: usize = 32;
pub const PROMPT_DEFAULT_MASK: bool = false;
pub const PROMPT_DEFAULT_FOCUSED: bool = false;
pub const PROMPT_DEFAULT_INVALID: bool = false;
pub const PROMPT_CURSOR: &str = "▌";
pub const PROMPT_SATURATING: usize = 6;
pub const PROPMT_PADDING: u16 = 1;

pub const SCROLL_W_SAT: usize = 6;
pub const SCROLL_H_SAT: usize = 2;
pub const SCROLL_DIR_SAT: usize = 5;
pub const SCROLL_SPACING: u16 = 1;
pub const SCROLL_ICON: &str = "› ";

pub const SUMMARY_W_SAT: usize = 4;
