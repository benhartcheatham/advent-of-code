use std::collections::HashMap;
use std::fs;
use std::io;

use aocutils::timing::Timer;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GateKind {
    And,
    Xor,
    Or,
    Identity,
}

#[derive(Debug)]
struct Gate {
    inputs: (String, String),
    output: String,
    kind: GateKind,
}

impl Gate {
    fn new(inputs: (&str, &str), output: &str, kind: GateKind) -> Self {
        Gate {
            inputs: (inputs.0.to_owned(), inputs.1.to_owned()),
            output: output.to_owned(),
            kind,
        }
    }

    fn update(&self, gates: &Vec<Gate>, zs: &Vec<Gate>, inputs: &HashMap<String, bool>) -> bool {
        use GateKind::*;
        let update = |gates: &Vec<Gate>, input: &String| {
            gates
                .iter()
                .find(|g| &g.output == input)
                .unwrap()
                .update(gates, zs, inputs)
        };

        if self.kind == Identity {
            return inputs[&self.output];
        }

        let v1 = if self.inputs.0.starts_with('z') {
            update(zs, &self.inputs.0)
        } else {
            update(gates, &self.inputs.0)
        };

        let v2 = if self.inputs.1.starts_with('z') {
            update(zs, &self.inputs.1)
        } else {
            update(gates, &self.inputs.1)
        };

        match self.kind {
            And => v1 && v2,
            Or => v1 || v2,
            Xor => v1 != v2,
            _ => panic!(),
        }
    }

    fn is_swapped(&self) -> bool {
        if self.output.starts_with('z') {
            self.output != "z45" && self.kind != GateKind::Xor
        } else {
            if self.inputs.0.is_empty() || self.inputs.1.is_empty() {
                return false;
            }

            match (self.inputs.0.as_bytes()[0], self.inputs.1.as_bytes()[0]) {
                (b'x', b'y') | (b'y', b'x') => false,
                _ => self.kind == GateKind::Xor,
            }
        }
    }

    fn find_swapped_z(&self, gates: &Vec<Gate>, zs: &Vec<Gate>) -> usize {
        if self.output.starts_with('z') {
            return self
                .output
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
                - 1;
        }

        let gate = gates
            .iter()
            .find(|g| g.inputs.1 == self.output)
            .unwrap_or_else(|| {
                zs.iter()
                    .find(|z| z.inputs.0 == self.output || z.inputs.1 == self.output)
                    .unwrap()
            });

        gate.find_swapped_z(gates, zs)
    }
}

fn build_expected_z(inputs: &HashMap<String, bool>) -> usize {
    let filter = |pat| {
        inputs
            .keys()
            .filter(|k| k.starts_with(pat))
            .cloned()
            .collect::<Vec<String>>()
    };
    let to_usize = |bs: Vec<String>| {
        usize::from_str_radix(
            &bs.iter()
                .map(|k| if inputs[k] { '1' } else { '0' })
                .rev()
                .collect::<String>(),
            2,
        )
        .unwrap()
    };
    let mut ys = filter('y');
    ys.sort();
    let mut xs = filter('x');
    xs.sort();

    to_usize(xs) + to_usize(ys)
}

fn part1(input: &str) {
    use GateKind::*;

    let mut gates = Vec::new();
    let mut inputs = HashMap::new();
    let mut zs = Vec::new();

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() == 2 {
            let name = &parts[0][..parts[0].len() - 1];
            gates.push(Gate::new(("", ""), name, Identity));
            inputs.insert(name.to_string(), parts[1] == "1");
        } else if parts.len() == 5 {
            let in1 = parts[0];
            let in2 = parts[2];
            let output = parts[4];
            let kind = match parts[1] {
                "AND" => GateKind::And,
                "XOR" => GateKind::Xor,
                "OR" => GateKind::Or,
                _ => GateKind::Identity,
            };

            if output.starts_with('z') {
                zs.push(Gate::new((in1, in2), output, kind));
            } else {
                gates.push(Gate::new((in1, in2), output, kind));
            }
        }
    }

    zs.sort_by(|a, b| a.output.cmp(&b.output));
    let mut num = String::new();
    for z in &zs {
        num = format!("{}", if z.update(&gates, &zs, &inputs) { 1 } else { 0 }) + num.as_str();
    }

    print!("part1: {}", usize::from_str_radix(&num, 2).unwrap());
}

fn part2(input: &str) {
    use GateKind::*;

    let mut gates = Vec::new();
    let mut inputs = HashMap::new();
    let mut zs = Vec::new();

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() == 2 {
            let name = &parts[0][..parts[0].len() - 1];
            gates.push(Gate::new(("", ""), name, Identity));
            inputs.insert(name.to_string(), parts[1] == "1");
        } else if parts.len() == 5 {
            let in1 = parts[0];
            let in2 = parts[2];
            let output = parts[4];
            let kind = match parts[1] {
                "AND" => GateKind::And,
                "XOR" => GateKind::Xor,
                "OR" => GateKind::Or,
                _ => GateKind::Identity,
            };

            if output.starts_with('z') {
                zs.push(Gate::new((in1, in2), output, kind));
            } else {
                gates.push(Gate::new((in1, in2), output, kind));
            }
        }
    }

    let expected = build_expected_z(&inputs);
    zs.sort_by(|a, b| a.output.cmp(&b.output));

    let mut swapped_zs = HashMap::new();
    for (i, _) in zs.iter().enumerate().filter(|(_, z)| z.is_swapped()) {
        swapped_zs.insert(i, String::new());
    }

    for (idx, output) in gates
        .iter()
        .filter(|g| g.is_swapped())
        .map(|g| (g.find_swapped_z(&gates, &zs), g.output.clone()))
    {
        swapped_zs.entry(idx).and_modify(|s| *s = output);
    }

    for (k, v) in &swapped_zs {
        let gidx = gates
            .iter()
            .enumerate()
            .find(|(_, g)| g.output == *v)
            .unwrap()
            .0;
        let mut gate = gates.remove(gidx);
        let mut z = zs.remove(*k);

        let temp = gate.output.clone();
        gate.output = z.output.clone();
        z.output = temp;
        zs.insert(*k, gate);
        gates.insert(gidx, z);
    }

    let num = usize::from_str_radix(
        &zs.iter()
            .map(|z| z.update(&gates, &zs, &inputs))
            .map(|b| if b { '1' } else { '0' })
            .rev()
            .collect::<String>(),
        2,
    )
    .unwrap();

    let xor = num ^ expected;
    let idx = (0..usize::BITS)
        .find(|n| xor & (1 << n) > 0)
        .unwrap_or_else(|| {
            panic!("x and y wires need to be tweaked, xor of expected number returned 0")
        });

    let mut swapped = gates
        .iter()
        .filter(|g| {
            (g.inputs.0 == format!("x{:02}", idx) && g.inputs.1 == format!("y{:02}", idx))
                || (g.inputs.1 == format!("x{:02}", idx) && g.inputs.0 == format!("y{:02}", idx))
        })
        .map(|g| g.output.clone())
        .collect::<Vec<String>>();

    swapped.append(&mut Vec::from_iter(
        swapped_zs
            .into_iter()
            .flat_map(|(idx, swap)| vec![format!("z{:02}", idx), swap]),
    ));

    assert_eq!(swapped.len(), 8);
    swapped.sort();

    print!("part2: ");
    for s in swapped.iter().take(swapped.len() - 1) {
        print!("{},", s);
    }
    print!("{}", swapped[swapped.len() - 1]);
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day24.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
