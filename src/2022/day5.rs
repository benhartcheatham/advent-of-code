use std::fs;
use std::io;

use aocutils::timeln;

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(levels: Vec<String>) -> Self {
        let mut grid = Vec::new();

        let grid_len = levels.iter().map(|s| s.len()).max().unwrap();
        for _ in 0..grid_len {
            grid.push(Vec::new());
        }

        for level in levels.iter().rev() {
            for (i, c) in level.char_indices().filter(|(_, c)| *c != ' ') {
                grid[i].push(c);
            }
        }

        Grid { grid }
    }

    fn do_move(&mut self, num: usize, from: usize, to: usize) {
        let from = from - 1;
        let to = to - 1;

        for _ in 0..num {
            if self.grid[from].is_empty() {
                break;
            }

            let c = self.grid[from].pop().unwrap();
            self.grid[to].push(c);
        }
    }

    fn do_multi_move(&mut self, num: usize, from: usize, to: usize) {
        let from = from - 1;
        let to = to - 1;

        let mut queue = Vec::new();
        for _ in 0..num {
            if self.grid[from].is_empty() {
                break;
            }

            queue.push(self.grid[from].pop().unwrap());
        }

        for _ in 0..queue.len() {
            self.grid[to].push(queue.pop().unwrap());
        }
    }

    fn skim_top(&self) -> String {
        self.grid.iter().filter_map(|v| v.last()).collect()
    }
}

fn parse_input(input: &str) -> (Grid, Vec<(usize, usize, usize)>) {
    let mut crates: Vec<&str> = Vec::new();

    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        crates.push(line);
    }

    crates.pop();

    let mut levels: Vec<String> = Vec::new();
    for level in crates
        .iter()
        .map(|s| s.as_bytes().iter().skip(1).step_by(4))
    {
        levels.push(String::new());

        let idx = levels.len() - 1;
        for c in level {
            levels[idx].push(*c as char);
        }
    }

    let grid = Grid::new(levels);

    let mut instructions: Vec<(usize, usize, usize)> = Vec::new();
    for line in lines {
        let v = line
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<u32>>();
        instructions.push((v[0] as usize, v[1] as usize, v[2] as usize));
    }

    (grid, instructions)
}

fn part1(input: &str) -> String {
    let (mut grid, instructions) = parse_input(input);

    for (num, from, to) in instructions {
        grid.do_move(num, from, to);
    }

    grid.skim_top()
}

fn part2(input: &str) -> String {
    let (mut grid, instructions) = parse_input(input);

    for (num, from, to) in instructions {
        grid.do_multi_move(num, from, to);
    }

    grid.skim_top()
}

pub fn run(_benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day5.txt")?;
    timeln!("part1: {}", part1(&input));
    timeln!("part2: {}", part2(&input));

    Ok(())
}
