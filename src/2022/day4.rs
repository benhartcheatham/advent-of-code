use std::fs;
use std::{io, ops::RangeInclusive};

trait Within<T: PartialOrd> {
    fn within(&self, other: &RangeInclusive<T>) -> bool;
}

trait Overlap<T: PartialOrd> {
    fn overlaps(&self, other: &RangeInclusive<T>) -> bool;
}

impl<T: PartialOrd> Within<T> for RangeInclusive<T> {
    fn within(&self, other: &RangeInclusive<T>) -> bool {
        other.contains(self.start()) && other.contains(self.end())
    }
}

impl<T: PartialOrd> Overlap<T> for RangeInclusive<T> {
    fn overlaps(&self, other: &RangeInclusive<T>) -> bool {
        self.contains(other.start())
            || self.contains(other.end())
            || other.contains(self.start())
            || other.contains(self.end())
    }
}

fn part1(input: &str) {
    println!(
        "part1: {}",
        input
            .lines()
            .map(|s| {
                // Parse input into Vec<u32> of format [n1, n2, n3, n4]
                s.split([',', '-'])
                    .filter_map(|c| c.parse().ok())
                    .collect::<Vec<u32>>()
            })
            .map(|v| (
                // Create ranges based off of Vec (must have lower number first due to how Ranges work)
                v[0].min(v[1])..=v[0].max(v[1]),
                v[2].min(v[3])..=v[2].max(v[3])
            ))
            .filter(|(r1, r2)| r1.within(r2) || r2.within(r1)) // Filter to only ranges that are
            // fully contained in other
            .count()
    );
}

fn part2(input: &str) {
    // same as part1, but count overlapping instead of contained
    println!(
        "part2: {}",
        input
            .lines()
            .map(|s| {
                s.split([',', '-'])
                    .filter_map(|c| c.parse().ok())
                    .collect::<Vec<u32>>()
            })
            .map(|v| (
                v[0].min(v[1])..=v[0].max(v[1]),
                v[2].min(v[3])..=v[2].max(v[3])
            ))
            .filter(|(r1, r2)| r1.overlaps(r2))
            .count()
    );
}

pub fn run() -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day4.txt")?;

    part1(&input);
    part2(&input);

    Ok(())
}
