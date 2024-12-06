use std::collections::VecDeque;
use std::fs;
use std::io;

use aocutils::graph::*;
use aocutils::timing;

#[derive(PartialEq, Eq, Debug, Clone)]
struct ValveState {
    vid: GraphID,
    time: u64,
    pressure: u64,
    open: u64,
}

impl ValveState {
    fn new(vid: GraphID, time: u64, pressure: u64, open: u64) -> Self {
        ValveState {
            vid,
            time,
            pressure,
            open,
        }
    }

    fn is_open(&self, vid: GraphID) -> bool {
        (self.open & (1 << vid)) > 0
    }

    fn push(&mut self, vid: GraphID) {
        self.open |= 1 << vid;
    }

    fn is_reachable(&self, e: &Edge, vid: GraphID, start_time: u64) -> bool {
        let reachable = (self.time + e.weight as u64) < start_time;
        !self.is_open(vid) && reachable
    }

    fn shares_open(&self, other: &ValveState) -> bool {
        (self.open & other.open) > 0
    }
}

fn calc_pressure(start: GraphID, start_time: u64, graph: &Graph<u64>) -> (u64, Vec<ValveState>) {
    let mut queue = VecDeque::new();
    let mut states = Vec::new();
    let mut max_p = 0;

    if start_time == 0 {
        return (0, states);
    }

    queue.push_back(ValveState::new(start, 0, 0, 0));
    while !queue.is_empty() {
        let mut state = queue.pop_front().unwrap();
        let v = graph.get_vertex(state.vid).unwrap();

        if v.data > 0 {
            state.pressure += v.data * (start_time - state.time - 1);
            state.time += 1;
            state.push(v.get_id());
            max_p = max_p.max(state.pressure);
        }

        states.push(state.clone());

        for e in v.iter() {
            let v2 = match graph.get_vertex(e.traverse()) {
                Some(v) => v,
                None => continue,
            };

            if state.is_reachable(e, v2.get_id(), start_time) {
                queue.push_back(ValveState::new(
                    v2.get_id(),
                    state.time + e.weight as u64,
                    state.pressure,
                    state.open,
                ));
            }
        }
    }

    (max_p, states)
}

fn prune_graph(graph: Graph<u64>) -> Graph<u64> {
    let mut pruned = Graph::new();

    for vid in graph.iter() {
        let v = graph.get_vertex(vid).unwrap();
        let paths = graph.djikstra(vid);

        let pid1 = pruned
            .find_vertex_by_label(&v.label)
            .unwrap_or_else(|| pruned.add_vertex(v.data, Some(&v.label)));
        for (vid2, w) in paths {
            let v2 = graph.get_vertex(vid2).unwrap();

            if v2.data == 0 || vid2 == vid {
                continue;
            }

            let pid2 = pruned
                .find_vertex_by_label(&v2.label)
                .unwrap_or_else(|| pruned.add_vertex(v2.data, Some(&v2.label)));

            pruned.add_edge(pid1, pid2, w);
        }
    }

    pruned
}

fn part1(input: &str) {
    let mut graph: Graph<u64> = Graph::new();

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let label = words[1];
        let rate = words[4].split('=').collect::<Vec<&str>>()[1]
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let tunnels = &words[9..];

        let vid = graph
            .find_vertex_by_label(label)
            .unwrap_or_else(|| graph.add_vertex(0, Some(label)));
        graph.get_vertex_mut(vid).unwrap().data = rate;

        for label2 in tunnels
            .iter()
            .map(|s| s.chars().filter(|c| c.is_alphabetic()).collect::<String>())
        {
            let vid2 = graph
                .find_vertex_by_label(&label2)
                .unwrap_or_else(|| graph.add_vertex(0, Some(&label2)));

            graph.add_edge(vid, vid2, 1);
        }
    }

    graph = prune_graph(graph);
    print!(
        "part1: {}",
        calc_pressure(graph.find_vertex_by_label("AA").unwrap(), 30, &graph).0
    );
}

fn part2(input: &str) {
    let mut graph: Graph<u64> = Graph::new();

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let label = words[1];
        let rate = words[4].split('=').collect::<Vec<&str>>()[1]
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let tunnels = &words[9..];

        let vid = graph
            .find_vertex_by_label(label)
            .unwrap_or_else(|| graph.add_vertex(0, Some(label)));
        graph.get_vertex_mut(vid).unwrap().data = rate;

        for label2 in tunnels
            .iter()
            .map(|s| s.chars().filter(|c| c.is_alphabetic()).collect::<String>())
        {
            let vid2 = graph
                .find_vertex_by_label(&label2)
                .unwrap_or_else(|| graph.add_vertex(0, Some(&label2)));

            graph.add_edge(vid, vid2, 1);
        }
    }

    graph = prune_graph(graph);
    let start_id = graph.find_vertex_by_label("AA").unwrap();
    let (_, states) = calc_pressure(start_id, 26, &graph);
    let mut max = 0;
    for i in 0..states.len() {
        for j in (i + 1)..states.len() {
            if !states[i].shares_open(&states[j]) {
                max = max.max(states[i].pressure + states[j].pressure);
            }
        }
    }

    print!("part2: {}", max);
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day16.txt")?;
    let mut timer = timing::start_benchmark(benchmark);

    part1(&input);
    timing::print_time(&mut timer);
    part2(&input);
    timing::print_time(&mut timer);

    Ok(())
}
