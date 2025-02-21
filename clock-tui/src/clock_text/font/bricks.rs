use std::cmp::min;

use ratatui::{buffer::Buffer, layout::Rect, style::Style};

use crate::clock_text::point::Point;

use super::Font;

pub struct BricksFont {
    pub size: u16,
}

impl BricksFont {
    pub fn new(size: u16) -> Self {
        Self { size }
    }

    /// each row is represented with a vector of numbers:
    ///   the odd indexed items represent the lenght of "off",
    ///   the even indexed items represent the lenght of "on".
    /// For exmaple:
    ///   vec![0, 6] is  "██████"
    ///   vec![2, 2] is  "  ██"
    ///   vec![0, 2, 2, 2] is  "██  ██"
    fn draw_row(
        start: Point,
        row: Vec<u16>,
        size: u16,
        style: Style,
        area_right: u16,
        area_bottom: u16,
        buf: &mut Buffer,
    ) {
        let mut p = start;
        let mut on = false;
        for len in row {
            let len = len * size;
            if p.0 > area_right {
                break;
            }

            if on {
                let s = std::cmp::min(len, area_right - p.0 + 1);
                let line = "█".repeat(s as usize);
                for r in 0..size {
                    if p.1 > area_bottom {
                        break;
                    }
                    buf.set_string(p.0, p.1 + r, line.as_str(), style);
                }
            }

            p.0 += len;
            on = !on;
        }
    }

    fn get_char_matrix(c: char) -> Option<[Vec<u16>; 5]> {
        match c {
            '0' => Some([
                vec![0, 6],
                vec![0, 2, 2, 2],
                vec![0, 2, 2, 2],
                vec![0, 2, 2, 2],
                vec![0, 6],
            ]),
            '1' => Some([vec![0, 4], vec![2, 2], vec![2, 2], vec![2, 2], vec![0, 6]]),
            '2' => Some([vec![0, 6], vec![4, 2], vec![0, 6], vec![0, 2], vec![0, 6]]),
            '3' => Some([vec![0, 6], vec![4, 2], vec![0, 6], vec![4, 2], vec![0, 6]]),
            '4' => Some([
                vec![0, 2, 2, 2],
                vec![0, 2, 2, 2],
                vec![0, 6],
                vec![4, 2],
                vec![4, 2],
            ]),
            '5' => Some([vec![0, 6], vec![0, 2], vec![0, 6], vec![4, 2], vec![0, 6]]),
            '6' => Some([
                vec![0, 6],
                vec![0, 2],
                vec![0, 6],
                vec![0, 2, 2, 2],
                vec![0, 6],
            ]),
            '7' => Some([vec![0, 6], vec![4, 2], vec![4, 2], vec![4, 2], vec![4, 2]]),
            '8' => Some([
                vec![0, 6],
                vec![0, 2, 2, 2],
                vec![0, 6],
                vec![0, 2, 2, 2],
                vec![0, 6],
            ]),
            '9' => Some([
                vec![0, 6],
                vec![0, 2, 2, 2],
                vec![0, 6],
                vec![4, 2],
                vec![0, 6],
            ]),
            ':' => Some([vec![], vec![2, 2], vec![], vec![2, 2], vec![]]),
            '.' => Some([vec![], vec![], vec![], vec![], vec![2, 2]]),
            '-' => Some([vec![], vec![], vec![0, 6], vec![], vec![]]),
            _ => None,
        }
    }
}

impl Font for BricksFont {
    fn get_char(&self, c: char) -> Option<&[Point]> {
        None // We don't use points for BricksFont
    }

    fn get_char_width(&self) -> u16 {
        6 * self.size
    }

    fn get_char_height(&self) -> u16 {
        5 * self.size
    }

    fn draw_char(&self, c: char, x: u16, y: u16, style: Style, buf: &mut Buffer) {
        if let Some(matrix) = Self::get_char_matrix(c) {
            let mut start = Point(x, y);
            let area_right = buf.area.right();
            let area_bottom = buf.area.bottom();
            for row in matrix {
                Self::draw_row(start, row, self.size, style, area_right, area_bottom, buf);
                start.1 += self.size;
            }
        }
    }
}
