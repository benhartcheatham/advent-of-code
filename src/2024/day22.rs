use std::collections::HashMap;
use std::fs;
use std::io;

use aocutils::timing::Timer;

#[derive(Debug)]
struct SecretNumber {
    val: i64,
    last: i64,
}

impl SecretNumber {
    fn new(n: i64) -> Self {
        SecretNumber {
            val: n,
            last: n % 10,
        }
    }

    fn mix(&mut self, other: i64) {
        self.val ^= other;
    }

    fn prune(&mut self) {
        self.val %= 16777216;
    }

    fn next_number(&mut self) {
        self.mix(self.val * 64);
        self.prune();
        self.mix((self.val as f64 / 32.0).floor() as i64);
        self.prune();
        self.mix(self.val * 2048);
        self.prune();
        self.last = self.val % 10;
    }
}

fn part1(input: &str) {
    let mut nums = Vec::new();

    for line in input.lines() {
        nums.push(SecretNumber::new(line.trim().parse::<i64>().unwrap()));
    }

    print!(
        "part1: {}",
        nums.iter_mut()
            .map(|n| {
                for _ in 0..2000 {
                    n.next_number();
                }

                n.val
            })
            .sum::<i64>()
    );
}

fn part2(input: &str) {
    let mut nums = Vec::new();

    for line in input.lines() {
        nums.push(SecretNumber::new(line.trim().parse::<i64>().unwrap()));
    }

    let mut buyer_changes = Vec::new();
    for (i, sn) in nums.iter_mut().enumerate() {
        buyer_changes.push(Vec::new());

        let mut last = sn.last;
        buyer_changes[i].push((last, 0));

        for _ in 0..2000 {
            sn.next_number();
            buyer_changes[i].push((sn.last, sn.last - last));
            last = sn.last;
        }
    }

    let mut buyers_map = HashMap::new();
    for changes in buyer_changes {
        let mut map = HashMap::new();

        for i in 0..(changes.len() - 3) {
            let change_slice = changes[i..=i + 3].iter().map(|c| c.1).collect::<Vec<i64>>();
            map.entry(change_slice).or_insert(changes[i + 3].0);
        }

        for (k, v) in map {
            buyers_map.entry(k).and_modify(|bv| *bv += v).or_insert(v);
        }
    }

    print!("part2: {}", buyers_map.into_values().max().unwrap());
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day22.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changes() {
        let mut buyer_changes = Vec::new();
        let mut sn = SecretNumber::new(123);
        let expected = vec![
            (3, 0),
            (0, -3),
            (6, 6),
            (5, -1),
            (4, -1),
            (4, 0),
            (6, 2),
            (4, -2),
            (4, 0),
            (2, -2),
        ];

        buyer_changes.push((sn.last, 0));
        let mut last = sn.last;
        for _ in 0..9 {
            sn.next_number();
            buyer_changes.push((sn.last, sn.last - last));
            last = sn.last;
        }

        assert_eq!(expected, buyer_changes);
    }

    #[test]
    fn day22_example2() {
        let mut nums = vec![
            SecretNumber::new(1),
            SecretNumber::new(2),
            SecretNumber::new(3),
            SecretNumber::new(2024),
        ];

        let mut buyer_changes = Vec::new();
        for (i, sn) in nums.iter_mut().enumerate() {
            buyer_changes.push(Vec::new());

            let mut last = sn.last;
            buyer_changes[i].push((last, 0));

            for _ in 0..2000 {
                sn.next_number();
                buyer_changes[i].push((sn.last, sn.last - last));
                last = sn.last;
            }
        }

        let mut buyers_map = HashMap::new();
        for changes in buyer_changes {
            let mut map = HashMap::new();

            for i in 0..(changes.len() - 3) {
                let change_slice = changes[i..=i + 3].iter().map(|c| c.1).collect::<Vec<i64>>();
                map.entry(change_slice).or_insert(changes[i + 3].0);
            }

            for (k, v) in map {
                buyers_map.entry(k).and_modify(|bv| *bv += v).or_insert(v);
            }
        }

        assert_eq!(23, buyers_map.into_values().max().unwrap());
    }
}
