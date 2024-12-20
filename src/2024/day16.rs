use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::graph::*;
use aocutils::grid::direction::GridDirection;
use aocutils::grid::direction::DIRECTIONS;
use aocutils::grid::in_bounds;
use aocutils::timing::Timer;

fn insert_node(
    coord: Coord,
    id_map: &mut HashMap<Coord, Vec<GraphID>>,
    graph: &mut Graph<(Coord, GridDirection)>,
) {
    use GridDirection::*;

    let mut nodes = Vec::new();
    for d in DIRECTIONS {
        nodes.push((
            d,
            graph.add_vertex(
                (coord, d),
                Some(&format!(
                    "{}{}",
                    coord,
                    match d {
                        Up => 'N',
                        Down => 'S',
                        Left => 'W',
                        Right => 'E',
                    }
                )),
            ),
        ));
    }

    for (dir, id) in &nodes {
        let adj = (dir.rotate_left(), dir.rotate_right());

        for (_, id2) in nodes.iter().filter(|(d, _)| *d == adj.0 || *d == adj.1) {
            graph.add_edge(*id, *id2, 1000);
        }
    }

    id_map.insert(coord, nodes.into_iter().map(|(_, id)| id).collect());
}

fn connect_nodes(
    coord: Coord,
    grid: &[Vec<char>],
    id_map: &HashMap<Coord, Vec<GraphID>>,
    graph: &mut Graph<(Coord, GridDirection)>,
) {
    for dir in DIRECTIONS.iter() {
        let vid = id_map
            .get(&coord)
            .unwrap()
            .iter()
            .find(|id| graph.get_vertex(**id).unwrap().data.1 == *dir)
            .unwrap();
        let c = coord + (*dir).into();

        if !in_bounds(grid, c) {
            continue;
        }

        let (x, y) = c.as_unsigned().unwrap();
        if grid[x][y] == '#' {
            continue;
        }

        let adj = id_map.get(&c).unwrap();
        let aid = adj
            .iter()
            .find(|aid| graph.get_vertex(**aid).unwrap().data.1 == *dir)
            .unwrap();

        graph.add_edge(*vid, *aid, 1);
    }
}

fn part1(input: &str) {
    let mut grid = Vec::new();
    let mut start = Coord::new(0, 0);
    let mut end = Coord::new(0, 0);

    for (i, line) in input.lines().enumerate() {
        grid.push(Vec::new());

        for (j, ch) in line.char_indices() {
            match ch {
                'S' => {
                    start = Coord::new(i as i64, j as i64);
                    grid[i].push(ch);
                }
                'E' => {
                    end = Coord::new(i as i64, j as i64);
                    grid[i].push(ch);
                }
                _ => grid[i].push(ch),
            }
        }
    }

    let mut graph = Graph::new();
    let mut id_map = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == '#' {
                continue;
            }

            insert_node(
                Coord::new(i as i64, j as i64),
                &mut id_map,
                &mut graph,
            );
        }
    }

    for k in id_map.keys() {
        connect_nodes(*k, &grid, &id_map, &mut graph);
    }

    let start = id_map
        .get(&start)
        .unwrap()
        .iter()
        .find(|id| graph.get_vertex(**id).unwrap().data.1 == GridDirection::Right)
        .unwrap();
    let paths = graph.djikstra(*start);

    print!(
        "part1: {}",
        id_map
            .get(&end)
            .unwrap()
            .iter()
            .map(|eid| paths
                .iter()
                .find(|(vid, _)| *vid == *eid)
                .unwrap_or(&(0, 0))
                .1)
            .min()
            .unwrap()
    );
}

fn part2(input: &str) {
    let mut grid = Vec::new();
    let mut start = Coord::new(0, 0);
    let mut end = Coord::new(0, 0);

    for (i, line) in input.lines().enumerate() {
        grid.push(Vec::new());

        for (j, ch) in line.char_indices() {
            match ch {
                'S' => {
                    start = Coord::new(i as i64, j as i64);
                    grid[i].push(ch);
                }
                'E' => {
                    end = Coord::new(i as i64, j as i64);
                    grid[i].push(ch);
                }
                _ => grid[i].push(ch),
            }
        }
    }

    let mut graph = Graph::new();
    let mut id_map = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == '#' {
                continue;
            }

            insert_node(
                Coord::new(i as i64, j as i64),
                &mut id_map,
                &mut graph,
            );
        }
    }

    for k in id_map.keys() {
        connect_nodes(*k, &grid, &id_map, &mut graph);
    }

    let start = id_map
        .get(&start)
        .unwrap()
        .iter()
        .find(|id| graph.get_vertex(**id).unwrap().data.1 == GridDirection::Right)
        .unwrap();
    let paths = graph.djikstra(*start);
    let (mut seid, mut min) = (0, i64::MAX);

    for eid in id_map.get(&end).unwrap() {
        for (id, cost) in paths.iter().filter(|(id, _)| *id == *eid) {
            if *cost < min {
                seid = *id;
                min = *cost;
            }
        }
    }

    let mut set = HashSet::new();
    let paths = graph.djikstra_all_paths(*start, seid);

    for path in paths {
        for v in path.iter().map(|id| graph.get_vertex(*id).unwrap()) {
            set.insert(v.data.0);
        }
    }

    print!("part2: {}", set.len());
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day16.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
