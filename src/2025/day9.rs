use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::timeln;
use iter_tools::Itertools;

fn area(p1: &Coord, p2: &Coord) -> u64 {
    (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1)
}

struct Polygon {
    points: Vec<(Coord, Coord)>,
    lines: Vec<(Coord, Coord)>,
}

impl Polygon {
    fn new(points: &[Coord]) -> Self {
        let mut lines: Vec<(Coord, Coord)> = points.iter().copied().tuple_windows().collect();
        lines.push((*points.last().unwrap(), points[0]));

        Self {
            points: points
                .iter()
                .permutations(2)
                .map(|v| (*v[0], *v[1]))
                .collect(),
            lines,
        }
    }

    fn get_area(&self, (r1, r2): (Coord, Coord)) -> Option<u64> {
        let (rxmin, rxmax) = (r1.x.min(r2.x), r1.x.max(r2.x));
        let (rymin, rymax) = (r1.y.min(r2.y), r1.y.max(r2.y));

        for (l1, l2) in &self.lines {
            if !(l1.x.max(l2.x) <= rxmin
                || rxmax <= l1.x.min(l2.x)
                || l1.y.max(l2.y) <= rymin
                || rymax <= l1.y.min(l2.y))
            {
                return None;
            }
        }

        Some(area(&r1, &r2))
    }

    fn max_rectangle(&self) -> u64 {
        self.points
            .iter()
            .filter_map(|rect| self.get_area(*rect))
            .max()
            .unwrap_or(0)
    }
}

fn part1(input: &str) -> u64 {
    let tiles: Vec<Coord> = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|v| Coord::new(v[0], v[1]))
        .collect();

    tiles
        .into_iter()
        .permutations(2)
        .map(|v| area(&v[0], &v[1]))
        .max()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    let tiles: Vec<Coord> = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|v| Coord::new(v[0], v[1]))
        .collect();

    Polygon::new(&tiles).max_rectangle()
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2025/day9.txt");

    if let Err(e) = input {
        if e.kind() == io::ErrorKind::NotFound {
            println!("Input file not found!");
        }

        Err(e)
    } else {
        if benchmark {
            timeln!("part1: {}", part1(input.as_ref().unwrap()));
            timeln!("part2: {}", part2(input.as_ref().unwrap()));
        } else {
            println!("part1: {}", part1(input.as_ref().unwrap()));
            println!("part2: {}", part2(input.as_ref().unwrap()));
        }

        Ok(())
    }
}
