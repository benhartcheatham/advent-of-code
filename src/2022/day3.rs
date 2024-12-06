use std::collections::HashSet;
use std::fs;
use std::io;

use aocutils::timing;

fn part1(input: &str) {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let (pack1, pack2) = line.as_bytes().split_at(line.len() / 2);
        let set1: HashSet<u8> = HashSet::from_iter(pack1.iter().cloned());
        let set2: HashSet<u8> = HashSet::from_iter(pack2.iter().cloned());

        sum += set1
            .intersection(&set2)
            .map(|c| match *c {
                b'a'..=b'z' => *c - b'a' + 1,
                b'A'..=b'Z' => *c - b'A' + 27,
                _ => 0,
            } as u64)
            .sum::<u64>();
    }

    print!("part1: {}", sum);
}

#[allow(unused)]
fn part2(input: &str) {
    let mut sum: u64 = 0;
    let mut group: Vec<HashSet<u8>> = Vec::new();

    for line in input.lines() {
        group.push(HashSet::from_iter(line.as_bytes().iter().cloned()));

        if group.len() == 3 {
            sum += HashSet::from_iter(group[0].intersection(&group[1]).cloned())
                .intersection(&group[2])
                .cloned()
                .map(|c| match c {
                    b'a'..=b'z' => c - b'a' + 1,
                    b'A'..=b'Z' => c - b'A' + 27,
                    _ => 0,
                } as u64)
                .sum::<u64>();

            group = Vec::new();
        }
    }

    print!("part2: {}", sum);
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day3.txt")?;
    let mut timer = timing::start_benchmark(benchmark);

    part1(&input);
    timing::print_time(&mut timer);
    part2(&input);
    timing::print_time(&mut timer);

    Ok(())
}
