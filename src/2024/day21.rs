use std::collections::HashMap;
use std::fs;
use std::io;

use aocutils::coord::Coord;
use aocutils::graph::*;
use aocutils::grid::direction::GridDirection;
use aocutils::timing::Timer;

type PathMap = HashMap<(u8, u8), Vec<Vec<u8>>>;

fn gen_paths(grid: Vec<Vec<u8>>) -> PathMap {
    use GridDirection::*;

    let mut graph = Graph::new();
    let mut map = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, u) in row.iter().enumerate() {
            if grid[i][j] != b'.' {
                graph.add_vertex(
                    Coord::new(i as i64, j as i64),
                    Some(&format!("{}", *u as char)),
                );
            }
        }
    }

    let mut pairs = Vec::new();
    for v in graph.iter() {
        for v2 in graph.iter().filter(|v2| v.data.manhattan(&v2.data) == 1) {
            pairs.push(((v.get_id(), v.data), (v2.get_id(), v2.data)));
        }
    }

    for ((vid1, c1), (vid2, c2)) in pairs {
        if let Some(dir) = (c1 - c2).into() {
            let weight = match dir {
                Left => 1,
                Up => 2,
                Down => 3,
                Right => 4,
            };

            graph.add_edge(vid1, vid2, weight);
        }
    }

    let ids = graph.iter().map(|v| v.get_id()).collect::<Vec<GraphID>>();

    for i in 0..ids.len() {
        for j in 0..ids.len() {
            let ch1 = graph.get_vertex(i).unwrap().label.as_bytes()[0];
            let ch2 = graph.get_vertex(j).unwrap().label.as_bytes()[0];

            if i == j {
                map.insert((ch1, ch2), vec![vec![b'A']]);
                continue;
            }

            let paths: Vec<Vec<u8>> = graph
                .djikstra_all_paths(i, j)
                .iter()
                .map(|v| {
                    let mut path = Vec::new();
                    for i in 0..(v.len() - 1) {
                        let c1 = graph.get_vertex(v[i]).unwrap().data;
                        let c2 = graph.get_vertex(v[i + 1]).unwrap().data;

                        path.push(match (c2 - c1).into() {
                            Some(Up) => b'^',
                            Some(Down) => b'v',
                            Some(Left) => b'<',
                            Some(Right) => b'>',
                            _ => panic!("Invalid path step {} -> {}", c1, c2),
                        });
                    }

                    path.push(b'A');
                    path
                })
                .collect();

            map.insert((ch1, ch2), paths);
        }
    }

    map
}

fn find_paths(ch1: u8, ch2: u8, paths: &PathMap) -> &Vec<Vec<u8>> {
    paths
        .get(&(ch1, ch2))
        .unwrap_or_else(|| panic!("No path between {} and {}", ch1 as char, ch2 as char))
}

fn find_shortest<'a>(
    ch1: u8,
    ch2: u8,
    depth: usize,
    max_depth: usize,
    npad_paths: &'a PathMap,
    dpad_paths: &'a PathMap,
    past_paths: &mut HashMap<((u8, u8), usize), usize>,
) -> usize {
    if let Some(min) = past_paths.get(&((ch1, ch2), depth)) {
        return *min;
    }

    let pad = if depth == 0 { npad_paths } else { dpad_paths };
    let paths = find_paths(ch1, ch2, pad);

    if depth == max_depth {
        let min = paths.iter().map(|p| p.len()).min().unwrap();
        past_paths.insert(((ch1, ch2), depth), min);
        return min;
    }

    let mut min = usize::MAX;
    for path in paths {
        let mut len = find_shortest(
            b'A',
            path[0],
            depth + 1,
            max_depth,
            npad_paths,
            dpad_paths,
            past_paths,
        );

        for i in 0..(path.len() - 1) {
            let res = find_shortest(
                path[i],
                path[i + 1],
                depth + 1,
                max_depth,
                npad_paths,
                dpad_paths,
                past_paths,
            );

            len += res;
        }

        min = min.min(len);
    }

    past_paths.insert(((ch1, ch2), depth), min);
    min
}

fn part1(input: &str) {
    let npad = vec![
        vec![b'7', b'8', b'9'],
        vec![b'4', b'5', b'6'],
        vec![b'1', b'2', b'3'],
        vec![b'.', b'0', b'A'],
    ];
    let dpad = vec![vec![b'.', b'^', b'A'], vec![b'<', b'v', b'>']];
    let npad_paths = gen_paths(npad);
    let dpad_paths = gen_paths(dpad);
    let mut past_map = HashMap::new();

    let mut complexity = 0;
    for line in input.lines().map(|line| line.as_bytes()) {
        let n = line[..line.len() - 1]
            .iter()
            .map(|u| *u as char)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let mut len = 0;

        let mut temp = line.to_vec();
        temp.insert(0, b'A');
        let line = temp;

        for i in 0..(line.len() - 1) {
            let l = find_shortest(
                line[i],
                line[i + 1],
                0,
                2,
                &npad_paths,
                &dpad_paths,
                &mut past_map,
            );
            len += l;
        }

        complexity += len * n;
    }

    print!("part1: {}", complexity);
}

fn part2(input: &str) {
    let npad = vec![
        vec![b'7', b'8', b'9'],
        vec![b'4', b'5', b'6'],
        vec![b'1', b'2', b'3'],
        vec![b'.', b'0', b'A'],
    ];
    let dpad = vec![vec![b'.', b'^', b'A'], vec![b'<', b'v', b'>']];
    let npad_paths = gen_paths(npad);
    let dpad_paths = gen_paths(dpad);
    let mut past_map = HashMap::new();

    let mut complexity = 0;
    for line in input.lines().map(|line| line.as_bytes()) {
        let n = line[..line.len() - 1]
            .iter()
            .map(|u| *u as char)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let mut len = 0;

        let mut temp = line.to_vec();
        temp.insert(0, b'A');
        let line = temp;

        for i in 0..(line.len() - 1) {
            let l = find_shortest(
                line[i],
                line[i + 1],
                0,
                25,
                &npad_paths,
                &dpad_paths,
                &mut past_map,
            );
            len += l;
        }

        complexity += len * n;
    }

    print!("part2: {}", complexity);
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day21.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_path(input: &[u8], past_map: &mut HashMap<((u8, u8), usize), usize>) -> usize {
        let npad = vec![
            vec![b'7', b'8', b'9'],
            vec![b'4', b'5', b'6'],
            vec![b'1', b'2', b'3'],
            vec![b'.', b'0', b'A'],
        ];
        let dpad = vec![vec![b'.', b'^', b'A'], vec![b'<', b'v', b'>']];
        let npad_paths = gen_paths(npad);
        let dpad_paths = gen_paths(dpad);

        let mut len = 0;
        for i in 0..(input.len() - 1) {
            println!("ch1: {} ch2: {}", input[i] as char, input[i + 1] as char);
            len += find_shortest(
                input[i],
                input[i + 1],
                0,
                2,
                &npad_paths,
                &dpad_paths,
                past_map,
            );
        }

        len
    }

    #[test]
    fn code_029a() {
        let len = 68;

        assert_eq!(len, get_path("A029A".as_bytes(), &mut HashMap::new()));
    }

    #[test]
    fn code_980a() {
        let len = 60;

        assert_eq!(len, get_path("A980A".as_bytes(), &mut HashMap::new()));
    }

    #[test]
    fn code_179a() {
        let len = 68;

        assert_eq!(len, get_path("A179A".as_bytes(), &mut HashMap::new()));
    }

    #[test]
    fn code_456a() {
        let len = 64;

        assert_eq!(len, get_path("A456A".as_bytes(), &mut HashMap::new()));
    }

    #[test]
    fn code_379a() {
        let len = 64;

        assert_eq!(len, get_path("A379A".as_bytes(), &mut HashMap::new()));
    }
}
