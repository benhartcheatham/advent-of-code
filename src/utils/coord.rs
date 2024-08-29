use std::ops::{Add, Sub};

use crate::utils::Direction;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UCoord {
    x: usize,
    y: usize,
}

#[allow(unused)]
impl UCoord {
    pub fn new(x: usize, y: usize) -> Self {
        UCoord { x, y }
    }

    pub fn translate(&self, direction: Direction, size: Option<usize>) -> Self {
        use Direction::*;

        let mut x = self.x;
        let mut y = self.y;

        (x, y) = match direction {
            Up => (x.saturating_sub(size.unwrap_or(1)), y),
            Down => (x.saturating_add(size.unwrap_or(1)), y),
            Left => (x, y.saturating_sub(size.unwrap_or(1))),
            Right => (x, y.saturating_add(size.unwrap_or(1))),
        };

        UCoord { x, y }
    }

    pub fn check_bounds(&self, x_bound: usize, y_bound: usize) -> bool {
        self.x < x_bound && self.y < y_bound
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn unit(&self) -> UCoord {
        let x = if self.x != 0 { 1 } else { 0 };
        let y = if self.y != 0 { 1 } else { 0 };

        UCoord::new(x, y)
    }

    pub fn as_signed(&self) -> Option<Coord> {
        if (self.x as i64) < 0 || (self.y as i64) < 0 {
            None
        } else {
            Some(Coord::new(self.x as i64, self.y as i64))
        }
    }
}

impl std::fmt::Display for UCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::cmp::Ord for UCoord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        f64::sqrt((self.x.pow(2) + self.y.pow(2)) as f64)
            .total_cmp(&f64::sqrt((other.x.pow(2) + other.y.pow(2)) as f64))
    }
}

impl std::cmp::PartialOrd for UCoord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<UCoord> for (usize, usize) {
    fn from(value: UCoord) -> Self {
        (value.x, value.y)
    }
}

impl Sub for UCoord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        let c = Coord::new(self.x as i64, self.y as i64);
        c.sub(Coord::new(rhs.x as i64, rhs.y as i64))
    }
}

impl Add for UCoord {
    type Output = UCoord;

    fn add(self, rhs: Self) -> Self::Output {
        UCoord::new(self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    x: i64,
    y: i64,
}

#[allow(unused)]
impl Coord {
    pub fn new(x: i64, y: i64) -> Self {
        Coord { x, y }
    }

    pub fn translate(&self, direction: Direction, size: Option<i64>) -> Self {
        use Direction::*;

        let mut x = self.x;
        let mut y = self.y;

        (x, y) = match direction {
            Up => (x.saturating_sub(size.unwrap_or(1)), y),
            Down => (x.saturating_add(size.unwrap_or(1)), y),
            Left => (x, y.saturating_sub(size.unwrap_or(1))),
            Right => (x, y.saturating_add(size.unwrap_or(1))),
        };

        Coord { x, y }
    }

    pub fn check_bounds(&self, x_lower: i64, x_upper: i64, y_lower: i64, y_upper: i64) -> bool {
        self.x < x_upper && self.y < y_upper && self.x > x_lower && self.y > y_lower
    }

    pub fn get_x(&self) -> i64 {
        self.x
    }

    pub fn get_y(&self) -> i64 {
        self.y
    }

    pub fn unit(&self) -> Coord {
        Coord::new(self.x.signum(), self.y.signum())
    }

    pub fn as_unsigned(&self) -> Option<UCoord> {
        if self.x < 0 || self.y < 0 {
            None
        } else {
            Some(UCoord::new(self.x as usize, self.y as usize))
        }
    }

    pub fn as_direction(&self) -> Option<Direction> {
        let unit: (i64, i64) = self.unit().into();

        match unit {
            (-1, 0) => Some(Direction::Up),
            (1, 0) => Some(Direction::Down),
            (0, -1) => Some(Direction::Left),
            (0, 1) => Some(Direction::Right),
            _ => None,
        }
    }

    pub fn manhattan(&self, other: &Coord) -> i64 {
        i64::abs(self.x.saturating_sub(other.x)) + i64::abs(self.y.saturating_sub(other.y))
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl std::cmp::Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        f64::sqrt((self.x.pow(2) + self.y.pow(2)) as f64)
            .total_cmp(&f64::sqrt((other.x.pow(2) + other.y.pow(2)) as f64))
    }
}

impl std::cmp::PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Coord> for (i64, i64) {
    fn from(value: Coord) -> Self {
        (value.x, value.y)
    }
}

impl From<Direction> for Coord {
    fn from(value: Direction) -> Self {
        use Direction::*;

        match value {
            Up => Coord::new(-1, 0),
            Down => Coord::new(1, 0),
            Left => Coord::new(0, -1),
            Right => Coord::new(0, 1),
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord::new(self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y))
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord::new(self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y))
    }
}
