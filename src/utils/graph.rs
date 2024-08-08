use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Vertex<T> {
    data: RefCell<T>,
    edges: RefCell<Vec<RefCell<Edge<T>>>>,
}

pub struct Edge<T> {
    mark: bool,
    weight: i64,
    to: Weak<Vertex<T>>,
}

#[allow(unused)]
pub struct Graph<T> {
    curr: usize,
    vertices: Vec<Rc<Vertex<T>>>,
}

impl<T> Vertex<T> {
    fn new(data: T) -> Self {
        Vertex {
            data: RefCell::new(data),
            edges: RefCell::new(Vec::new()),
        }
    }
}

#[allow(unused)]
impl<T: Copy> Vertex<T> {
    pub fn get_data(&self) -> T {
        *self.data.borrow()
    }

    pub fn set_data(&self, data: T) {
        *self.data.borrow_mut() = data;
    }
}

#[allow(unused)]
impl<T> Edge<T> {
    fn new(to: Rc<Vertex<T>>, weight: i64) -> Self {
        Edge {
            mark: false,
            weight,
            to: Rc::downgrade(&to),
        }
    }

    pub fn mark(&mut self) -> bool {
        self.mark = !self.mark;
        self.mark
    }

    pub fn marked(&self) -> bool {
        self.mark
    }

    pub fn follow(&self) -> Option<Rc<Vertex<T>>> {
        self.to.upgrade()
    }

    pub fn update_weight(&mut self, new: i64) -> i64 {
        let old = self.weight;

        self.weight = new;
        old
    }
}

#[allow(unused)]
impl<T: Copy> Graph<T> {
    pub fn new() -> Self {
        Graph {
            curr: 0,
            vertices: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, data: T) -> Rc<Vertex<T>> {
        let new = Rc::new(Vertex::new(data));
        self.vertices.push(Rc::clone(&new));
        new
    }

    pub fn connect(&self, from: Rc<Vertex<T>>, to: Rc<Vertex<T>>, weight: i64) {
        from.edges
            .borrow_mut()
            .push(RefCell::new(Edge::new(to, weight)));
    }
}

impl<T> Iterator for Graph<T> {
    type Item = Rc<Vertex<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.vertices.len() {
            None
        } else {
            let v = &self.vertices[self.curr];
            self.curr += 1;
            Some(Rc::clone(v))
        }
    }
}
