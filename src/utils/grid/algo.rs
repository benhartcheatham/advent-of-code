use super::direction::GridDirection;
use crate::{coord::Coord, grid::in_ibounds};
use std::{collections::{BinaryHeap, HashSet}, fmt::Debug};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeapElem {
    coord: Coord,
    data: usize,
}

impl HeapElem {
    fn new(data: usize, coord: Coord) -> Self {
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

pub type DjikstraCostFn<T> = fn(&[Vec<T>], Coord, Coord) -> usize;

/// Takes in a starting and ending point and returns the shortest path between them
#[allow(unused)]
pub fn djikstra<T: Copy + Clone + Ord + Debug>(
    grid: &[Vec<T>],
    start: Coord,
    end: Coord,
    func: DjikstraCostFn<T>,
) -> Vec<Coord> {
    use GridDirection::*;

    if grid.is_empty() || !in_ibounds(grid, start) || !in_ibounds(grid, end) {
        return Vec::new();
    }

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut dist =
        vec![vec![(usize::MAX, Coord::new(i64::MAX, i64::MAX)); grid[0].len()]; grid.len()];

    let (sx, sy) = start.as_unsigned().unwrap();
    dist[sx][sy] = (0, Coord::new(0, 0));

    queue.push(HeapElem::new(0, start));
    while let Some(he) = queue.pop() {
        let u = he.coord;
        visited.insert(u);

        if u == end {
            break;
        }

        for d in super::direction::DIRECTIONS {
            let next = u + d.into();

            if !in_ibounds(grid, next) || visited.contains(&next) {
                continue;
            }

            let (x, y) = u.as_unsigned().unwrap();
            let (nx, ny) = next.as_unsigned().unwrap();

            let alt = func(grid, u, next);
            if alt < dist[nx][ny].0 {
                dist[nx][ny] = (alt, u);
                queue.push(HeapElem::new(alt, next));
            }
        }
    }

    let mut path = Vec::new();
    path.push(end);

    let (ex, ey) = end.as_unsigned().unwrap();
    let mut prev = dist[ex][ey].1;

    while prev != start {
        path.push(prev);
        let (px, py) = prev.as_unsigned().unwrap();
        prev = dist[px][py].1;
    }

    path.push(start);
    path.reverse();

    path
}
