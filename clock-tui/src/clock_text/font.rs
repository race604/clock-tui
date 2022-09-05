use tui::{style::Style, layout::Rect, buffer::Buffer};

pub mod bricks;

pub(crate) trait Font {
	fn render(&self, char: char, size: u16, style: Style, area: Rect, buf: &mut Buffer);
}
