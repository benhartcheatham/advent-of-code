use super::direction::GridDirection;
use crate::{coord::Coord, grid::in_bounds};
use std::{collections::BinaryHeap, fmt::Debug};

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

/// Takes in a starting and ending point and returns the shortest path between them
/// if it exists
///
/// @func is a function that takes the grid, the current node, the cost of reaching the
/// current node, and the next node and retuns the cost of current -> next
#[allow(unused)]
pub fn djikstra<T, F>(grid: &[Vec<T>], start: Coord, end: Coord, func: F) -> Option<Vec<Coord>>
where
    T: Copy + Clone + Ord + Debug,
    F: Fn(&[Vec<T>], Coord, usize, Coord) -> usize,
{
    use GridDirection::*;

    if grid.is_empty() || !in_bounds(grid, start) || !in_bounds(grid, end) {
        return None;
    }

    let mut queue = BinaryHeap::new();
    let mut dist =
        vec![vec![(usize::MAX, Coord::new(i64::MAX, i64::MAX)); grid[0].len()]; grid.len()];

    let (sx, sy) = start.as_unsigned().unwrap();
    dist[sx][sy] = (0, Coord::new(0, 0));

    queue.push(HeapElem::new(0, start));
    while let Some(he) = queue.pop() {
        let curr = he.coord;

        if curr == end {
            break;
        }

        for d in super::direction::DIRECTIONS {
            let next = curr + d.into();

            if !in_bounds(grid, next) {
                continue;
            }

            let (x, y) = curr.as_unsigned().unwrap();
            let (nx, ny) = next.as_unsigned().unwrap();
            let alt = func(grid, curr, dist[x][y].0, next);

            if alt < dist[nx][ny].0 {
                dist[nx][ny] = (alt, curr);
                queue.push(HeapElem::new(alt, next));
            }
        }
    }

    let mut path = Vec::new();
    path.push(end);

    let (ex, ey) = end.as_unsigned().unwrap();
    let mut prev = dist[ex][ey].1;

    while prev != start {
        if in_bounds(grid, prev) {
            path.push(prev);
            let (px, py) = prev.as_unsigned().unwrap();
            prev = dist[px][py].1;
        } else {
            return None;
        }
    }

    path.push(start);
    path.reverse();

    Some(path)
}
