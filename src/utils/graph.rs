use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter, Result};

pub type GraphID = usize;

#[derive(Debug)]
pub struct Vertex<T: Ord + Clone> {
    pub label: String,
    id: GraphID,
    pub data: T,
    pub mark: bool,
    edges: Vec<Edge>,
    eid_alloc: GraphID,
}

#[allow(unused)]
impl<T: Ord + Clone> Vertex<T> {
    fn new(data: T, id: GraphID, label: Option<&str>) -> Self {
        Vertex {
            label: label.unwrap_or("").to_owned(),
            id,
            data,
            mark: false,
            edges: Vec::new(),
            eid_alloc: 0,
        }
    }

    fn get_label(&self) -> String {
        if self.label.is_empty() {
            format!("[{}]", self.id)
        } else {
            self.label.clone()
        }
    }

    fn next_eid(&mut self) -> GraphID {
        self.eid_alloc += 1;
        self.eid_alloc - 1
    }

    pub fn iter(&self) -> std::slice::Iter<Edge> {
        self.edges.iter()
    }

    pub fn get_id(&self) -> GraphID {
        self.id
    }

    pub fn mark(&mut self) {
        self.mark = !self.mark;
    }

    fn get_edge(&self, eid: GraphID) -> Option<&Edge> {
        self.edges.iter().find(|e| e.id == eid)
    }

    fn get_edge_mut(&mut self, eid: GraphID) -> Option<&mut Edge> {
        self.edges.iter_mut().find(|e| e.id == eid)
    }

    fn remove_edge(&mut self, eid: GraphID) {
        if let Some((i, _)) = self.edges.iter().enumerate().find(|(_, e)| e.id == eid) {
            self.edges.remove(i);
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Edge {
    from: GraphID,
    to: GraphID,
    pub weight: i64,
    id: GraphID,
}

#[allow(unused)]
impl Edge {
    fn new(from: GraphID, to: GraphID, weight: i64, id: GraphID) -> Self {
        Edge {
            from,
            id,
            to,
            weight,
        }
    }

    pub fn get_weight(&self) -> i64 {
        self.weight
    }

    pub fn get_id(&self) -> GraphID {
        self.id
    }

    pub fn traverse(&self) -> GraphID {
        self.to
    }
}

pub struct Graph<T: Ord + Clone> {
    vertices: Vec<Option<Vertex<T>>>,
    label_map: HashMap<String, GraphID>,
    vid_alloc: GraphID,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DState {
    vid: GraphID,
    dist: i64,
}

impl DState {
    fn new(vid: GraphID, dist: i64) -> Self {
        DState { vid, dist }
    }
}

impl Ord for DState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.vid.cmp(&other.vid))
    }
}

impl PartialOrd for DState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(unused)]
impl<T: Ord + Clone> Graph<T> {
    pub fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            label_map: HashMap::new(),
            vid_alloc: 0,
        }
    }

    fn next_vid(&mut self) -> GraphID {
        self.vid_alloc += 1;
        self.vid_alloc - 1
    }

    pub fn get_vertex(&self, id: GraphID) -> Option<&Vertex<T>> {
        self.vertices.get(id)?.as_ref()
    }

    pub fn get_vertex_mut(&mut self, id: GraphID) -> Option<&mut Vertex<T>> {
        self.vertices.get_mut(id)?.as_mut()
    }

    pub fn find_vertex_by_label(&self, label: &str) -> Option<GraphID> {
        self.label_map.get(label).copied()
    }

    pub fn add_vertex(&mut self, data: T, label: Option<&str>) -> GraphID {
        let id = self.next_vid();

        self.vertices.insert(id, Some(Vertex::new(data, id, label)));
        if let Some(label) = label {
            self.label_map.insert(label.to_string(), id);
        }

        id
    }

    pub fn add_edge(&mut self, vid1: GraphID, vid2: GraphID, weight: i64) -> Option<GraphID> {
        if self.get_vertex(vid1).is_none() || self.get_vertex(vid2).is_none() {
            return None;
        }

        let v1 = self.get_vertex_mut(vid1)?;
        let eid = v1.next_eid();

        let edge = Edge::new(vid1, vid2, weight, eid);
        v1.edges.insert(eid, edge);

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
            self.get_vertex_mut(v1).unwrap().remove_edge(eid1);
            return None;
        }

        Some((eid1, eid2.unwrap()))
    }

    pub fn remove_vertex(&mut self, id: GraphID) {
        if id < self.vertices.len() {
            self.vertices[id] = None;
        }
    }

    pub fn len(&self) -> usize {
        self.vertices.iter().filter(|v| v.is_some()).count()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Finds weight of shortest path from start to all other vertices in graph
    ///
    /// Return: Vec where tuple.0 is ID of vertex and tuple.1 is weight
    /// of shortest path
    pub fn djikstra(&self, start: GraphID) -> Vec<(GraphID, i64)> {
        let mut queue: BinaryHeap<DState> = BinaryHeap::new();
        let mut dist = vec![i64::MAX; self.len()];

        dist[start] = 0;

        queue.push(DState::new(start, dist[start]));
        while !queue.is_empty() {
            let u = queue.pop().unwrap();

            for e in &self.get_vertex(u.vid).unwrap().edges {
                let alt = dist[u.vid] + e.get_weight();
                let v = e.traverse();

                if alt < dist[v] {
                    dist[v] = alt;
                    queue.push(DState::new(v, alt));
                }
            }
        }

        dist.into_iter().enumerate().collect()
    }

    fn print_edge(&self, edge: &Edge) -> String {
        if let Some(v) = self.get_vertex(edge.traverse()) {
            v.get_label()
        } else {
            String::new()
        }
    }
}

impl<T: Ord + Clone> Default for Graph<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Clone> Display for Graph<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for v in self.vertices.iter().filter_map(|v| v.as_ref()) {
            write!(f, "{}: [", v.get_label())?;

            if !v.edges.is_empty() {
                for e in v.iter().take(v.edges.len() - 1) {
                    write!(f, "{}({}), ", self.print_edge(e), e.weight)?;
                }

                let idx = v.edges.len() - 1;
                write!(
                    f,
                    "{}({})",
                    self.print_edge(&v.edges[idx]),
                    &v.edges[idx].weight
                )?;
            }

            writeln!(f, "]")?;
        }

        Ok(())
    }
}

pub struct GraphIterator<'a, T: Ord + Clone> {
    current: GraphID,
    graph: &'a Graph<T>,
}

impl<'a, T: Ord + Clone> GraphIterator<'a, T> {
    fn new(graph: &'a Graph<T>) -> Self {
        GraphIterator { current: 0, graph }
    }
}

impl<'a, T: Ord + Clone> Iterator for GraphIterator<'a, T> {
    type Item = GraphID;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current < self.graph.vertices.len() {
            let ret = self.graph.vertices.get(self.current);
            self.current += 1;

            if ret.is_some() {
                return Some(self.current - 1);
            }
        }

        None
    }
}

#[allow(unused)]
impl<T: Ord + Clone> Graph<T> {
    pub fn iter(&self) -> GraphIterator<T> {
        GraphIterator::new(self)
    }
}
