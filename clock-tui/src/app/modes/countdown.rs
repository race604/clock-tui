use crate::clock_text::font::bricks::BricksFont;
use crate::clock_text::ClockText;
use chrono::{DateTime, Duration, Local};
use ratatui::{style::Style, widgets::Widget};

use super::{format_duration, render_centered, DurationFormat};

pub struct Countdown {
    pub size: u16,
    pub style: Style,
    pub time: DateTime<Local>,
    pub title: Option<String>,
    pub continue_on_zero: bool,
    pub(crate) reverse: bool,
    pub(crate) format: DurationFormat,
}

impl Countdown {
    pub(crate) fn remaining_time(&self) -> Duration {
        let now = Local::now();
        let result = self.time.signed_duration_since(now);
        if self.reverse {
            -result
        } else {
            result
        }
    }
}

impl Widget for &Countdown {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let remaining_time = self.remaining_time();
        let time_str = if remaining_time < Duration::zero() && !self.continue_on_zero {
            if (remaining_time.num_milliseconds()).abs() % 1000 < 500 {
                return;
            } else {
                format_duration(Duration::zero(), self.format)
            }
        } else {
            format_duration(remaining_time, self.format)
        };

        let font = BricksFont::new(self.size);
        let text = ClockText::new(time_str.to_string(), &font, self.style);
        render_centered(area, buf, &text, self.title.to_owned(), None);
    }
}
