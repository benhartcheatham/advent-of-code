use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;
use std::io;

use aocutils::timing;

type MonkeyOp = fn(u64, Option<u64>) -> u64;

#[derive(Debug)]
struct Monkey {
    items: RefCell<VecDeque<u64>>,
    num_inspected: RefCell<usize>,
    operand: Option<u64>,
    operation: MonkeyOp,
    test: u64,
    divisor: u64,
    throw: (usize, usize),
}

impl Monkey {
    fn new(
        items: VecDeque<u64>,
        operand: Option<u64>,
        operation: fn(u64, Option<u64>) -> u64,
        test: u64,
        throw: (usize, usize),
    ) -> Self {
        Monkey {
            items: RefCell::new(items),
            num_inspected: RefCell::new(0),
            operand,
            operation,
            test,
            divisor: 1,
            throw,
        }
    }

    fn inspect(&self, relief: bool) {
        for item in self.items.borrow_mut().iter_mut() {
            *item = (self.operation)(*item, self.operand);

            if relief {
                *item /= 3;
            } else {
                *item %= self.divisor;
            }

            *self.num_inspected.borrow_mut() += 1;
        }
    }

    fn receive_item(&self, item: u64) {
        self.items.borrow_mut().push_back(item);
    }

    fn test_and_throw(&self, monkeys: &[Monkey]) {
        while let Some(item) = self.items.borrow_mut().pop_front() {
            let idx = if item % self.test == 0 {
                self.throw.0
            } else {
                self.throw.1
            };

            monkeys[idx].receive_item(item);
        }
    }

    fn get_num_inspected(&self) -> usize {
        *self.num_inspected.borrow()
    }
}

fn create_op(line: &str) -> (Option<u64>, MonkeyOp) {
    let parts: Vec<&str> = line.split_whitespace().skip(4).collect();

    let closure = match parts[0] {
        "*" => |n, n2| if let Some(n2) = n2 { n * n2 } else { n * n },
        "+" => |n, n2| if let Some(n2) = n2 { n + n2 } else { n + n },
        _ => |n, _| {
            println!("invalid operator!");
            n
        },
    };

    (parts[1].parse::<u64>().ok(), closure)
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut lines = input.lines().map(|l| l.trim());
    let mut monkeys = Vec::new();

    while let Some(_) = lines.next() {
        let items: VecDeque<u64> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|s| {
                s.chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u64>()
                    .ok()
            })
            .collect();

        let (operand, operation) = create_op(lines.next().unwrap());

        let mut nums: [usize; 3] = [0; 3];
        for n in &mut nums {
            *n = lines
                .next()
                .unwrap()
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<usize>>()[0];
        }

        monkeys.push(Monkey::new(
            items,
            operand,
            operation,
            nums[0] as u64,
            (nums[1], nums[2]),
        ));

        lines.next();
    }

    monkeys
}

fn part1(input: &str) {
    let mut monkeys = parse_monkeys(input);

    for _ in 0..20 {
        for m in &monkeys {
            m.inspect(true);
            m.test_and_throw(&monkeys);
        }
    }

    monkeys.sort_by_key(|m| m.get_num_inspected());
    let business = monkeys[monkeys.len() - 1].get_num_inspected()
        * monkeys[monkeys.len() - 2].get_num_inspected();
    print!("part1: {}", business);
}

fn part2(input: &str) {
    let mut monkeys = parse_monkeys(input);

    let divisor = monkeys.iter().map(|m| m.test).product();

    for m in monkeys.iter_mut() {
        m.divisor = divisor;
    }

    for _ in 0..10_000 {
        for m in &monkeys {
            m.inspect(false);
            m.test_and_throw(&monkeys);
        }
    }

    monkeys.sort_by_key(|m| m.get_num_inspected());
    let business = monkeys[monkeys.len() - 1].get_num_inspected()
        * monkeys[monkeys.len() - 2].get_num_inspected();
    print!("part2: {}", business);
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day11.txt")?;
    let mut timer = timing::start_benchmark(benchmark);

    part1(&input);
    timing::print_time(&mut timer);
    part2(&input);
    timing::print_time(&mut timer);

    Ok(())
}
