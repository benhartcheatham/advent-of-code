pub mod coord;
pub mod algo;
pub mod direction;

use coord::GridCoord;
use super::coord::Coord;

pub fn in_bounds<T>(grid: &[Vec<T>], coord: GridCoord) -> bool {
    let (x, y): (usize, usize) = coord.into();

    if grid.is_empty() || x >= grid.len() {
        return false;
    }

    !grid[x].is_empty() && y < grid[0].len()
}

pub fn in_ibounds<T>(grid: &[Vec<T>], coord: Coord) -> bool {
    let (x, y): (i64, i64) = coord.into();

    if grid.is_empty() || x < 0 || grid[x as usize].is_empty() || y < 0 {
        return false;
    }

    (x as usize) < grid.len() && (y as usize) < grid[0].len()
}
