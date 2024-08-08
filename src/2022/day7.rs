use std::cell::RefCell;
use std::fs;
use std::io;
use std::rc::Rc;
use std::rc::Weak;

struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        File {
            name: String::from(name),
            size,
        }
    }
}

struct Dir {
    name: String,
    parent: Option<Weak<RefCell<Dir>>>,
    subdirs: Vec<Rc<RefCell<Dir>>>,
    files: Vec<File>,
    size: usize,
}

impl Dir {
    fn new(name: &str) -> Self {
        Dir {
            name: String::from(name),
            parent: None,
            subdirs: Vec::new(),
            files: Vec::new(),
            size: 0,
        }
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file)
    }

    fn add_dir(&mut self, dir: Dir) {
        self.subdirs.push(Rc::new(RefCell::new(dir)));
    }

    fn set_parent(&mut self, dir: Weak<RefCell<Dir>>) {
        self.parent = Some(dir);
    }

    fn update_size(&mut self) {
        let mut sdsize = 0;
        for d in &self.subdirs {
            d.borrow_mut().update_size();
            sdsize += d.borrow().size;
        }

        self.size = self.files.iter().map(|f| f.size).sum::<usize>() + sdsize;
    }

    #[allow(unused)]
    fn print(&self, level: usize) {
        println!("{}{} {}", " ".repeat(level * 4), self.name, self.size);

        for d in &self.subdirs {
            d.borrow().print(level + 1);
        }

        for f in &self.files {
            println!("{}{} {}", " ".repeat((level + 1) * 4), f.name, f.size);
        }
    }
}

fn cd(cwd: Rc<RefCell<Dir>>, dir: &str) -> Rc<RefCell<Dir>> {
    match dir {
        ".." => Rc::clone(&cwd.borrow().parent.as_ref().unwrap().upgrade().unwrap()),
        _ => Rc::clone(
            cwd.borrow()
                .subdirs
                .iter()
                .find(|d| d.borrow().name == dir)
                .unwrap(),
        ),
    }
}

fn parse_input(root: &Rc<RefCell<Dir>>, input: &str) {
    let mut cwd = Rc::clone(root);

    for line in input.lines().skip(1) {
        let parts: Vec<&str> = line.split(' ').collect();

        if parts[0] == "$" {
            if parts[1] == "cd" {
                cwd = cd(cwd, parts[2]);
            }
        } else {
            match parts[0] {
                "dir" => {
                    let mut dir = Dir::new(parts[1]);
                    dir.set_parent(Rc::downgrade(&cwd));
                    cwd.borrow_mut().add_dir(dir);
                }
                _ => cwd
                    .borrow_mut()
                    .add_file(File::new(parts[1], parts[0].parse::<usize>().unwrap())),
            }
        }
    }
}

fn part1_helper(root: &Rc<RefCell<Dir>>, size: usize) -> usize {
    let rsize = if root.borrow().size < size {
        root.borrow().size
    } else {
        0
    };

    rsize
        + root
            .borrow()
            .subdirs
            .iter()
            .map(|d| part1_helper(d, size))
            .sum::<usize>()
}

fn part1(input: &str) {
    let root = Rc::new(RefCell::new(Dir::new("/")));

    parse_input(&root, input);

    root.borrow_mut().update_size();
    println!("part1: {}", part1_helper(&root, 100_000));
}

fn part2_helper(root: &Rc<RefCell<Dir>>, size: usize) -> usize {
    let rsize = if root.borrow().size >= size {
        root.borrow().size
    } else {
        usize::MAX
    };

    usize::min(
        rsize,
        root.borrow()
            .subdirs
            .iter()
            .map(|d| part2_helper(d, size))
            .min()
            .unwrap_or(usize::MAX),
    )
}
fn part2(input: &str) {
    let root = Rc::new(RefCell::new(Dir::new("/")));

    parse_input(&root, input);

    root.borrow_mut().update_size();
    let size_needed = 30_000_000 - (70_000_000 - root.borrow().size);
    println!("part2: {}", part2_helper(&root, size_needed));
}

pub fn run() -> io::Result<()> {
    let input = fs::read_to_string("inputs/2022/day7.txt")?;

    part1(&input);
    part2(&input);

    Ok(())
}
