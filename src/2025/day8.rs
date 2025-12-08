use std::collections::HashSet;
use std::fs;
use std::io;

use aocutils::timeln;

#[derive(Clone, Copy)]
struct Coord3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord3 {
    fn new(xyz: &[i64]) -> Option<Self> {
        if xyz.len() < 3 {
            None
        } else {
            Some(Coord3 {
                x: xyz[0],
                y: xyz[1],
                z: xyz[2],
            })
        }
    }

    fn dist(&self, other: &Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

fn shortest_pairs(boxes: &[Coord3]) -> Vec<(usize, usize)> {
    let mut dists: Vec<(usize, usize, f64)> = Vec::new();

    for (i, c) in boxes.iter().enumerate() {
        for (j, c2) in boxes.iter().enumerate().skip(i + 1) {
            dists.push((i, j, c.dist(c2)));
        }
    }

    dists.sort_by(|e1, e2| e1.2.partial_cmp(&e2.2).unwrap());
    dists.into_iter().map(|(i, j, _)| (i, j)).collect()
}

fn find_circuit(circuits: &Vec<HashSet<usize>>, b: usize) -> usize {
    (0..circuits.len())
        .find(|i| circuits[*i].contains(&b))
        .unwrap()
}

fn connect(boxes: &[Coord3], npairs: Option<usize>) -> usize {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    for i in 0..boxes.len() {
        let mut c = HashSet::new();
        c.insert(i);
        circuits.push(c);
    }

    for (b1, b2) in shortest_pairs(boxes)
        .iter()
        .take(npairs.unwrap_or(usize::MAX))
    {
        let c1 = find_circuit(&circuits, *b1);
        let c2 = find_circuit(&circuits, *b2);

        if c1 != c2 {
            circuits[c1] = circuits[c1]
                .union(&circuits[c2])
                .copied()
                .collect::<HashSet<usize>>();
            circuits.remove(c2);
        }

        if circuits.len() == 1 {
            return (boxes[*b1].x * boxes[*b2].x) as usize;
        }
    }

    circuits.sort_by(|s1, s2| s1.len().cmp(&s2.len()).reverse());
    circuits
        .iter()
        .take(3)
        .map(|v| v.len())
        .reduce(|acc, e| acc * e)
        .unwrap()
}

fn part1(input: &str, npairs: usize) -> usize {
    let boxes: Vec<Coord3> = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|d| d.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter_map(|v| Coord3::new(&v))
        .collect();

    connect(&boxes, Some(npairs))
}

fn part2(input: &str) -> usize {
    let boxes: Vec<Coord3> = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|d| d.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter_map(|v| Coord3::new(&v))
        .collect();

    connect(&boxes, None)
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2025/day8.txt");
    let pairs = 1000;

    if let Err(e) = input {
        if e.kind() == io::ErrorKind::NotFound {
            println!("Input file not found!");
        }

        Err(e)
    } else {
        if benchmark {
            timeln!("part1: {}", part1(input.as_ref().unwrap(), pairs));
            timeln!("part2: {}", part2(input.as_ref().unwrap()));
        } else {
            println!("part1: {}", part1(input.as_ref().unwrap(), pairs));
            println!("part2: {}", part2(input.as_ref().unwrap()));
        }

        Ok(())
    }
}
