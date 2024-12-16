use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::graph::*;
use aocutils::grid::direction::GridDirection;
use aocutils::grid::direction::DIRECTIONS;
use aocutils::grid::in_ibounds;
use aocutils::timing::Timer;

fn insert_node_helper(coord: Coord, graph: &mut Graph<(Coord, GridDirection)>) -> Vec<usize> {
    use GridDirection::*;

    let ids: Vec<usize> = graph
        .iter()
        .filter(|v| v.data.0 == coord)
        .map(|v| v.get_id())
        .collect();

    if !ids.is_empty() {
        return ids;
    }

    let mut ids = Vec::new();
    for d in DIRECTIONS {
        let id = if let Some(v) = graph.iter().find(|v| v.data == (coord, d)) {
            v.get_id()
        } else {
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
            )
        };

        ids.push((d, id));
    }

    for (dir, id) in &ids {
        let adj = (dir.rotate_left(), dir.rotate_right());

        for (_, id2) in ids.iter().filter(|(d, _)| *d == adj.0 || *d == adj.1) {
            graph.add_edge(*id, *id2, 1000);
        }
    }

    ids.into_iter().map(|(_, id)| id).collect()
}

fn insert_node(
    coord: Coord,
    grid: &[Vec<char>],
    id_map: &mut HashMap<Coord, Vec<GraphID>>,
    graph: &mut Graph<(Coord, GridDirection)>,
) {
    let ids = id_map
        .entry(coord)
        .or_insert(insert_node_helper(coord, graph))
        .clone();

    for dir in DIRECTIONS.iter() {
        let vid = ids
            .iter()
            .find(|id| graph.get_vertex(**id).unwrap().data.1 == *dir)
            .unwrap();
        let c = coord + (*dir).into();

        if !in_ibounds(grid, c) {
            continue;
        }

        let (x, y) = c.as_unsigned().unwrap();
        if grid[x][y] == '#' {
            continue;
        }

        let adj = id_map.entry(c).or_insert(insert_node_helper(c, graph));
        let aid = adj
            .iter_mut()
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

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '#' {
                continue;
            }

            insert_node(
                Coord::new(i as i64, j as i64),
                &grid,
                &mut id_map,
                &mut graph,
            );
        }
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

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '#' {
                continue;
            }

            insert_node(
                Coord::new(i as i64, j as i64),
                &grid,
                &mut id_map,
                &mut graph,
            );
        }
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

    let mut path = Vec::new();
    graph.djikstra_path(*start, seid, &mut path);
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
