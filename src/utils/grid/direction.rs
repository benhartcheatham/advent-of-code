use std::str::FromStr;

use crate::{coord::Coord, direction::Direction};

#[allow(unused)]
pub const DIRECTIONS: [GridDirection; 4] = [
    GridDirection::Up,
    GridDirection::Down,
    GridDirection::Left,
    GridDirection::Right,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridDirection {
    Up,
    Down,
    Left,
    Right,
}

impl GridDirection {
    pub fn invert(&self) -> Self {
        use GridDirection::*;

        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    pub fn rotate_right(&self) -> Self {
        use GridDirection::*;

        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn rotate_left(&self) -> Self {
        use GridDirection::*;

        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
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
            Up => Coord::new(-1, 0),
            Down => Coord::new(1, 0),
            Left => Coord::new(0, -1),
            Right => Coord::new(0, 1),
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
