use tui::{style::Style, widgets::Widget};

use self::font::bricks::Bricks;
use self::font::Font;
use self::point::Point;

mod font;
mod point;

pub struct BricksText {
    text: String,
    space: u16,
    style: Style,
    font: Bricks,
}

impl BricksText {
    pub fn new(text: &str, size: u16, space: u16, style: Style) -> BricksText {
        BricksText {
            text: text.to_string(),
            space,
            style,
            font: Bricks { size },
        }
    }

    pub fn size(&self) -> (u16, u16) {
        let Point(w, h) = self.font.size();
        let n_chars = self.text.chars().count() as u16;
        (w * n_chars + self.space * (n_chars - 1), h)
    }
}

impl Widget for &BricksText {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let mut area = area;
        for char in self.text.chars() {
            let Point(w, _) = self.font.size();
            self.font.render(char, self.style, area, buf);
            let l = w + self.space;
            area.x += l;
            area.width = area.width.saturating_sub(l);
            if area.area() == 0 {
                break;
            }
        }
    }
}
