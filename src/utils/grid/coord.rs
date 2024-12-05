use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::coord::Coord;

/// Coordinate system for Grids (a.k.a Vec<Vec<T>>) with some
/// utility functions and traits
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GridCoord {
    x: usize,
    y: usize,
}

#[allow(unused)]
impl GridCoord {
    pub fn new(x: usize, y: usize) -> Self {
        GridCoord { x, y }
    }

    pub fn mult_scalar(&self, size: usize) -> Self {
        Self::new(self.x * size, self.y * size)
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn unit(&self) -> GridCoord {
        let x = if self.x != 0 { 1 } else { 0 };
        let y = if self.y != 0 { 1 } else { 0 };

        GridCoord::new(x, y)
    }

    pub fn saturating_sub(&self, rhs: GridCoord) -> Self {
        Self::new(self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y))
    }

    pub fn saturating_add(&self, rhs: GridCoord) -> Self {
        Self::new(self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y))
    }

    pub fn from_coord(coord: Coord) -> Option<Self> {
        if coord.get_x() < 0 || coord.get_y() < 0 {
            None
        } else {
            Some(Self::new(coord.get_x() as usize, coord.get_y() as usize))
        }
    }
}

impl std::fmt::Display for GridCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::cmp::Ord for GridCoord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        f64::sqrt((self.x.pow(2) + self.y.pow(2)) as f64)
            .total_cmp(&f64::sqrt((other.x.pow(2) + other.y.pow(2)) as f64))
    }
}

impl std::cmp::PartialOrd for GridCoord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Only fails when the GridCoord has a coordinate larger than i64::MAX
impl From<GridCoord> for Option<Coord> {
    fn from(value: GridCoord) -> Self {
        if value.x > i64::MAX as usize || value.y > i64::MAX as usize {
            None
        } else {
            Some(Coord::new(value.x as i64, value.y as i64))
        }
    }
}

impl From<GridCoord> for (usize, usize) {
    fn from(value: GridCoord) -> Self {
        (value.x, value.y)
    }
}

/// Only fails when the GridCoord has a coordinate larger than i64::MAX
impl From<GridCoord> for Option<(i64, i64)> {
    fn from(value: GridCoord) -> Self {
        if value.x > i64::MAX as usize || value.y > i64::MAX as usize {
            None
        } else {
            Some((value.x as i64, value.y as i64))
        }
    }
}

impl Sub for GridCoord {
    type Output = GridCoord;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add for GridCoord {
    type Output = GridCoord;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for GridCoord {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl SubAssign for GridCoord {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
