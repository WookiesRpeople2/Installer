use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Gauge as RatatuiGauge, Widget},
};

use crate::components::constants::{
    ACCENT, BORDER, GAUGE_MAX_RATIO, GAUGE_MIN_RATIO, GAUGE_PERCENTAGE,
};

pub struct Gauge {
    ratio: f64,
    label: String,
}

impl Gauge {
    pub fn new(ratio: f64, label: impl Into<String>) -> Self {
        Self {
            ratio: ratio.clamp(GAUGE_MIN_RATIO, GAUGE_MAX_RATIO),
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
                    .border_style(Style::default().fg(BORDER)),
            )
            .gauge_style(Style::default().fg(ACCENT))
            .ratio(self.ratio)
            .label(format!(
                "{}  {:.0}%",
                self.label,
                self.ratio * GAUGE_PERCENTAGE
            ))
            .render(area, buf);
    }
}
