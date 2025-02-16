use crate::clock_text::BricksText;
use chrono::{DateTime, Duration, Local};
use tui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use crate::app::Pause;

use super::{format_duration, render_centered, DurationFormat};

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

    pub(crate) fn total_time(&self) -> Duration {
        if let Some(start_at) = self.started_at {
            let now = Local::now();
            self.duration + now.signed_duration_since(start_at)
        } else {
            self.duration
        }
    }

    pub fn get_display_time(&self) -> String {
        format_duration(self.total_time(), DurationFormat::HourMinSecDeci)
    }
}

impl Widget for &Stopwatch {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let time_str = format_duration(self.total_time(), DurationFormat::HourMinSecDeci);
        let text = BricksText::new(time_str.as_str(), self.size, self.size, self.style);
        let footer = if self.is_paused() {
            Some("PAUSED (press <SPACE> to resume)".to_string())
        } else {
            None
        };
        render_centered(area, buf, &text, None, footer);
    }
}

impl Pause for Stopwatch {
    fn is_paused(&self) -> bool {
        self.started_at.is_none()
    }

    fn pause(&mut self) {
        if let Some(start_at) = self.started_at {
            let now = Local::now();
            self.duration = self.duration + now.signed_duration_since(start_at);
            self.started_at = None;
        }
    }

    fn resume(&mut self) {
        if self.started_at.is_none() {
            self.started_at = Some(Local::now());
        }
    }
}
