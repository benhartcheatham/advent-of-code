use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

use aocutils::graph::*;
use aocutils::timing::Timer;

fn part1(input: &str) {
    let mut network = Graph::new();
    let mut edges = Vec::new();
    let mut nodes = HashSet::new();
    let mut map = HashMap::new();

    for line in input.lines() {
        let computers = line.trim().split('-').collect::<Vec<&str>>();
        nodes.insert(computers[0]);
        nodes.insert(computers[1]);
        edges.push((computers[0], computers[1]));
    }

    for node in nodes {
        map.insert(node, network.add_vertex(node, Some(node)));
    }

    for (v1, v2) in edges
        .iter()
        .map(|(v1, v2)| (*map.get(v1).unwrap(), *map.get(v2).unwrap()))
    {
        network.add_edge_bidirectional(v1, v2, 0);
    }

    print!(
        "part1: {}",
        network
            .complete(3)
            .iter()
            .filter(|clique| clique.iter().any(|v| network
                .get_vertex(*v)
                .unwrap()
                .label
                .starts_with('t')))
            .count()
    );
}

fn part2(input: &str) {
    let mut network = Graph::new();
    let mut edges = Vec::new();
    let mut nodes = HashSet::new();
    let mut map = HashMap::new();

    for line in input.lines() {
        let computers = line.trim().split('-').collect::<Vec<&str>>();
        nodes.insert(computers[0]);
        nodes.insert(computers[1]);
        edges.push((computers[0], computers[1]));
    }

    for node in nodes {
        map.insert(node, network.add_vertex(node, Some(node)));
    }

    for (v1, v2) in edges
        .iter()
        .map(|(v1, v2)| (*map.get(v1).unwrap(), *map.get(v2).unwrap()))
    {
        network.add_edge_bidirectional(v1, v2, 0);
    }

    let mut names = network
        .bron_kerbosch()
        .iter()
        .map(|id| network.get_vertex(*id).unwrap().label.as_str())
        .collect::<Vec<&str>>();
    names.sort();

    print!("part2: ");

    for n in names.iter().take(names.len() - 1) {
        print!("{},", n);
    }

    print!("{}", names[names.len() - 1]);
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day23.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
