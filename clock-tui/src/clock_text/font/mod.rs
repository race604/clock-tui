pub mod bricks;

use ratatui::{buffer::Buffer, layout::Rect, style::Style};

use super::point::Point;

pub trait Font {
    fn get_char(&self, c: char) -> Option<&[Point]>;
    fn get_char_width(&self) -> u16;
    fn get_char_height(&self) -> u16;

    fn draw_char(&self, c: char, x: u16, y: u16, style: Style, buf: &mut Buffer) {
        if let Some(points) = self.get_char(c) {
            for point in points {
                let x = x + point.0;
                let y = y + point.1;
                if x < buf.area.right() && y < buf.area.bottom() {
                    buf.get_mut(x, y).set_style(style);
                }
            }
        }
    }

    fn draw_str(&self, s: &str, area: Rect, style: Style, buf: &mut Buffer) {
        let mut x = area.x;
        let y = area.y;
        let spacing = 2;
        for c in s.chars() {
            if x + self.get_char_width() > area.right() {
                break;
            }
            self.draw_char(c, x, y, style, buf);
            x += self.get_char_width() + spacing;
        }
    }
}
