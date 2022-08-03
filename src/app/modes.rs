mod clock;
mod stopwatch;
mod timer;

use std::cmp::min;

use chrono::Duration;
pub(crate) use clock::Clock;
use clock_tui::bricks_text::BricksText;
pub(crate) use stopwatch::Stopwatch;
pub(crate) use timer::Timer;
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
    let millis = duration.num_milliseconds();
    let seconds = millis / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;
    let mut result = String::new();
    if days > 0 {
        result.push_str(&format!("{}:", days));
    }
    if hours > 0 {
        result.push_str(&format!("{}:", hours % 24));
    }
    result.push_str(&format!("{}:", minutes % 60));
    match format {
        DurationFormat::HourMinSecDeci => {
            result.push_str(&format!("{:02}.{}", seconds % 60, (millis % 1000) / 100))
        }
        DurationFormat::HourMinSec => result.push_str(&format!("{:02}", seconds % 60)),
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
