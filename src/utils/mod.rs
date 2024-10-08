use std::str::FromStr;

pub mod coord;
pub mod grid;
pub mod graph;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for u64 {
    fn from(value: Direction) -> Self {
        use Direction::*;

        match value {
            Up => 1,
            Down => 2,
            Left => 3,
            Right => 4,
        }
    }
}

impl From<Direction> for i64 {
    fn from(value: Direction) -> Self {
        use Direction::*;

        match value {
            Up | Left => -1,
            Down | Right => 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DirectionParseError;

impl FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;

        match s.to_lowercase().as_str() {
            "l" | "left" => Ok(Left),
            "r" | "right" => Ok(Right),
            "u" | "up" => Ok(Up),
            "d" | "down" => Ok(Down),
            _ => Err(DirectionParseError),
        }
    }
}

impl Direction {
    pub fn invert(&self) -> Direction {
        use Direction::*;

        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}
