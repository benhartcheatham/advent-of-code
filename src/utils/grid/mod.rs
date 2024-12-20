pub mod algo;
pub mod direction;

use super::coord::Coord;

pub fn in_bounds<T>(grid: &[Vec<T>], coord: Coord) -> bool {
    let (x, y): (i64, i64) = coord.into();

    if x < 0 || y < 0 {
        return false;
    }

    let (x, y) = coord.as_unsigned().unwrap();

    if grid.is_empty() || x >= grid.len() {
        return false;
    }

    !grid[x].is_empty() && y < grid[x].len()
}
