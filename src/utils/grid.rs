use crate::utils::coord::*;
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

#[allow(unused)]
pub struct Grid<T> {
    cells: Vec<Vec<T>>,
    traversal_fn: fn(&Grid<T>, Coord, Coord) -> i64,
}

#[allow(unused)]
impl<T: Copy + Clone> Grid<T> {
    /// traversal is a function that, given two adjacent points (v, u), computes the
    /// traversal cost from cells[v] to cells[u]
    pub fn new(cells: Vec<Vec<T>>, traversal: fn(&Grid<T>, Coord, Coord) -> i64) -> Self {
        Grid {
            cells,
            traversal_fn: traversal,
        }
    }

    pub fn get(&self, coord: Coord) -> T {
        let (x, y) = coord.into();
        self.cells[x][y]
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..(self.cells.len() - 1) {
            writeln!(f, "{:?}", self.cells[r])?;
        }

        if !self.cells.is_empty() {
            write!(f, "{:?}", self.cells[self.cells.len() - 1])
        } else {
            write!(f, "[]")
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeapElem {
    coord: Coord,
    data: i64,
}

#[allow(unused)]
impl HeapElem {
    fn new(data: i64, coord: Coord) -> Self {
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

#[allow(unused)]
impl<T: Copy + Clone + Ord + Debug> Grid<T> {
    /// Takes in a starting and ending point and returns the shortest path between them
    pub fn djikstra(&self, start: Coord, end: Coord) -> Vec<Coord> {
        use crate::utils::Direction::*;

        let mut queue = BinaryHeap::new();
        let mut visited = Vec::new();
        let mut dist = HashMap::new();
        let mut prev = HashMap::new();

        for (i, r) in self.cells.iter().enumerate() {
            for (j, v) in r.iter().enumerate() {
                dist.insert(Coord::new(i, j), i64::MAX);
                prev.insert(Coord::new(i, j), Coord::new(usize::MAX, usize::MAX));
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

            for d in &directions {
                let v = u.coord.translate(*d, None);

                if visited.contains(&v) || !v.check_bounds(self.cells.len(), self.cells[0].len()) {
                    continue;
                }

                let alt = dist[&u.coord]
                    .checked_add((self.traversal_fn)(self, u.coord, v))
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
            if start == end || *u == Coord::new(usize::MAX, usize::MAX) {
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
}
