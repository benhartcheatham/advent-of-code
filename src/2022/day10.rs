use std::fs;
use std::io;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Instructions {
    Addx,
    Noop,
}

#[derive(Debug, PartialOrd, PartialEq)]
struct InstructionParseErr;

impl std::fmt::Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Instructions::Addx => "addx",
                _ => "noop",
            }
        )
    }
}

impl FromStr for Instructions {
    type Err = InstructionParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addx" => Ok(Instructions::Addx),
            "noop" => Ok(Instructions::Noop),
            _ => Err(InstructionParseErr),
        }
    }
}

struct Cpu {
    cycle: usize,
    x: i64,
    interval: usize,
    signals: Vec<i64>,
}

impl Cpu {
    fn new(interval: usize) -> Self {
        Cpu {
            cycle: 0,
            x: 1,
            interval,
            signals: Vec::new(),
        }
    }

    fn run_instruction(&mut self, inst: Instructions, val: Option<i64>) {
        self.cycle += 1;

        if self.cycle % self.interval == 0 {
            self.signals.push(self.x * self.cycle as i64);
        }

        if inst == Instructions::Addx {
            self.x += val.unwrap();
        }
    }
}

struct Crt {
    pos: usize,
    screen: Vec<Vec<char>>,
    width: usize,
    cpu: Cpu,
}

impl Crt {
    fn new(width: usize) -> Self {
        Crt {
            pos: 0,
            screen: Vec::new(),
            width,
            cpu: Cpu::new(1000),
        }
    }

    fn render_pixel(&mut self, inst: Instructions, val: Option<i64>) {
        if self.pos % self.width == 0 {
            self.screen.push(Vec::new());
            self.pos = 0;
        }

        let row = self.screen.len() - 1;

        if self.pos as i64 >= self.cpu.x - 1 && self.pos as i64 <= self.cpu.x + 1 {
            self.screen[row].push('#');
        } else {
            self.screen[row].push('.');
        }

        self.pos += 1;

        self.cpu.run_instruction(inst, val);
    }
}

fn part1(input: &str) {
    let mut cpu = Cpu::new(20);

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let inst = parts[0].parse::<Instructions>().unwrap();

        cpu.run_instruction(Instructions::Noop, None);

        if inst == Instructions::Addx {
            cpu.run_instruction(inst, Some(parts[1].parse::<i64>().unwrap()));
        }
    }

    println!(
        "part1: {}",
        cpu.signals.iter().step_by(2).take(6).sum::<i64>()
    );
}

fn part2(input: &str) {
    let mut crt = Crt::new(40);

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let inst = parts[0].parse::<Instructions>().unwrap();

        crt.render_pixel(Instructions::Noop, None);

        if inst == Instructions::Addx {
            crt.render_pixel(inst, Some(parts[1].parse::<i64>().unwrap()));
        }
    }

    println!("part2:");
    for r in crt.screen {
        for c in r {
            print!("{}", c);
        }

        println!();
    }
}

pub fn run() -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day10.txt")?;

    part1(&input);
    part2(&input);

    Ok(())
}
