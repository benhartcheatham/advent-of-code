use crate::utils::coord::*;
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    ops::{Add, Sub},
    str::FromStr,
};

use super::direction::Direction;

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

impl From<GridCoord> for (usize, usize) {
    fn from(value: GridCoord) -> Self {
        (value.x, value.y)
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

pub fn in_bounds<T>(grid: &Vec<Vec<T>>, coord: GridCoord) -> bool {
    let (x, y) = coord.into();

    if grid.is_empty() {
        return false;
    }

    x < grid.len() && y < grid[0].len()
}

pub fn in_ibounds<T>(grid: &Vec<Vec<T>>, coord: Coord) -> bool {
    let (x, y) = coord.into();

    if grid.is_empty() || x < 0 || y < 0 {
        return false;
    }

    (x as usize) < grid.len() && (y as usize) < grid[0].len()
}

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeapElem {
    coord: GridCoord,
    data: i64,
}

impl HeapElem {
    fn new(data: i64, coord: GridCoord) -> Self {
        HeapElem { data, coord }
    }
}

impl Ord for HeapElem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .data
            .cmp(&self.data)
            .then_with(|| self.coord.cmp(&other.coord))
    }
}

impl PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub type DjikstraCostFn<T> = fn(&[Vec<T>], GridCoord, GridCoord) -> i64;

/// Takes in a starting and ending point and returns the shortest path between them
#[allow(unused)]
pub fn djikstra<T: Copy + Clone + Ord + Debug>(
    grid: &Vec<Vec<T>>,
    start: GridCoord,
    end: GridCoord,
    func: DjikstraCostFn<T>,
) -> Vec<GridCoord> {
    use GridDirection::*;

    let mut queue = BinaryHeap::new();
    let mut visited = Vec::new();
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();

    for (i, r) in grid.iter().enumerate() {
        for j in 0..r.len() {
            dist.insert(GridCoord::new(i, j), i64::MAX);
            prev.insert(GridCoord::new(i, j), GridCoord::new(usize::MAX, usize::MAX));
        }
    }

    if let Some(v) = dist.get_mut(&start) {
        *v = 0;
    }

    let directions = [Up, Down, Left, Right];

    queue.push(HeapElem::new(0, start));
    while !queue.is_empty() {
        let u = queue.pop().unwrap();
        visited.push(u.coord);

        if u.coord == end {
            break;
        }

        // TODO: Verify this isn't broken, I had to replace the .translate() method
        for d in &directions {
            let dc: Coord = (*d).into();
            let v = if dc.get_x() < 0 {
                u.coord.saturating_sub(GridCoord::new(1, 0))
            } else {
                u.coord + GridCoord::new(1, 0)
            };
            let v = if dc.get_y() < 0 {
                v.saturating_sub(GridCoord::new(0, 1))
            } else {
                v.saturating_add(GridCoord::new(0, 1))
            };

            if visited.contains(&v) || !in_bounds(grid, v) {
                continue;
            }

            let alt = dist[&u.coord]
                .checked_add((func)(grid, u.coord, v))
                .unwrap_or(i64::MAX);
            if alt < dist[&v] {
                *dist.get_mut(&v).unwrap() = alt;
                *prev.get_mut(&v).unwrap() = u.coord;
                queue.push(HeapElem::new(alt, v));
            }
        }
    }

    let mut path = Vec::new();
    if let Some(u) = prev.get(&end) {
        if start == end || *u == GridCoord::new(usize::MAX, usize::MAX) {
            return path;
        }
    } else {
        return path;
    }

    let mut u = end;
    while u != start && prev.contains_key(&u) {
        path.push(*prev.get(&u).unwrap());
        u = *prev.get(&u).unwrap();
    }

    path
}
