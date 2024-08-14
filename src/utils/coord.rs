use crate::utils::Direction;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    x: usize,
    y: usize,
}

#[allow(unused)]
impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
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

        Coord { x, y }
    }

    pub fn check_bounds(&self, x_bound: usize, y_bound: usize) -> bool {
        self.x < x_bound && self.y < y_bound
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

impl From<Coord> for (usize, usize) {
    fn from(value: Coord) -> Self {
        (value.x, value.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ICoord {
    x: i64,
    y: i64,
}

#[allow(unused)]
impl ICoord {
    pub fn new(x: i64, y: i64) -> Self {
        ICoord { x, y }
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

        ICoord { x, y }
    }

    pub fn check_bounds(&self, x_lower: i64, x_upper: i64, y_lower: i64, y_upper: i64) -> bool {
        self.x < x_upper && self.y < y_upper && self.x > x_lower && self.y > y_lower
    }
}

impl std::fmt::Display for ICoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl std::cmp::Ord for ICoord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        f64::sqrt((self.x.pow(2) + self.y.pow(2)) as f64)
            .total_cmp(&f64::sqrt((other.x.pow(2) + other.y.pow(2)) as f64))
    }
}

impl std::cmp::PartialOrd for ICoord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<ICoord> for (i64, i64) {
    fn from(value: ICoord) -> Self {
        (value.x, value.y)
    }
}
