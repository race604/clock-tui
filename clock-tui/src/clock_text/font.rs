use tui::{buffer::Buffer, layout::Rect, style::Style};

use super::point::Point;

pub mod bricks;

pub(crate) trait Font {
    fn size(&self) -> Point;
    fn render(&self, char: char, style: Style, area: Rect, buf: &mut Buffer);
}
