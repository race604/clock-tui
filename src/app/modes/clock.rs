use chrono::Local;
use clock_tui::bricks_text::BricksText;
use tui::{layout::Rect, style::Style, widgets::Widget};

use super::render_centered;

pub(crate) struct Clock {
    pub size: u16,
    pub style: Style,
    pub show_date: bool,
    pub show_millis: bool,
}

impl Widget for &Clock {
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let now = Local::now();
        let time_str = if self.show_millis {
            let mut str = now.format("%H:%M:%S%.3f").to_string();
            str.truncate(str.len() - 2);
            str
        } else {
            now.format("%H:%M:%S").to_string()
        };
        let time_str = time_str.as_str();
        let text = BricksText::new(time_str, self.size, self.size, self.style);
        let header = if self.show_date {
            Some(now.format("%Y-%m-%d %Z").to_string())
        } else {
            None
        };
        render_centered(area, buf, &text, header, None);
    }
}
