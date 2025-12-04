use crate::coord::*;

#[allow(unused)]
pub const DIRECTIONS: [Direction; 8] = [
    Direction::N,
    Direction::S,
    Direction::E,
    Direction::W,
    Direction::NE,
    Direction::NW,
    Direction::SE,
    Direction::SW,
];

#[allow(unused)]
pub const NEWS: [Direction; 4] = [
    Direction::N,
    Direction::E,
    Direction::W,
    Direction::S,
];

#[allow(unused)]
pub const DIAGS: [Direction; 4] = [
    Direction::NE,
    Direction::NW,
    Direction::SE,
    Direction::SW,
];

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

#[allow(unused)]
impl Direction {
    pub fn invert(&self) -> Self {
        use Direction::*;

        match &self {
            N => S,
            E => W,
            S => N,
            W => E,
            NE => SW,
            NW => SE,
            SE => NW,
            SW => NE,
        }
    }

    pub fn rotate_right(&self) -> Self {
        use Direction::*;

        match &self {
            N => NE,
            NE => E,
            E => SE,
            SE => S,
            S => SW,
            SW => W,
            W => NW,
            NW => N,
        }
    }

    pub fn rotate_left(&self) -> Self {
        use Direction::*;

        match &self {
            N => NW,
            NW => W,
            W => SW,
            SW => S,
            S => SE,
            SE => E,
            E => NE,
            NE => N,
        }
    }
}

impl From<Direction> for Coord {
    fn from(value: Direction) -> Self {
        use Direction::*;

        match value {
            N => Coord::new(0, 1),
            E => Coord::new(1, 0),
            S => Coord::new(0, -1),
            W => Coord::new(-1, 0),
            NE => Coord::new(1, 1),
            NW => Coord::new(-1, 1),
            SE => Coord::new(1, -1),
            SW => Coord::new(-1, -1),
        }
    }
}

impl From<Direction> for (i64, i64) {
    fn from(value: Direction) -> Self {
        Into::<Coord>::into(value).into()
    }
}
