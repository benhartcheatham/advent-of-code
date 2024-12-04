use crate::utils::coord::*;
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

pub type DjikstraCostFn<T> = fn(&[Vec<T>], UCoord, UCoord) -> i64;

pub fn in_bounds<T>(grid: &Vec<Vec<T>>, coord: UCoord) -> bool {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeapElem {
    coord: UCoord,
    data: i64,
}

impl HeapElem {
    fn new(data: i64, coord: UCoord) -> Self {
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

/// Takes in a starting and ending point and returns the shortest path between them
#[allow(unused)]
pub fn djikstra<T: Copy + Clone + Ord + Debug>(
    grid: &Vec<Vec<T>>,
    start: UCoord,
    end: UCoord,
    func: DjikstraCostFn<T>,
) -> Vec<UCoord> {
    use crate::utils::Direction::*;

    let mut queue = BinaryHeap::new();
    let mut visited = Vec::new();
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();

    for (i, r) in grid.iter().enumerate() {
        for j in 0..r.len() {
            dist.insert(UCoord::new(i, j), i64::MAX);
            prev.insert(UCoord::new(i, j), UCoord::new(usize::MAX, usize::MAX));
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
        if start == end || *u == UCoord::new(usize::MAX, usize::MAX) {
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
