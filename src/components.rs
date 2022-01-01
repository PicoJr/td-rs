use crate::Vec2;
use std::ops::{Add, Sub};

pub type Distance = i32;

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub x: Distance,
    pub y: Distance,
}

impl From<Vec2> for Position {
    fn from(v: Vec2) -> Self {
        Position {
            x: v.x as i32,
            y: v.y as i32,
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub for &Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Position {
    pub fn norm_squared(&self) -> Distance {
        self.x * self.x + self.y * self.y
    }
}

// i32 is convenient for inflicting damages > health
// checks cane be done using health >= 0
#[derive(Clone, Debug)]
pub struct Health {
    pub value: i32,
    pub max: i32,
}

// distance / simulation step
#[derive(Clone, Debug)]
pub struct Speed(pub Distance);

// Raw damage
#[derive(Clone, Debug)]
pub struct Damage(pub i32);

// distance <= range => unit is at range
#[derive(Clone, Debug)]
pub struct Range {
    pub squared: Distance,
}

#[derive(Clone, Debug)]
pub struct Score(pub i32);

#[derive(Debug)]
pub struct Target {
    pub(crate) position: Option<Position>,
}

#[derive(Debug)]
pub struct Waypoint {
    pub index: usize,
}
