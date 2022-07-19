use std::cmp::min;

use chrono::Local;
use clock_tui::bricks_text::BricksText;
use tui::{
    layout::Rect,
    style::Style,
    text::Span,
    widgets::{Paragraph, Widget},
};

pub(crate) struct Clock {
    pub size: u16,
    pub style: Style,
    pub long: bool,
}

impl Widget for &Clock {
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let now = Local::now();
        let time_str = if self.long {
            let mut str = now.format("%H:%M:%S%.3f").to_string();
            str.truncate(str.len() - 2);
            str
        } else {
            now.format("%H:%M:%S").to_string()
        };
        let time_str = time_str.as_str();
        let text = BricksText::new(time_str, self.size, self.size, self.style);
        let text_size = text.size();
        let text_area = Rect {
            x: area.x + (area.width.saturating_sub(text_size.0)) / 2,
            y: area.y + (area.height.saturating_sub(text_size.1)) / 2,
            width: min(text_size.0, area.width),
            height: min(text_size.0, area.height),
        };
        text.render(text_area, buf);
        let text = now.format("%Y-%m-%d %Z").to_string();
        let text_len = text.as_str().len() as u16;
        let paragrahp = Paragraph::new(Span::from(text)).style(Style::default());

        let para_area = Rect {
            x: area.x + (area.width.saturating_sub(text_len)) / 2,
            y: text_area.y.saturating_sub(2),
            width: min(text_len, area.width),
            height: min(1, area.height),
        };
        paragrahp.render(para_area, buf);
    }
}
