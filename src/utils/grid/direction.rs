use std::str::FromStr;

use crate::utils::{coord::Coord, direction::Direction};

#[allow(unused)]
pub const DIRECTIONS: [GridDirection; 4] = [
    GridDirection::Up,
    GridDirection::Down,
    GridDirection::Left,
    GridDirection::Right,
];

#[derive(Debug, Clone, Copy)]
pub enum GridDirection {
    Up,
    Down,
    Left,
    Right,
}

impl From<GridDirection> for i64 {
    fn from(value: GridDirection) -> Self {
        use GridDirection::*;

        match value {
            Up | Left => -1,
            Down | Right => 1,
        }
    }
}

impl From<GridDirection> for Coord {
    fn from(value: GridDirection) -> Self {
        use GridDirection::*;

        match value {
            Up => Coord::new(0, -1),
            Down => Coord::new(0, 1),
            Left => Coord::new(-1, 0),
            Right => Coord::new(1, 0),
        }
    }
}

impl From<GridDirection> for Direction {
    fn from(value: GridDirection) -> Self {
        use GridDirection::*;

        match value {
            Up => Direction::N,
            Down => Direction::S,
            Left => Direction::W,
            Right => Direction::E,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DirectionParseError;

impl FromStr for GridDirection {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GridDirection::*;

        match s.to_lowercase().as_str() {
            "l" | "left" => Ok(Left),
            "r" | "right" => Ok(Right),
            "u" | "up" => Ok(Up),
            "d" | "down" => Ok(Down),
            _ => Err(DirectionParseError),
        }
    }
}

impl GridDirection {
    pub fn invert(&self) -> GridDirection {
        use GridDirection::*;

        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}
