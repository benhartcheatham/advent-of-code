use std::cell::RefCell;
use std::rc::{Rc, Weak};

type GraphID = usize;

#[allow(unused)]
pub struct Vertex<T: Ord + Eq + Clone> {
    label: Option<String>,
    id: GraphID,
    data: T,
    mark: bool,
    edges: Vec<Weak<Edge<T>>>,
}

#[allow(unused)]
impl<T: Ord + Eq + Clone> Vertex<T> {
    fn new(data: T, id: GraphID, label: Option<&str>) -> Self {
        Vertex {
            label: label.map(|s| s.to_string()),
            id,
            data,
            mark: false,
            edges: Vec::new(),
        }
    }

    pub fn mark(&mut self) {
        self.mark = !self.mark;
    }

    pub fn get_data(&self) -> T {
        self.data.clone()
    }

    pub fn set_data(&mut self, new: T) {
        self.data = new;
    }

    pub fn remove_edge(&mut self, id: GraphID) {
        let idx = self
            .edges
            .iter()
            .enumerate()
            .filter_map(|(i, we)| we.upgrade().map(|up| (i, up)))
            .find(|(_, e)| e.id == id);

        if let Some(idx) = idx {
            self.edges.remove(idx.0);
        }
    }
}

impl<T: Ord + Eq + Clone> From<Vertex<T>> for GraphID {
    fn from(value: Vertex<T>) -> Self {
        value.id
    }
}

#[allow(unused)]
pub struct Edge<T: Ord + Eq + Clone> {
    from: Weak<RefCell<Vertex<T>>>,
    to: Weak<RefCell<Vertex<T>>>,
    weight: i64,
    id: GraphID,
}

#[allow(unused)]
impl<T: Ord + Eq + Clone> Edge<T> {
    fn new(
        from: Weak<RefCell<Vertex<T>>>,
        to: Weak<RefCell<Vertex<T>>>,
        weight: i64,
        id: GraphID,
    ) -> Self {
        Edge {
            from,
            id,
            to,
            weight,
        }
    }

    pub fn traverse(&self) -> Option<GraphID> {
        self.to.upgrade().map(|v| v.borrow().id)
    }

    pub fn get_weight(&self) -> i64 {
        self.weight
    }
}

impl<T: Ord + Eq + Clone> From<Edge<T>> for GraphID {
    fn from(value: Edge<T>) -> Self {
        value.id
    }
}

#[allow(unused)]
struct Graph<T: Ord + Eq + Clone> {
    pub vertices: Vec<Rc<RefCell<Vertex<T>>>>,
    pub edges: Vec<Rc<Edge<T>>>,
    id_alloc: GraphID,
    iter: usize,
}

#[allow(unused)]
impl<T: Ord + Eq + Clone> Graph<T> {
    pub fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            edges: Vec::new(),
            id_alloc: 0,
            iter: 0,
        }
    }

    fn next_id(&mut self) -> GraphID {
        let ret = self.id_alloc;

        self.id_alloc += 1;
        ret
    }

    pub fn add_vertex(&mut self, data: T, label: Option<&str>) -> GraphID {
        let id = self.next_id();
        let ret = Rc::new(RefCell::new(Vertex::new(data, id, label)));

        self.vertices.push(Rc::clone(&ret));
        id
    }

    pub fn add_edge(&mut self, v1: GraphID, v2: GraphID, weight: i64) -> Option<GraphID> {
        let eid = self.next_id();
        let v1 = self.vertices.iter().find(|v| v.borrow().id == v1);
        let v2 = self.vertices.iter().find(|v| v.borrow().id == v2);

        if v1.is_none() || v2.is_none() {
            return None;
        }

        let edge = Rc::new(Edge::new(
            Rc::downgrade(v1.unwrap()),
            Rc::downgrade(v2.unwrap()),
            weight,
            eid,
        ));
        v1.unwrap().borrow_mut().edges.push(Rc::downgrade(&edge));
        self.edges.push(Rc::clone(&edge));

        Some(eid)
    }

    pub fn add_edge_bidirectional(
        &mut self,
        v1: GraphID,
        v2: GraphID,
        weight: i64,
    ) -> Option<(GraphID, GraphID)> {
        let eid1 = self.add_edge(v1, v2, weight)?;
        let eid2 = self.add_edge(v2, v1, weight);

        if eid2.is_none() {
            self.remove_edge(eid1);
            return None;
        }

        Some((eid1, eid2.unwrap()))
    }

    pub fn get_vertex(&self, id: GraphID) -> Option<Rc<RefCell<Vertex<T>>>> {
        self.vertices.iter().find(|v| v.borrow().id == id).cloned()
    }

    pub fn remove_vertex(&mut self, id: GraphID) {
        let idx = self
            .vertices
            .iter()
            .enumerate()
            .find(|(_, v)| v.borrow().id == id);

        if let Some(idx) = idx {
            self.vertices.remove(idx.0);
        }
    }

    pub fn remove_edge(&mut self, id: GraphID) {
        let idx = self.edges.iter().enumerate().find(|(_, e)| e.id == id);

        if let Some((idx, edge)) = idx {
            if let Some(v) = edge.from.upgrade() {
                v.borrow_mut().remove_edge(edge.id);
            }

            self.edges.remove(idx);
        }
    }
}
