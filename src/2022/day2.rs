use std::fs;
use std::io;

use aocutils::timing;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shapes {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Shapes {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Shapes::Rock,
            "B" | "Y" => Shapes::Paper,
            "C" | "Z" => Shapes::Scissors,
            _ => panic!("Invalid input: {}", value),
        }
    }
}

impl From<Shapes> for u64 {
    fn from(value: Shapes) -> Self {
        match value {
            Shapes::Rock => 1,
            Shapes::Paper => 2,
            Shapes::Scissors => 3,
        }
    }
}

impl From<u64> for Shapes {
    fn from(value: u64) -> Self {
        match value {
            1 => Shapes::Rock,
            2 => Shapes::Paper,
            3 => Shapes::Scissors,
            _ => panic!("Invalid input: {}", value),
        }
    }
}

struct Round {
    theirs: Shapes,
    ours: Shapes,
    score: u64,
}

impl Round {
    fn new(ours: &str, theirs: &str) -> Self {
        let ours: Shapes = ours.into();
        Round {
            ours,
            theirs: Into::<Shapes>::into(theirs),
            score: ours.into(),
        }
    }

    fn calc_score(&mut self) {
        use Shapes::*;

        match (self.ours, self.theirs) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => self.score += 6,
            (Scissors, Rock) | (Rock, Paper) | (Paper, Scissors) => (),
            _ => self.score += 3,
        }
    }

    fn calc_shape(theirs: &str, result: &str) -> Self {
        let theirs: Shapes = theirs.into();
        let n: u64 = theirs.into();

        let ours: Shapes = match result {
            "Z" => ((n % 3) + 1).into(),
            "Y" => theirs,
            "X" => {
                if n == 1 {
                    Into::<Shapes>::into(3u64)
                } else {
                    (n - 1).into()
                }
            }
            _ => panic!("Invalid input: {}", result),
        };

        Round {
            ours,
            theirs,
            score: ours.into(),
        }
    }
}

#[allow(unused)]
fn part1(input: &str) {
    let mut rounds: Vec<Round> = Vec::new();

    for shapes in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        let mut round = Round::new(shapes[1], shapes[0]);
        round.calc_score();
        rounds.push(round);
    }

    print!("part1: {}", rounds.iter().map(|r| r.score).sum::<u64>());
}

fn part2(input: &str) {
    let mut rounds: Vec<Round> = Vec::new();

    for shapes in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        let mut round = Round::calc_shape(shapes[0], shapes[1]);
        round.calc_score();
        rounds.push(round);
    }

    print!("part2: {}", rounds.iter().map(|r| r.score).sum::<u64>());
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day2.txt")?;
    let mut timer = timing::start_benchmark(benchmark);

    part1(&input);
    timing::print_time(&mut timer);
    part2(&input);
    timing::print_time(&mut timer);

    Ok(())
}
