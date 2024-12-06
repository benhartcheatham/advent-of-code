use std::io;
use std::fs;

use aocutils::timing;

fn part1(input: &str) {
    print!("part1: {}", 0);
}

fn part2(input: &str) {
    print!("part2: {}", 0);
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/20XX/dayYY_example.txt")?;
    let mut timer = timing::start_benchmark(benchmark);

    part1(&input);
    timing::print_time(&mut timer);
    part2(&input);
    timing::print_time(&mut timer);

    Ok(())
}
