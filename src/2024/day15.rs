use std::fmt::Display;
use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::grid::direction::GridDirection;
use aocutils::grid::in_ibounds;
use aocutils::timing::Timer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Robot,
    Box,
    Wall,
    WideBoxLeft,
    WideBoxRight,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Robot => '@',
                Cell::Box => 'O',
                Cell::Wall => '#',
                Cell::WideBoxLeft => '[',
                Cell::WideBoxRight => ']',
            }
        )
    }
}

struct Robot {
    pos: Coord,
    use_wide: bool,
}

impl Robot {
    fn new(pos: Coord) -> Self {
        Robot {
            pos,
            use_wide: false,
        }
    }

    fn find_empty(&self, grid: &[Vec<Option<Cell>>], dir: Coord) -> Option<Coord> {
        let mut next = self.pos + dir;

        if !in_ibounds(grid, next) {
            return None;
        }

        let (mut x, mut y) = next.as_unsigned().unwrap();
        while grid[x][y].is_some() {
            next += dir;

            if !in_ibounds(grid, next) || grid[x][y].unwrap() == Cell::Wall {
                return None;
            }

            (x, y) = next.as_unsigned().unwrap();
        }

        Some(next)
    }

    fn can_push_wide_box(leftpos: Coord, grid: &mut [Vec<Option<Cell>>], dir: Coord) -> bool {
        use Cell::*;

        let nextleft = leftpos + dir;
        let nextright = leftpos + dir + Coord::new(0, 1);

        if !in_ibounds(grid, nextleft) || !in_ibounds(grid, nextright) {
            return false;
        }

        let (x1, y1) = nextleft.as_unsigned().unwrap();
        let (x2, y2) = nextright.as_unsigned().unwrap();

        match (grid[x1][y1], grid[x2][y2]) {
            (Some(Wall), _) | (_, Some(Wall)) => false,
            (Some(WideBoxLeft), Some(WideBoxRight)) => Self::can_push_wide_box(nextleft, grid, dir),
            (Some(WideBoxRight), None) => {
                Self::can_push_wide_box(nextleft - Coord::new(0, 1), grid, dir)
            }
            (None, Some(WideBoxLeft)) => Self::can_push_wide_box(nextright, grid, dir),
            (Some(WideBoxRight), Some(WideBoxLeft)) => {
                Self::can_push_wide_box(nextleft - Coord::new(0, 1), grid, dir)
                    && Self::can_push_wide_box(nextright, grid, dir)
            }
            _ => true,
        }
    }

    fn push_wide_box(leftpos: Coord, grid: &mut [Vec<Option<Cell>>], dir: Coord) {
        use Cell::*;

        let nextleft = leftpos + dir;
        let nextright = leftpos + dir + Coord::new(0, 1);

        let (x1, y1) = nextleft.as_unsigned().unwrap();
        let (x2, y2) = nextright.as_unsigned().unwrap();
        let (px, py) = leftpos.as_unsigned().unwrap();

        match (grid[x1][y1], grid[x2][y2]) {
            (Some(WideBoxLeft), Some(WideBoxRight)) => Self::push_wide_box(nextleft, grid, dir),
            (Some(WideBoxRight), None) => {
                Self::push_wide_box(nextleft - Coord::new(0, 1), grid, dir)
            }
            (None, Some(WideBoxLeft)) => Self::push_wide_box(nextright, grid, dir),
            (Some(WideBoxRight), Some(WideBoxLeft)) => {
                Self::push_wide_box(nextleft - Coord::new(0, 1), grid, dir);
                Self::push_wide_box(nextright, grid, dir);
            }
            _ => (),
        }

        grid[x1][y1] = grid[px][py].take();
        grid[x2][y2] = grid[px][py + 1].take();
    }

    fn push_helper(&mut self, grid: &mut [Vec<Option<Cell>>], dir: Coord) {
        let next = self.find_empty(grid, dir);
        let next = if let Some(coord) = next {
            coord
        } else {
            return;
        };

        let mut prev = next;
        let mut next = next + dir.mult_scalar(-1);

        while prev != self.pos {
            let (px, py) = prev.as_unsigned().unwrap();
            let (nx, ny) = next.as_unsigned().unwrap();

            grid[px][py] = grid[nx][ny].take();

            prev = next;
            next += dir.mult_scalar(-1);
        }

        self.pos += dir;
    }

    fn push_wide_helper(&mut self, grid: &mut [Vec<Option<Cell>>], dir: Coord) {
        if self.find_empty(grid, dir).is_none() {
            return;
        }

        let (px, py) = self.pos.as_unsigned().unwrap();
        let next = self.pos + dir;
        let (x, y) = next.as_unsigned().unwrap();

        match grid[x][y] {
            Some(Cell::WideBoxLeft) => {
                if Self::can_push_wide_box(next, grid, dir) {
                    Self::push_wide_box(next, grid, dir);
                    grid[x][y] = grid[px][py].take();
                    self.pos += dir;
                }
            }
            Some(Cell::WideBoxRight) => {
                let leftpos = Coord::new(x as i64, y as i64 - 1);
                if Self::can_push_wide_box(leftpos, grid, dir) {
                    Self::push_wide_box(leftpos, grid, dir);
                    grid[x][y] = grid[px][py].take();
                    self.pos += dir;
                }
            }
            _ => {
                grid[x][y] = grid[px][py].take();
                self.pos += dir;
            }
        }
    }

    fn push(&mut self, grid: &mut [Vec<Option<Cell>>], dir: GridDirection) {
        use GridDirection::*;

        match dir {
            Up | Down => {
                if self.use_wide {
                    self.push_wide_helper(grid, dir.into())
                } else {
                    self.push_helper(grid, dir.into())
                }
            }
            Left | Right => self.push_helper(grid, dir.into()),
        }
    }
}

fn part1(input: &str) {
    let mut lines = input.lines();
    let mut grid = Vec::new();
    let mut robot = Robot::new(Coord::new(0, 0));

    for (i, line) in lines.by_ref().enumerate() {
        if line.trim().is_empty() {
            break;
        }

        grid.push(Vec::new());
        for (j, ch) in line.char_indices() {
            grid[i].push(match ch {
                '#' => Some(Cell::Wall),
                'O' => Some(Cell::Box),
                '@' => {
                    robot.pos = Coord::new(i as i64, j as i64);
                    Some(Cell::Robot)
                }
                _ => None,
            });
        }
    }

    let mut movements = Vec::new();
    for line in lines {
        for ch in line.trim().chars().map(|ch| ch.to_string()) {
            movements.push(ch.parse::<GridDirection>().unwrap());
        }
    }

    for m in movements {
        robot.push(&mut grid, m);
    }

    print!(
        "part1: {}",
        grid.iter()
            .enumerate()
            .map(|(i, row)| row
                .iter()
                .enumerate()
                .map(|(j, cell)| match cell {
                    Some(Cell::Box) => i * 100 + j,
                    _ => 0,
                })
                .sum::<usize>())
            .sum::<usize>()
    );
}

fn part2(input: &str) {
    let mut lines = input.lines();
    let mut grid = Vec::new();
    let mut robot = Robot::new(Coord::new(0, 0));

    robot.use_wide = true;

    for (i, line) in lines.by_ref().enumerate() {
        if line.trim().is_empty() {
            break;
        }

        grid.push(Vec::new());
        for (j, ch) in line.char_indices() {
            match ch {
                '#' => {
                    grid[i].push(Some(Cell::Wall));
                    grid[i].push(Some(Cell::Wall))
                }
                'O' => {
                    grid[i].push(Some(Cell::WideBoxLeft));
                    grid[i].push(Some(Cell::WideBoxRight))
                }
                '@' => {
                    robot.pos = Coord::new(i as i64, (j * 2) as i64);
                    grid[i].push(Some(Cell::Robot));
                    grid[i].push(None);
                }
                '.' => {
                    grid[i].push(None);
                    grid[i].push(None);
                }
                _ => panic!("Unexpected character: {}!", ch),
            }
        }
    }

    let mut movements = Vec::new();
    for line in lines {
        for ch in line.trim().chars().map(|ch| ch.to_string()) {
            movements.push(ch.parse::<GridDirection>().unwrap());
        }
    }

    for m in movements.iter() {
        robot.push(&mut grid, *m);
    }

    print!(
        "part2: {}",
        grid.iter()
            .enumerate()
            .map(|(i, row)| row
                .iter()
                .enumerate()
                .map(|(j, cell)| match cell {
                    Some(Cell::WideBoxLeft) => i * 100 + j,
                    _ => 0,
                })
                .sum::<usize>())
            .sum::<usize>()
    );
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day15.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
