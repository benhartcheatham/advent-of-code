use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Cartesian coordinate type with some utility functions
/// and traits
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

    pub fn get_x(&self) -> i64 {
        self.x
    }

    pub fn get_y(&self) -> i64 {
        self.y
    }

    /// Gets a new Coord where each inner coordinate is
    /// reduced to either -1, 0, or 1.
    pub fn unit(&self) -> Self {
        Coord::new(self.x.signum(), self.y.signum())
    }

    /// Gets a new Coord where each inner coordinate is
    /// the absolute value of this Coord
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    pub fn saturating_add(&self, rhs: &Self) -> Self {
        Self::new(self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y))
    }

    pub fn saturating_sub(&self, rhs: &Self) -> Self {
        Self::new(self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y))
    }

    /// Gets the manhattan distance between this Coord and @other
    pub fn manhattan(&self, other: &Self) -> i64 {
        i64::abs(self.x - other.x) + i64::abs(self.y - other.y)
    }

    /// Gets the cartesian distance between this Coord and @other
    pub fn distance(&self, other: &Self) -> f64 {
        f64::sqrt(((self.x - other.x) as f64).powf(2.0) + ((self.y - other.y) as f64).powf(2.0))
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

/// Fails when Coord has a negative value
impl From<Coord> for Option<(usize, usize)> {
    fn from(value: Coord) -> Self {
        if value.x < 0 || value.y < 0 {
            None
        } else {
            Some((value.x as usize, value.y as usize))
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
