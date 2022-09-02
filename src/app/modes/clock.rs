use chrono::{Local, Utc};
use chrono_tz::Tz;
use clock_tui::bricks_text::BricksText;
use tui::{layout::Rect, style::Style, widgets::Widget};

use super::render_centered;

pub(crate) struct Clock {
    pub size: u16,
    pub style: Style,
    pub show_date: bool,
    pub show_millis: bool,
    pub show_secs: bool,
    pub timezone: Option<Tz>,
}

impl Widget for &Clock {
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let now = if let Some(ref tz) = self.timezone {
            Utc::now().with_timezone(tz).naive_local()
        } else {
            Local::now().naive_local()
        };
        let mut time_str = now.format("%H:%M:%S%.3f").to_string();
        if self.show_millis {
            time_str.truncate(time_str.len() - 2);
        } else if self.show_secs {
            time_str.truncate(time_str.len() - 4);
        } else {
            time_str.truncate(time_str.len() - 7);
        };
        let time_str = time_str.as_str();
        let text = BricksText::new(time_str, self.size, self.size, self.style);
        let header = if self.show_date {
            let mut title = now.format("%Y-%m-%d").to_string();
            if let Some(tz) = self.timezone {
                title.push(' ');
                title.push_str(tz.name());
            }
            Some(title)
        } else {
            None
        };
        render_centered(area, buf, &text, header, None);
    }
}
