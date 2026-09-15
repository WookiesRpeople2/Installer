use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Gauge as RatatuiGauge, Widget},
};

use crate::components::constants;

pub struct Gauge {
    ratio: f64,
    label: String,
}

impl Gauge {
    pub fn new(ratio: f64, label: impl Into<String>) -> Self {
        Self {
            ratio: ratio.clamp(0.0, 1.0),
            label: label.into(),
        }
    }
}

impl Widget for &Gauge {
    fn render(self, area: Rect, buf: &mut Buffer) {
        RatatuiGauge::default()
            .block(
                Block::bordered()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(constants::BORDER)),
            )
            .gauge_style(Style::default().fg(constants::ACCENT))
            .ratio(self.ratio)
            .label(format!("{}  {:.0}%", self.label, self.ratio * 100.0))
            .render(area, buf);
    }
}
