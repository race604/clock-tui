use chrono::{DateTime, Duration, Local};
use clock_tui::bricks_text::BricksText;
use tui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use super::{format_duration, render_centered};

pub struct Stopwatch {
    pub size: u16,
    pub style: Style,
    duration: Duration,
    started_at: Option<DateTime<Local>>,
}

impl Stopwatch {
    pub(crate) fn new(size: u16, style: Style) -> Self {
        Self {
            size,
            style,
            duration: Duration::zero(),
            started_at: Some(Local::now()),
        }
    }

    pub(crate) fn is_paused(&self) -> bool {
        self.started_at.is_none()
    }

    pub(crate) fn pause(&mut self) {
        if let Some(start_at) = self.started_at {
            let now = Local::now();
            self.duration = self.duration + now.signed_duration_since(start_at);
            self.started_at = None;
        }
    }

    pub(crate) fn resume(&mut self) {
        if self.started_at.is_none() {
            self.started_at = Some(Local::now());
        }
    }

    pub(crate) fn total_time(&self) -> Duration {
        if let Some(start_at) = self.started_at {
            let now = Local::now();
            self.duration + now.signed_duration_since(start_at)
        } else {
            self.duration
        }
    }
}

impl Widget for &Stopwatch {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let time_str = format_duration(self.total_time());
        let text = BricksText::new(time_str.as_str(), self.size, self.size, self.style);
        render_centered(area, buf, &text);
    }
}
