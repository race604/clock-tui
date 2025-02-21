use ratatui::{style::Style, widgets::Widget};

use crate::clock_text::font::Font;

pub mod font;
pub mod point;

#[derive(Clone)]
pub struct ClockText<'a> {
    pub text: String,
    pub font: &'a dyn Font,
    pub style: Style,
}

impl<'a> ClockText<'a> {
    pub fn new(text: String, font: &'a dyn Font, style: Style) -> ClockText<'a> {
        ClockText { text, font, style }
    }
    pub fn size(&self) -> (u16, u16) {
        let width = self.text.chars().count() as u16 * self.font.get_char_width();
        let height = self.font.get_char_height();
        (width, height)
    }
}

impl<'a> Widget for ClockText<'a> {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        self.font.draw_str(&self.text, area, self.style, buf);
    }
}
