use std::{cmp::max, ops};
use tui::{buffer::Buffer, layout::Rect, style::Style};

pub(crate) trait BricksChar {
    fn render(&self, size: u16, style: Style, area: Rect, buf: &mut Buffer);
}

pub struct BrickChar(char);

impl BrickChar {
    const H_UNIT: u16 = 2;
    const V_UNIT: u16 = 1;
    const UNIT_SIZE: Point = Point(3 * Self::H_UNIT, 5 * Self::V_UNIT);

    pub(crate) fn size(size: u16) -> Point {
        Self::UNIT_SIZE.clone() * size
    }

    pub(crate) fn from(char: char) -> BrickChar {
        BrickChar(char)
    }

    pub(crate) fn render(&self, size: u16, style: Style, area: Rect, buf: &mut Buffer) {
        let char_size = BrickChar::size(size);
        match self.0 {
            '0'..='9' => Self::draw_digital(self.0, size, style, area, buf),
            ':' => {
                let start_x = area.x + size * Self::H_UNIT;
                let end_x = area.x + char_size.0 - size * Self::H_UNIT;
                let start_y = area.y + size * Self::V_UNIT;
                let start_y2 = area.y + (char_size.1 + size * Self::V_UNIT) / 2;
                let len = (char_size.1 - 3 * size * Self::V_UNIT) / 2;
                for x in (start_x..end_x).step_by((size * Self::H_UNIT) as usize) {
                    Self::draw_line(
                        size,
                        Point(x, start_y),
                        len,
                        LineDir::Vertical,
                        style,
                        &area,
                        buf,
                    );
                    Self::draw_line(
                        size,
                        Point(x, start_y2),
                        len,
                        LineDir::Vertical,
                        style,
                        &area,
                        buf,
                    );
                }
            }
            '-' => {
                let x = area.x;
                let y = area.y + (char_size.1 - size * Self::V_UNIT) / 2;
                Self::draw_line(
                    size,
                    Point(x, y),
                    char_size.1,
                    LineDir::Horizontal,
                    style,
                    &area,
                    buf,
                );
            }
            '.' => {
                let x = area.x + char_size.0 - size * Self::H_UNIT;
                let y = area.y + char_size.1 - size * Self::V_UNIT;
                Self::draw_line(
                    size,
                    Point(x, y),
                    size * Self::H_UNIT,
                    LineDir::Horizontal,
                    style,
                    &area,
                    buf,
                );
            }
            _ => {}
        }
    }

    fn draw_digital(d: char, size: u16, style: Style, area: Rect, buf: &mut Buffer) {
        let char_size = BrickChar::size(size);
        let mut draw_line =
            |x, y, len, dir| Self::draw_line(size, Point(x, y), len, dir, style, &area, buf);
        let x_start = area.x;
        let x_end = area.x + char_size.0 - size * Self::H_UNIT;
        let y_start = area.y;
        let y_end = area.y + char_size.1 - size * Self::V_UNIT;
        let y_center = area.y + (char_size.1 - size * Self::V_UNIT) / 2;
        let half_h = (char_size.1 + size * Self::V_UNIT) / 2;
        match d {
            '0' => {
                draw_line(x_start, y_start, half_h, LineDir::Vertical);
                draw_line(x_start, y_center, half_h, LineDir::Vertical);
                draw_line(x_end, y_start, half_h, LineDir::Vertical);
                draw_line(x_end, y_center, half_h, LineDir::Vertical);

                draw_line(x_start, y_start, char_size.0, LineDir::Horizontal);
                // draw_line(x_start, y_center, char_size.0, LineDir::Horizontal);
                draw_line(x_start, y_end, char_size.0, LineDir::Horizontal);
            }
            '1' => {
                // draw_line(x_start, y_start, half_h, LineDir::Vertical);
                // draw_line(x_start, y_center, half_h, LineDir::Vertical);
                draw_line(x_end, y_start, half_h, LineDir::Vertical);
                draw_line(x_end, y_center, half_h, LineDir::Vertical);

                // draw_line(x_start, y_start, char_size.0, LineDir::Horizontal);
                // draw_line(x_start, y_center, char_size.0, LineDir::Horizontal);
                // draw_line(x_start, y_end, char_size.0, LineDir::Horizontal);
            }
            '2' => {
                // draw_line(x_start, y_start, half_h, LineDir::Vertical);
                draw_line(x_start, y_center, half_h, LineDir::Vertical);
                draw_line(x_end, y_start, half_h, LineDir::Vertical);
                // draw_line(x_end, y_center, half_h, LineDir::Vertical);

                draw_line(x_start, y_start, char_size.0, LineDir::Horizontal);
                draw_line(x_start, y_center, char_size.0, LineDir::Horizontal);
                draw_line(x_start, y_end, char_size.0, LineDir::Horizontal);
            }
            '3' => {
                // draw_line(x_start, y_start, half_h, LineDir::Vertical);
                // draw_line(x_start, y_center, half_h, LineDir::Vertical);
                draw_line(x_end, y_start, half_h, LineDir::Vertical);
                draw_line(x_end, y_center, half_h, LineDir::Vertical);

                draw_line(x_start, y_start, char_size.0, LineDir::Horizontal);
                draw_line(x_start, y_center, char_size.0, LineDir::Horizontal);
                draw_line(x_start, y_end, char_size.0, LineDir::Horizontal);
            }
            '4' => {
                draw_line(x_start, y_start, half_h, LineDir::Vertical);
                // draw_line(x_start, y_center, half_h, LineDir::Vertical);
                draw_line(x_end, y_start, half_h, LineDir::Vertical);
                draw_line(x_end, y_center, half_h, LineDir::Vertical);

                // draw_line(x_start, y_start, char_size.0, LineDir::Horizontal);
                draw_line(x_start, y_center, char_size.0, LineDir::Horizontal);
                // draw_line(x_start, y_end, char_size.0, LineDir::Horizontal);
            }
            '5' => {
                draw_line(x_start, y_start, half_h, LineDir::Vertical);
                // draw_line(x_start, y_center, half_h, LineDir::Vertical);
                // draw_line(x_end, y_start, half_h, LineDir::Vertical);
                draw_line(x_end, y_center, half_h, LineDir::Vertical);

                draw_line(x_start, y_start, char_size.0, LineDir::Horizontal);
                draw_line(x_start, y_center, char_size.0, LineDir::Horizontal);
                draw_line(x_start, y_end, char_size.0, LineDir::Horizontal);
            }
            '6' => {
                draw_line(x_start, y_start, half_h, LineDir::Vertical);
                draw_line(x_start, y_center, half_h, LineDir::Vertical);
                // draw_line(x_end, y_start, half_h, LineDir::Vertical);
                draw_line(x_end, y_center, half_h, LineDir::Vertical);

                draw_line(x_start, y_start, char_size.0, LineDir::Horizontal);
                draw_line(x_start, y_center, char_size.0, LineDir::Horizontal);
                draw_line(x_start, y_end, char_size.0, LineDir::Horizontal);
            }
            '7' => {
                // draw_line(x_start, y_start, half_h, LineDir::Vertical);
                // draw_line(x_start, y_center, half_h, LineDir::Vertical);
                draw_line(x_end, y_start, half_h, LineDir::Vertical);
                draw_line(x_end, y_center, half_h, LineDir::Vertical);

                draw_line(x_start, y_start, char_size.0, LineDir::Horizontal);
                // draw_line(x_start, y_center, char_size.0, LineDir::Horizontal);
                // draw_line(x_start, y_end, char_size.0, LineDir::Horizontal);
            }
            '8' => {
                draw_line(x_start, y_start, half_h, LineDir::Vertical);
                draw_line(x_start, y_center, half_h, LineDir::Vertical);
                draw_line(x_end, y_start, half_h, LineDir::Vertical);
                draw_line(x_end, y_center, half_h, LineDir::Vertical);

                draw_line(x_start, y_start, char_size.0, LineDir::Horizontal);
                draw_line(x_start, y_center, char_size.0, LineDir::Horizontal);
                draw_line(x_start, y_end, char_size.0, LineDir::Horizontal);
            }
            '9' => {
                draw_line(x_start, y_start, half_h, LineDir::Vertical);
                // draw_line(x_start, y_center, half_h, LineDir::Vertical);
                draw_line(x_end, y_start, half_h, LineDir::Vertical);
                draw_line(x_end, y_center, half_h, LineDir::Vertical);

                draw_line(x_start, y_start, char_size.0, LineDir::Horizontal);
                draw_line(x_start, y_center, char_size.0, LineDir::Horizontal);
                draw_line(x_start, y_end, char_size.0, LineDir::Horizontal);
            }
            _ => {}
        }
    }

    fn draw_line(
        size: u16,
        start: Point,
        len: u16,
        dir: LineDir,
        style: Style,
        area: &Rect,
        buf: &mut Buffer,
    ) {
        let step = match dir {
            LineDir::Horizontal => Point(Self::H_UNIT, 0),
            LineDir::Vertical => Point(0, Self::V_UNIT),
        };

        let line = match dir {
            LineDir::Horizontal => Point(0, Self::V_UNIT),
            LineDir::Vertical => Point(Self::H_UNIT, 0),
        };

        let mut from = start;
        for _ in 0..size {
            let mut p = from;
            for _ in (0..len).step_by(max(step.0, step.1).into()) {
                if !p.in_area(&area) {
                    break;
                }
                // println!("p = {:?} area = {:?}", p, area);
                buf.get_mut(p.0, p.1).set_symbol("██").set_style(style);
                p = p + &step;
            }
            from = from + &line;
        }
    }
}

enum LineDir {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Point(pub u16, pub u16);

impl Point {
    pub(crate) fn in_area(&self, area: &Rect) -> bool {
        area.left() <= self.0
            && self.0 < area.right()
            && area.top() <= self.1
            && self.1 < area.bottom()
    }
}

impl ops::Add<&Point> for Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl ops::Mul<u16> for Point {
    type Output = Point;

    fn mul(self, other: u16) -> Point {
        Point(self.0 * other, self.1 * other)
    }
}
