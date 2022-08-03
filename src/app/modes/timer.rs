use chrono::{DateTime, Duration, Local};
use clock_tui::bricks_text::BricksText;
use tui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use super::{format_duration, render_centered, DurationFormat};

pub struct Timer {
    pub size: u16,
    pub style: Style,
    format: DurationFormat,
    duration: Duration,
    ended_at: Option<DateTime<Local>>,
}

impl Timer {
    pub(crate) fn new(duration: Duration, size: u16, style: Style, format: DurationFormat) -> Self {
        Self {
            duration,
            size,
            style,
            format,
            ended_at: Some(Local::now() + duration),
        }
    }

    pub(crate) fn is_paused(&self) -> bool {
        self.ended_at.is_none()
    }

    pub(crate) fn pause(&mut self) {
        if let Some(end_at) = self.ended_at {
            if end_at <= Local::now() {
                self.duration = Duration::zero();
            } else {
                self.duration = end_at - Local::now();
            }
            self.ended_at = None;
        }
    }

    pub(crate) fn resume(&mut self) {
        if self.ended_at.is_none() {
            self.ended_at = Some(Local::now() + self.duration);
        }
    }

    pub(crate) fn remaining_time(&self) -> Duration {
        if let Some(end_at) = self.ended_at {
            let now = Local::now();
            end_at.signed_duration_since(now)
        } else {
            self.duration
        }
    }
}

impl Widget for &Timer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let remaining_time = self.remaining_time();
        let time_str = if remaining_time < Duration::zero() {
            if remaining_time.num_seconds() % 2 == 0 {
                return;
            } else {
                format_duration(Duration::zero(), self.format)
            }
        } else {
            format_duration(remaining_time, self.format)
        };

        let text = BricksText::new(time_str.as_str(), self.size, self.size, self.style);
        let footer = if self.is_paused() {
            Some("PAUSED (press <SPACE> to resume)".to_string())
        } else {
            None
        };
        render_centered(area, buf, &text, None, footer);
    }
}
