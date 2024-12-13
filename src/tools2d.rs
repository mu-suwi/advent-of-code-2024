#![allow(dead_code)]

// i don't know how many more of these problems are going to involve 2D grids
// so maybe a little helper module will help me avoid reimplementing Vec2 every time...

use std::ops;

pub const COMPASS: [Vec2; 4] = [
    Vec2 { x: 0, y: -1 },
    Vec2 { x: 1, y: 0 },
    Vec2 { x: 0, y: 1 },
    Vec2 { x: -1, y: 0 },
];

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

// impl<T> Vec2
// where
//     for<'a> &'a T: Sized,
// {
//     pub fn bounds(other: &[Vec<T>]) -> Self
//     where
//         T: Sized,
//     {
//         Vec2 {
//             x: other[0].len() as isize,
//             y: other.len() as isize,
//         }
//     }
// }
