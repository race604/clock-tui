mod clock;
mod countdown;
mod stopwatch;
mod timer;
mod pause;

use std::cmp::min;
use std::fmt::Write as _;

use crate::clock_text::BricksText;
use chrono::Duration;
pub(crate) use clock::Clock;
pub(crate) use countdown::Countdown;
pub(crate) use stopwatch::Stopwatch;
pub(crate) use timer::Timer;
pub(crate) use pause::Pause;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Span,
    widgets::{Paragraph, Widget},
};

#[derive(Copy, Clone)]
pub(crate) enum DurationFormat {
    /// Hours, minutes, seconds, deciseconds
    HourMinSecDeci,
    /// Hours, minutes, seconds
    HourMinSec,
}

fn format_duration(duration: Duration, format: DurationFormat) -> String {
    let is_neg = duration < Duration::zero();
    let duration = if is_neg { -duration } else { duration };

    let millis = duration.num_milliseconds();
    let seconds = millis / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;
    let mut result = String::new();

    fn append_number(s: &mut String, num: i64) {
        if s.is_empty() {
            let _ = write!(s, "{}", num);
        } else {
            let _ = write!(s, "{:02}", num);
        }
    }

    if days > 0 {
        let _ = write!(result, "{}:", days);
    }
    if hours > 0 {
        append_number(&mut result, hours % 24);
        result.push(':');
    }
    append_number(&mut result, minutes % 60);
    result.push(':');

    if is_neg {
        result.insert(0, '-');
    }
    match format {
        DurationFormat::HourMinSecDeci => {
            let _ = write!(result, "{:02}.{}", seconds % 60, (millis % 1000) / 100);
        }
        DurationFormat::HourMinSec => {
            let _ = write!(result, "{:02}", seconds % 60);
        }
    }

    result
}

fn render_centered(
    area: Rect,
    buf: &mut Buffer,
    text: &BricksText,
    header: Option<String>,
    footer: Option<String>,
) {
    let text_size = text.size();
    let text_area = Rect {
        x: area.x + (area.width.saturating_sub(text_size.0)) / 2,
        y: area.y + (area.height.saturating_sub(text_size.1)) / 2,
        width: min(text_size.0, area.width),
        height: min(text_size.1, area.height),
    };
    text.render(text_area, buf);

    let render_text_center = |text: &str, top: u16, buf: &mut Buffer| {
        let text_len = text.len() as u16;
        let paragrahp = Paragraph::new(Span::from(text)).style(Style::default());

        let para_area = Rect {
            x: area.left() + (area.width.saturating_sub(text_len)) / 2,
            y: top,
            width: min(text_len, area.width),
            height: min(1, area.height),
        };
        paragrahp.render(para_area, buf);
    };

    if let Some(text) = header {
        if area.top() + 2 <= text_area.top() {
            render_text_center(text.as_str(), text_area.top() - 2, buf);
        }
    }

    if let Some(text) = footer {
        if area.bottom() >= text_area.bottom() + 2 {
            render_text_center(text.as_str(), text_area.bottom() + 1, buf);
        }
    }
}
