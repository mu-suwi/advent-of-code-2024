// i don't know how many more of these problems are going to involve 2D grids

use std::ops;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<isize> for Vec2 {
    type Output = Vec2;

    fn mul(self, factor: isize) -> Vec2 {
        Vec2 {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl Vec2 {
    // adding two vectors together is the same as
    // applying an offset to some absolute coordinates.
    pub fn offset_by(self, offset: Self) -> Self {
        self + offset
    }

    // subtracting your own coordinates from someone else's
    // returns position of other relative to self.
    pub fn get_offset(self, other: Self) -> Self {
        other - self
    }

    // multiplying each number in a vector by -1
    // is the same as rotating 180 degrees.
    pub fn invert(self) -> Self {
        self * -1
    }

    // part 2: multiplying all numbers in a vector by the same number
    // is the same as multiplying its magnitude.
    pub fn multiply(self, factor: isize) -> Self {
        self * factor
    }

    pub fn taxi_dist(self) -> usize {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}
