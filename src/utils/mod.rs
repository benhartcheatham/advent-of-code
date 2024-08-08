mod graph;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for u64 {
    fn from(value: Direction) -> Self {
        use Direction::*;

        match value {
            Up => 1,
            Down => 2,
            Left => 3,
            Right => 4,
        }
    }
}

impl From<Direction> for i64 {
    fn from(value: Direction) -> Self {
        use Direction::*;

        match value {
            Up | Left => -1,
            Down | Right => 1,
        }
    }
}
