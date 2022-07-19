mod clock;
mod stopwatch;
mod timer;

use std::cmp::min;

use chrono::Duration;
pub(crate) use clock::Clock;
use clock_tui::bricks_text::BricksText;
pub(crate) use stopwatch::Stopwatch;
pub(crate) use timer::Timer;
use tui::{buffer::Buffer, layout::Rect, widgets::Widget};

fn format_duration(duration: Duration) -> String {
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
    result.push_str(&format!("{:02}.{}", seconds % 60, (millis % 1000) / 100));

    result
}

fn render_centered(area: Rect, buf: &mut Buffer, text: &BricksText) {
    let text_size = text.size();
    let text_area = Rect {
        x: area.x + (area.width.saturating_sub(text_size.0)) / 2,
        y: area.y + (area.height.saturating_sub(text_size.1)) / 2,
        width: min(text_size.0, area.width),
        height: min(text_size.0, area.height),
    };
    text.render(text_area, buf);
}
