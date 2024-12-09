use std::fs;
use std::io;

use aocutils::timing::Timer;

#[derive(Debug, Clone, Copy)]
struct Block {
    id: usize,
    free: bool,
}

impl Block {
    fn new(id: usize, free: bool) -> Self {
        Block { id, free }
    }

    fn is_free(&self) -> bool {
        self.free
    }
}

struct FreeSpace {
    idx: usize,
    len: usize,
}

struct File {
    id: usize,
    idx: usize,
    len: usize,
}

impl File {
    fn new(id: usize, idx: usize, len: usize) -> Self {
        File { id, idx, len }
    }

    fn void(&self, blocks: &mut [Block]) {
        memset(blocks, self.idx, self.len, Block::new(0, true));
    }
}

impl FreeSpace {
    fn new(idx: usize, len: usize) -> Self {
        FreeSpace { idx, len }
    }

    fn can_contain(&self, file: &File) -> bool {
        self.len >= file.len
    }

    /// Returns whether free space was filled
    fn swap(&mut self, blocks: &mut [Block], file: &mut File) {
        if !self.can_contain(file) {
            return;
        }

        memset(blocks, self.idx, file.len, Block::new(file.id, false));
        file.void(blocks);

        self.len -= file.len;
        self.idx += file.len;
    }
}

fn memset(blocks: &mut [Block], idx: usize, len: usize, block: Block) {
    for i in 0..len {
        blocks[idx + i] = block;
    }
}

fn part1(input: &str) {
    let mut blocks = Vec::new();
    let mut id = 0;

    for (i, ch) in input.trim().char_indices() {
        let n = ch.to_digit(10).unwrap() as usize;
        let free = i % 2 != 0;

        for _ in 0..n {
            blocks.push(Block::new(id, free));
        }

        if !free {
            id += 1;
        }
    }

    let (mut i, mut j) = (0, blocks.len() - 1);
    while i < j {
        if !blocks[i].is_free() {
            i += 1;
            continue;
        }

        if blocks[j].is_free() {
            j -= 1;
            continue;
        }

        blocks.swap(i, j);
        i += 1;
        j -= 1;
    }

    print!(
        "part1: {}",
        blocks
            .iter()
            .enumerate()
            .filter(|(_, b)| !b.is_free())
            .map(|(i, b)| i * b.id)
            .sum::<usize>()
    );
}

fn part2(input: &str) {
    let mut blocks = Vec::new();
    let mut files = Vec::new();
    let mut freespace = Vec::new();
    let mut id = 0;
    let mut idx = 0;

    for (i, ch) in input.trim().char_indices() {
        let n = ch.to_digit(10).unwrap() as usize;
        let free = i % 2 != 0;

        for _ in 0..n {
            blocks.push(Block::new(id, free));
        }

        if !free {
            files.push(File::new(id, idx, n));
            id += 1;
        } else {
            freespace.push(FreeSpace::new(idx, n));
        }

        idx += n;
    }

    for f in files.iter_mut().rev() {
        for s in freespace.iter_mut().take_while(|s| s.idx < f.idx) {
            if s.can_contain(f) {
                s.swap(&mut blocks, f);
                break;
            }
        }
    }

    print!(
        "part2: {}",
        blocks
            .iter()
            .enumerate()
            .filter(|(_, b)| !b.is_free())
            .map(|(i, b)| i * b.id)
            .sum::<usize>()
    );
}

pub fn run(benchmark: bool) -> io::Result<()> {
    let input = fs::read_to_string("inputs/2024/day9.txt")?;
    let mut timer = Timer::new(benchmark);

    timer.time(part1, &input);
    timer.time(part2, &input);

    Ok(())
}
