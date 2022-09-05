use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point(pub u16, pub u16);

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
