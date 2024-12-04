use std::fs;
use std::io;
use std::collections::VecDeque;

use crate::utils::coord::*;
use crate::utils::direction::*;

#[derive(Debug)]
struct Space {
    spaces: VecDeque<u8>,
    highest: usize,
}

// TODO: Figure out how to make this space & time effecient for part two.
// Can't use a byte per layer, there are waaaaay to many layers for that to work
impl Space {
    fn new(size: usize) -> Self {
        let mut v = VecDeque::new();
        v.resize(size, 0);
        v[0] = u8::MAX;

        Space { spaces: v, highest: 0 }
    }

    fn conflicts(&self, coord: Coord) -> bool {
        let (x, y) = (coord.get_x() as usize, coord.get_y() as usize);
        assert!(x < 7);

        if y >= self.spaces.len() {
            return false;
        }

        let x = 1 << x;
        (self.spaces[y] & x) > 0
    }

    fn update(&mut self, coord: Coord) {
        let (x, y) = (coord.get_x() as usize, coord.get_y() as usize);
        assert!(x < 7);

        while self.spaces.len() < (y + 1) {
            self.spaces.push_back(0);
        }

        self.highest = self.highest.max(y);
        self.spaces[y] |= 1 << x;
    }

    fn len(&self) -> usize {
        7
    }

    fn get_highest(&self) -> usize {
        self.highest
    }
}

#[derive(Clone, Debug)]
struct Rock {
    points: Vec<Coord>,
    bottom: Vec<usize>,
}

impl Rock {
    /// @init is the bottom left corner of rock
    fn new(rtype: RockTypes) -> Self {
        let points = match rtype {
            RockTypes::Minus => vec![
                Coord::new(0, 0),
                Coord::new(1, 0),
                Coord::new(2, 0),
                Coord::new(3, 0),
            ],
            RockTypes::Plus => vec![
                Coord::new(1, 2),
                Coord::new(0, 1),
                Coord::new(1, 1),
                Coord::new(2, 1),
                Coord::new(1, 0),
            ],
            RockTypes::Hook => vec![
                Coord::new(2, 2),
                Coord::new(2, 1),
                Coord::new(0, 0),
                Coord::new(1, 0),
                Coord::new(2, 0),
            ],
            RockTypes::Pipe => vec![
                Coord::new(0, 3),
                Coord::new(0, 2),
                Coord::new(0, 1),
                Coord::new(0, 0),
            ],
            RockTypes::Box => vec![
                Coord::new(0, 1),
                Coord::new(1, 1),
                Coord::new(0, 0),
                Coord::new(1, 0),
            ],
        };

        let bottom: Vec<usize> = match rtype {
            RockTypes::Minus => [0, 1, 2, 3].into(),
            RockTypes::Plus => [1, 4, 3].into(),
            RockTypes::Hook => [2, 3, 4].into(),
            RockTypes::Pipe => [3].into(),
            RockTypes::Box => [2, 3].into(),
        };

        Rock { points, bottom }
    }

    /// @return: whether the rock is finished falling
    fn update(&mut self, dir: Direction, space: &mut Space) -> bool {
        let mut landed = false;

        self.translate(dir.into());

        for p in &self.points {
            let x = p.get_x();

            if x < 0 || x as usize >= space.len() || space.conflicts(*p) {
                self.translate(dir.invert().into());
                break;
            }
        }

        for p in &self.bottom {
            let c = self.points[*p];

            if space.conflicts(Coord::new(c.get_x(), c.get_y() - 1)) {
                landed = true;
                break;
            }
        }

        if landed {
            for p in &self.points {
                space.update(*p);
            }
        } else {
            self.translate(Direction::S.into());
            if self.points.iter().map(|p| p.get_y()).any(|n| n < 0) {
                panic!("Invalid coordinates: {:?}", self);
            }
        }

        landed
    }

    fn translate(&mut self, coord: Coord) {
        for p in self.points.iter_mut() {
            *p = *p + coord;
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum RockTypes {
    Minus,
    Plus,
    Hook,
    Pipe,
    Box,
}

fn char_to_dir(ch: char) -> Direction {
    match ch {
        '>' => Direction::E,
        '<' => Direction::W,
        _ => panic!("Invalid character {}!", ch),
    }
}

fn part1(input: &str) {
    use RockTypes::*;

    let rocks = [Minus, Plus, Hook, Pipe, Box];
    let mut rocks_iter = rocks.iter().cycle();
    let mut space: Space = Space::new(4);
    let mut hp: usize = 0;
    let mut n_rocks = 0;
    let mut direction_iter = input.trim().chars().cycle().map(char_to_dir);

    while n_rocks < 2022 {
        let rt = rocks_iter.next().unwrap();
        let mut r = Rock::new(*rt);
        r.translate(Coord::new(2, (hp + 4) as i64));

        loop {
            let d = direction_iter.next().unwrap();
            if r.update(d, &mut space) {
                hp = space.get_highest();
                break;
            }
        }

        n_rocks += 1;
    }

    println!("part1: {}", space.get_highest());
}

#[allow(unused)]
fn part2(input: &str) {
    use RockTypes::*;

    let rocks = [Minus, Plus, Hook, Pipe, Box];
    let mut rocks_iter = rocks.iter().cycle();
    let mut space: Space = Space::new(100_000);
    let mut hp: usize = 0;
    let mut n_rocks: usize = 0;
    let mut direction_iter = input.trim().chars().cycle().map(char_to_dir);

    while n_rocks < 1_000_000_000_000 {
        if n_rocks % 1_000_000 == 0 {
            println!("{}", n_rocks);
        }
        let rt = rocks_iter.next().unwrap();
        let mut r = Rock::new(*rt);
        r.translate(Coord::new(2, (hp + 4) as i64));

        loop {
            let d = direction_iter.next().unwrap();
            if r.update(d, &mut space) {
                hp = space.get_highest();
                break;
            }
        }

        n_rocks += 1;
    }

    println!("part2: {}", space.get_highest());
}

pub fn run() -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day17.txt")?;

    part1(&input);
    // part2(&input);
    Ok(())
}
