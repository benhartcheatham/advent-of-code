use std::ops::{Add, Sub};

/// Cartesian coordinate type 
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    x: i64,
    y: i64,
}

// TODO: Implement .X notation for Coord (like with tuples)
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

    pub fn unit(&self) -> Coord {
        Coord::new(self.x.signum(), self.y.signum())
    }

    pub fn abs(&self) -> Coord {
        Self::new(self.x.abs(), self.y.abs())
    }

    pub fn saturating_add(&self, rhs: &Coord) -> Self {
        Self::new(self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y))
    }

    pub fn saturating_sub(&self, rhs: &Coord) -> Self {
        Self::new(self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y))
    }

    pub fn manhattan(&self, other: &Coord) -> i64 {
        i64::abs(self.x - other.x) + i64::abs(self.y - other.y)
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
