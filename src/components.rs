pub type Distance = i32;

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: Distance,
    pub y: Distance,
}

// i32 is convenient for inflicting damages > health
// checks cane be done using health >= 0
#[derive(Debug)]
pub struct Health {
    pub value: i32,
    pub max: i32,
}

// distance / simulation step
#[derive(Debug)]
pub struct Speed(pub Distance);

// Raw damage
#[derive(Debug)]
pub struct Damage(pub i32);

// distance <= range => unit is at range
#[derive(Debug)]
pub struct Range(pub Distance);

#[derive(Debug)]
pub struct Score(pub i32);

#[derive(Debug)]
pub struct Target {
    pub(crate) position: Option<Position>,
}
