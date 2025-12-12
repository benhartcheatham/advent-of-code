use std::fs;
use std::io;

use aocutils::timeln;

fn part1(input: &str) -> usize {
    let mut grids = Vec::new();

    // Apparently just checking if the areas of the shapes can
    // fit in the grid is enough. All of the shapes are 3x3,
    // so we don't even need to parse the shapes
    for line in input.lines().skip(30) {
        let size = line
            .split(&['x', ':'])
            .take(2)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let counts = line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        grids.push(((size[0], size[1]), counts));
    }

    grids
        .into_iter()
        .filter(|((x, y), counts)| {
            x * y
                >= counts
                    .iter()
                    .copied()
                    .reduce(|acc, e| acc + (e * 9))
                    .unwrap()
        })
        .count()
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2025/day12.txt");

    if let Err(e) = input {
        if e.kind() == io::ErrorKind::NotFound {
            println!("Input file not found!");
        }

        Err(e)
    } else {
        if benchmark {
            timeln!("part1: {}", part1(input.as_ref().unwrap()));
        } else {
            println!("part1: {}", part1(input.as_ref().unwrap()));
        }

        Ok(())
    }
}
