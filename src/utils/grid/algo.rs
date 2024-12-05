use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};
use super::{in_bounds, direction::GridDirection, coord::GridCoord};
use crate::coord::Coord;


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
            let v = if dc.x < 0 {
                u.coord.saturating_sub(GridCoord::new(1, 0))
            } else {
                u.coord + GridCoord::new(1, 0)
            };
            let v = if dc.y < 0 {
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
