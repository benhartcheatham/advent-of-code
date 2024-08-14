use std::collections::VecDeque;
use std::fmt::Display;
use std::fs;
use std::io;

#[derive(Debug, Clone, Eq)]
enum ListItem {
    List(Vec<ListItem>),
    Integer(i64),
}

impl ListItem {
    fn push(&mut self, item: ListItem) {
        if let ListItem::List(v) = self {
            v.push(item);
        }
    }

    fn parse(line: &mut VecDeque<&str>) -> Self {
        let mut list = ListItem::List(Vec::new());

        while let Some(s) = line.pop_front() {
            match s {
                "[" => list.push(Self::parse(line)),
                "]" => return list,
                _ => {
                    if let Ok(i) = s.parse::<i64>() {
                        list.push(ListItem::Integer(i));
                    } else {
                        return list;
                    }
                }
            }
        }

        list
    }

    fn new(line: &str) -> Self {
        let line = line.replace('[', "[,").replace(']', ",]");
        let mut line: VecDeque<&str> = line.split(',').collect();

        match Self::parse(&mut line) {
            ListItem::List(l) => l[0].clone(),
            _ => panic!(),
        }
    }
}

impl Display for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListItem::List(v) => {
                write!(f, "[")?;
                if !v.is_empty() {
                    for item in v.iter().take(v.len() - 1) {
                        write!(f, "{}, ", item)?;
                    }

                    write!(f, "{}", v[v.len() - 1])?;
                }

                write!(f, "]")
            }
            ListItem::Integer(i) => write!(f, "{}", i),
        }
    }
}

impl From<i64> for ListItem {
    fn from(value: i64) -> Self {
        ListItem::List(vec![ListItem::Integer(value)])
    }
}

impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ListItem::Integer(i), ListItem::Integer(j)) => i == j,
            (ListItem::Integer(i), ListItem::List(_)) => Into::<ListItem>::into(*i).eq(other),
            (ListItem::List(_), ListItem::Integer(i)) => self.eq(&Into::<ListItem>::into(*i)),
            (ListItem::List(v1), ListItem::List(v2)) => v1.eq(v2),
        }
    }
}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (ListItem::Integer(i), ListItem::Integer(j)) => i.cmp(j),
            (ListItem::Integer(i), ListItem::List(_)) => Into::<ListItem>::into(*i).cmp(other),
            (ListItem::List(_), ListItem::Integer(i)) => self.cmp(&Into::<ListItem>::into(*i)),
            (ListItem::List(v1), ListItem::List(v2)) => v1.cmp(v2),
        }
    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) {
    let mut packets: Vec<(ListItem, ListItem)> = Vec::new();

    let mut pair = (ListItem::Integer(0), ListItem::Integer(0));
    let mut i = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            packets.push(pair.clone());
            i = 0;
            continue;
        }

        match i {
            0 => pair.0 = ListItem::new(line),
            1 => pair.1 = ListItem::new(line),
            _ => panic!(),
        }

        i += 1;
    }

    println!(
        "part1: {}",
        packets
            .iter()
            .enumerate()
            .filter_map(|(i, (p1, p2))| if p1 < p2 { Some(i + 1) } else { None })
            .sum::<usize>()
    );
}

fn part2(input: &str) {
    let mut packets: Vec<ListItem> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        packets.push(ListItem::new(line));
    }

    let divider1 = ListItem::new("[[2]]");
    let divider2 = ListItem::new("[[6]]");
    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort();

    println!(
        "part2: {}",
        packets
            .iter()
            .enumerate()
            .filter_map(|(i, p)| if p == &divider1 || p == &divider2 {
                Some(i + 1)
            } else {
                None
            })
            .product::<usize>()
    );
}

pub fn run() -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day13.txt")?;

    part1(&input);
    part2(&input);

    Ok(())
}
