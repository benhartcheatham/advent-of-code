use std::collections::HashMap;
use std::fs;
use std::io;

use aocutils::graph::*;
use aocutils::timeln;
use iter_tools::Itertools;

fn parse_input(input: &str, start: &str) -> (Graph<i64>, (GraphID, GraphID)) {
    let mut graph = Graph::new();

    for nodes in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        let label = &nodes[0][..nodes[0].len() - 1];
        let vid = if let Some(v) = graph.find_vertex_by_label(label) {
            v.get_id()
        } else {
            graph.add_vertex(0, Some(label))
        };

        for label in nodes.iter().skip(1) {
            let vid2 = if let Some(v2) = graph.find_vertex_by_label(label) {
                v2.get_id()
            } else {
                graph.add_vertex(0, Some(label))
            };

            graph.add_edge(vid, vid2, 1);
        }
    }

    let inid = graph.find_vertex_by_label(start).unwrap().get_id();
    let outid = graph.find_vertex_by_label("out").unwrap().get_id();

    (graph, (inid, outid))
}

fn find_paths_helper(
    graph: &Graph<i64>,
    start: GraphID,
    _end: GraphID,
    past: &mut HashMap<GraphID, usize>,
) -> usize {
    if let Some(count) = past.get(&start) {
        return *count;
    }

    let count = graph
        .get_vertex(start)
        .unwrap()
        .iter()
        .map(|e| find_paths_helper(graph, e.traverse(), _end, past))
        .sum();
    past.insert(start, count);
    count
}

fn find_paths(graph: &Graph<i64>, nodes: &[GraphID]) -> usize {
    nodes
        .iter()
        .tuple_windows()
        .map(|(start, end)| {
            let mut map = HashMap::new();
            map.insert(*end, 1);
            find_paths_helper(graph, *start, *end, &mut map)
        })
        .reduce(|acc, e| acc * e)
        .unwrap_or(0)
}

fn part1(input: &str) -> usize {
    let (graph, (inid, outid)) = parse_input(input, "you");

    find_paths(&graph, &[inid, outid])
}

fn part2(input: &str) -> usize {
    let (graph, (svrid, outid)) = parse_input(input, "svr");
    let fftid = graph.find_vertex_by_label("fft").unwrap().get_id();
    let dacid = graph.find_vertex_by_label("dac").unwrap().get_id();

    find_paths(&graph, &[svrid, fftid, dacid, outid])
        + find_paths(&graph, &[svrid, dacid, fftid, outid])
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2025/day11.txt");

    if let Err(e) = input {
        if e.kind() == io::ErrorKind::NotFound {
            println!("Input file not found!");
        }

        Err(e)
    } else {
        if benchmark {
            timeln!("part1: {}", part1(input.as_ref().unwrap()));
            timeln!("part2: {}", part2(input.as_ref().unwrap()));
        } else {
            println!("part1: {}", part1(input.as_ref().unwrap()));
            println!("part2: {}", part2(input.as_ref().unwrap()));
        }

        Ok(())
    }
}
