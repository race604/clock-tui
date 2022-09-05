use tui::{buffer::Buffer, layout::Rect, style::Style};

pub mod bricks;

pub(crate) trait Font {
    fn render(&self, char: char, size: u16, style: Style, area: Rect, buf: &mut Buffer);
}
