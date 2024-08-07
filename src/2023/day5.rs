use std::thread;
use std::{fs, io};

#[derive(Debug, Clone, Copy)]
struct MapEntry {
    source: u64,
    dest: u64,
    range: u64,
}

#[derive(Debug, Clone)]
struct Map {
    entries: Vec<MapEntry>,
}

impl MapEntry {
    fn new(source: u64, dest: u64, range: u64) -> Self {
        MapEntry {
            source,
            dest,
            range,
        }
    }

    fn get_seed_mapping(&self, seed: u64) -> u64 {
        if self.source <= seed && seed < (self.source + self.range) {
            seed - self.source + self.dest
        } else {
            seed
        }
    }
}

impl Map {
    fn new() -> Self {
        Map {
            entries: Vec::new(),
        }
    }

    fn insert(&mut self, e: MapEntry) {
        self.entries.push(e);
    }

    fn get_mapping(&self, seed: u64) -> u64 {
        for e in &self.entries {
            let m = e.get_seed_mapping(seed);
            if m != seed {
                return m;
            }
        }

        seed
    }
}

pub fn run() -> Result<(), io::Error> {
    let input = fs::read_to_string("inputs/2023/day5.txt")?;

    println!("solution: {}", solution(&input));
    Ok(())
}

fn parse_line(line: &str) -> MapEntry {
    let nums: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    if nums.len() != 3 {
        println!("line: {}", line);
        println!("nums: {:?}", nums);
        panic!("Bad input!");
    }

    MapEntry::new(nums[1], nums[0], nums[2])
}

fn find_location(seed: u64, maps: &[Map]) -> u64 {
    maps.iter().fold(seed, |id, e| e.get_mapping(id))
}

fn solution(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().filter(|s| !s.trim().is_empty()).collect();
    let seeds: Vec<u64> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut maps: Vec<Map> = Vec::new();

    let mut map = Map::new();
    let mut iter = lines.iter().skip(1).peekable();
    while iter.peek().is_some() {
        let line = iter.next().unwrap();

        if line.ends_with("map:") {
            if !map.entries.is_empty() {
                maps.push(map);
                map = Map::new();
            }

            continue;
        }

        map.insert(parse_line(line));
    }

    if !map.entries.is_empty() {
        maps.push(map);
    }

    let mut handles = Vec::new();
    let ranges = seeds.chunks(2);
    for r in ranges {
        let (start, range) = (r[0], r[1]);
        let m = maps.clone();
        let h = thread::spawn(move || {
            let mut min = u64::MAX;

            for i in 0..range {
                min = min.min(find_location(start + i, &m));
            }

            min
        });

        handles.push(h);
    }

    let mut locations = Vec::new();
    for h in handles {
        locations.push(h.join().unwrap());
    }

    *locations.iter().min().unwrap()
}
