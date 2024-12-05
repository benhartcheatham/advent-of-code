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
