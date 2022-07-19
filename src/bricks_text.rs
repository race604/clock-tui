use tui::{style::Style, widgets::Widget};

use self::chars::{BrickChar, Point};

mod chars;

pub struct BricksText {
    text: String,
    size: u16,
    space: u16,
    style: Style,
}

impl BricksText {
    pub fn new(text: &str, size: u16, space: u16, style: Style) -> BricksText {
        BricksText {
            text: text.to_string(),
            size,
            space,
            style,
        }
    }

    pub fn size(&self) -> (u16, u16) {
        let Point(w, h) = BrickChar::size(self.size);
        let n_chars = self.text.chars().count() as u16;
        (w * n_chars + self.space * (n_chars - 1), h)
    }
}

impl Widget for &BricksText {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let mut area = area.clone();
        for char in self.text.chars() {
            let Point(w, _) = BrickChar::size(self.size);
            let char = BrickChar::from(char);
            char.render(self.size, self.style, area, buf);
            let l = w + self.space;
            area.x += l;
            area.width = area.width.saturating_sub(l);
            if area.area() == 0 {
                break;
            }
        }
    }
}
