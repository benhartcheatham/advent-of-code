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
    sort: &[GraphID],
    start: GraphID,
    end: GraphID,
) -> usize {
    let mut counts = vec![0; graph.iter().map(|v| v.get_id()).max().unwrap() + 1];
    counts[end] = 1;

    for vid in sort
        .iter()
        .copied()
        .rev()
        .skip_while(|vid| *vid != end)
        .skip(1)
        .take_while_inclusive(|vid| *vid != start)
    {
        counts[vid] = graph
            .get_vertex(vid)
            .unwrap()
            .iter()
            .map(|e| counts[e.traverse()])
            .sum();
    }

    counts[start]
}

fn find_paths(graph: &mut Graph<i64>, sort: &[GraphID], nodes: &[GraphID]) -> usize {
    nodes
        .iter()
        .tuple_windows()
        .map(|(start, end)| find_paths_helper(graph, sort, *start, *end))
        .reduce(|acc, e| acc * e)
        .unwrap_or(0)
}

fn part1(input: &str) -> usize {
    let (mut graph, (inid, outid)) = parse_input(input, "you");

    if let Some(sort) = graph.topo_sort() {
        find_paths(&mut graph, &sort, &[inid, outid])
    } else {
        0
    }
}

fn part2(input: &str) -> usize {
    let (mut graph, (svrid, outid)) = parse_input(input, "svr");
    let fftid = graph.find_vertex_by_label("fft").unwrap().get_id();
    let dacid = graph.find_vertex_by_label("dac").unwrap().get_id();

    if let Some(sort) = graph.topo_sort() {
        find_paths(&mut graph, &sort, &[svrid, fftid, dacid, outid])
            + find_paths(&mut graph, &sort, &[svrid, dacid, fftid, outid])
    } else {
        0
    }
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
