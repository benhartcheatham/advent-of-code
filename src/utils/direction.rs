use crate::utils::coord::*;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    N,
    S,
    W,
    NW,
    SW,
    E,
    SE,
    NE,
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
