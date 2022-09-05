use std::cmp::min;

use tui::{buffer::Buffer, layout::Rect, style::Style};

use super::Font;
use crate::clock_text::point::Point;

pub struct Bricks {
    pub size: u16,
}

impl Bricks {
    const UNIT_SIZE: Point = Point(6, 5);

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
        area: &Rect,
        buf: &mut Buffer,
    ) {
        let mut p = start;
        let mut on = false;
        for len in row {
            let len = len * size;
            if p.0 > area.right() {
                break;
            }

            if on {
                let s = min(len, area.right() - p.0 + 1);
                let line = "█".repeat(s as usize);
                for r in 0..size {
                    if p.1 > area.bottom() {
                        break;
                    }
                    buf.set_string(p.0, p.1 + r, line.as_str(), style);
                }
            }

            p.0 += len;
            on = !on;
        }
    }

    fn draw_matrix(mat: [Vec<u16>; 5], size: u16, style: Style, area: &Rect, buf: &mut Buffer) {
        let mut start = Point(area.x, area.y);
        for row in mat {
            Self::draw_row(start, row, size, style, area, buf);
            start.1 += size;
        }
    }
}

impl Font for Bricks {
    fn size(&self) -> Point {
        Self::UNIT_SIZE * self.size
    }

    fn render(&self, char: char, style: Style, area: Rect, buf: &mut Buffer) {
        let size = self.size;
        let mut render_matrix = |mat: [Vec<u16>; 5]| {
            Bricks::draw_matrix(mat, size, style, &area, buf);
        };

        match char {
            '0' => render_matrix([
                vec![0, 6],
                vec![0, 2, 2, 2],
                vec![0, 2, 2, 2],
                vec![0, 2, 2, 2],
                vec![0, 6],
            ]),
            '1' => render_matrix([vec![0, 4], vec![2, 2], vec![2, 2], vec![2, 2], vec![0, 6]]),
            '2' => render_matrix([vec![0, 6], vec![4, 2], vec![0, 6], vec![0, 2], vec![0, 6]]),
            '3' => render_matrix([vec![0, 6], vec![4, 2], vec![0, 6], vec![4, 2], vec![0, 6]]),
            '4' => render_matrix([
                vec![0, 2, 2, 2],
                vec![0, 2, 2, 2],
                vec![0, 6],
                vec![4, 2],
                vec![4, 2],
            ]),
            '5' => render_matrix([vec![0, 6], vec![0, 2], vec![0, 6], vec![4, 2], vec![0, 6]]),
            '6' => render_matrix([
                vec![0, 6],
                vec![0, 2],
                vec![0, 6],
                vec![0, 2, 2, 2],
                vec![0, 6],
            ]),
            '7' => render_matrix([vec![0, 6], vec![4, 2], vec![4, 2], vec![4, 2], vec![4, 2]]),
            '8' => render_matrix([
                vec![0, 6],
                vec![0, 2, 2, 2],
                vec![0, 6],
                vec![0, 2, 2, 2],
                vec![0, 6],
            ]),
            '9' => render_matrix([
                vec![0, 6],
                vec![0, 2, 2, 2],
                vec![0, 6],
                vec![4, 2],
                vec![0, 6],
            ]),
            ':' => render_matrix([vec![], vec![2, 2], vec![], vec![2, 2], vec![]]),
            '.' => render_matrix([vec![], vec![], vec![], vec![], vec![2, 2]]),
            '-' => render_matrix([vec![], vec![], vec![0, 6], vec![], vec![]]),
            _ => {}
        }
    }
}
