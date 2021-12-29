use crate::Vec2;

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
pub struct Range(pub Distance);

#[derive(Debug)]
pub struct Score(pub i32);

#[derive(Debug)]
pub struct Target {
    pub(crate) position: Option<Position>,
}

#[derive(Debug)]
pub struct Waypoint {
    pub index: usize,
}
