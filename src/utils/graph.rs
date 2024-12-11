use std::collections::hash_map::{Values, ValuesMut};
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter, Result};

// ID used by graphs to identify vertices
pub type GraphID = usize;

/* VERTEX IMPLEMENTATION */

#[derive(Debug)]
pub struct Vertex<T: Ord + Clone> {
    id: GraphID,
    pub data: T,
    pub mark: bool,
    edges: Vec<Edge>,
    pub label: String,
}

impl<T: Ord + Clone> Vertex<T> {
    fn new(data: T, id: GraphID, label: &str) -> Self {
        Vertex {
            label: label.to_owned(),
            id,
            data,
            mark: false,
            edges: Vec::new(),
        }
    }

    /// Creates an iterator over the outgoing Edges
    pub fn iter(&self) -> std::slice::Iter<Edge> {
        self.edges.iter()
    }

    /// Gets the GraphID of this Vertex
    pub fn get_id(&self) -> GraphID {
        self.id
    }

    /// Inverts the mark attribute of this Vertex
    pub fn mark(&mut self) {
        self.mark = !self.mark;
    }

    /// Removes the first Edge from this Vertex to Vertex with id = vid
    pub fn remove_edge(&mut self, vid: GraphID) -> Option<Edge> {
        if let Some((i, _)) = self.edges.iter().enumerate().find(|(_, e)| e.to == vid) {
            Some(self.edges.remove(i))
        } else {
            None
        }
    }

    /// Removes all Edges from this Vertex to Vertex with id = vid
    pub fn remove_edges(&mut self, vid: GraphID) {
        loop {
            if self.remove_edge(vid).is_none() {
                break;
            }
        }
    }
}

/* EDGE IMPLEMENTATION */

#[derive(Debug)]
pub struct Edge {
    pub weight: i64,
    to: GraphID,
}

impl Edge {
    fn new(to: GraphID, weight: i64) -> Self {
        Edge { to, weight }
    }

    /// Gets the weight of this Edge
    pub fn get_weight(&self) -> i64 {
        self.weight
    }

    /// Gets the GraphID of the Vertex this Edge points to
    pub fn traverse(&self) -> GraphID {
        self.to
    }
}

/* GRAPH IMPLEMENTATION */

pub struct Graph<T: Ord + Clone> {
    vertices: HashMap<GraphID, Vertex<T>>,
    vid_alloc: GraphID,
}

impl<T: Ord + Clone> Graph<T> {
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::new(),
            vid_alloc: 0,
        }
    }

    fn next_vid(&mut self) -> GraphID {
        self.vid_alloc += 1;
        self.vid_alloc - 1
    }

    /// Gets a reference to the Vertex with id = @id, or None if said Vertex doesn't exist
    pub fn get_vertex(&self, id: GraphID) -> Option<&Vertex<T>> {
        self.vertices.get(&id)
    }

    /// Gets a mutable reference to the Vertex with id = @id, or None if said Vertex doesn't exist
    pub fn get_vertex_mut(&mut self, id: GraphID) -> Option<&mut Vertex<T>> {
        self.vertices.get_mut(&id)
    }

    /// Gets a reference to the Vertex with label = @label, or None if said Vertex doesn't exist
    pub fn find_vertex_by_label(&self, label: &str) -> Option<&Vertex<T>> {
        self.vertices.values().find(|v| v.label == label)
    }

    /// Adds a Vertex to this Graph. If @label = None, then the label will be set to the
    /// resulting Vertex's GraphID
    pub fn add_vertex(&mut self, data: T, label: Option<&str>) -> GraphID {
        let id = self.next_vid();
        self.vertices
            .insert(id, Vertex::new(data, id, label.unwrap_or(&id.to_string())));

        id
    }

    /// Adds an Edge from Vertex with id = @vid1 to Vertex with id = @vid2 with
    /// edge weight @weight
    ///
    /// Returns true if the edge was added, false otherwise
    pub fn add_edge(&mut self, vid1: GraphID, vid2: GraphID, weight: i64) -> bool {
        if self.get_vertex(vid1).is_none() || self.get_vertex(vid2).is_none() {
            return false;
        }

        if let Some(v1) = self.get_vertex_mut(vid1) {
            let edge = Edge::new(vid2, weight);
            v1.edges.push(edge);
            true
        } else {
            false
        }
    }

    /// Adds an Edge from Vertex with id = @vid1 to Vertex with id = @vid2 with
    /// edge weight @weight, and another Edge with the same weight in the opposite
    /// direction
    ///
    /// Returns true if the edges were added, false otherwise
    pub fn add_edge_bidirectional(&mut self, v1: GraphID, v2: GraphID, weight: i64) -> bool {
        let eid1 = self.add_edge(v1, v2, weight);

        if !eid1 {
            return false;
        }

        let eid2 = self.add_edge(v2, v1, weight);

        if !eid2 {
            self.get_vertex_mut(v1).unwrap().edges.pop();
            return false;
        }

        true
    }

    /// Removes the Vertex with id = @id from this Graph and all of
    /// its incoming edges, if it exists
    pub fn remove_vertex(&mut self, id: GraphID) {
        self.vertices.remove(&id);

        for v in self.vertices.values_mut() {
            v.remove_edges(id);
        }
    }

    /// Gets the number of vertices in this Graph
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// Whether this Graph has any vertices, equivalent to self.len() == 0
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Gets an iterator of references to the vertices in this Graph
    pub fn iter(&self) -> GraphIterator<'_, T> {
        GraphIterator::new(self)
    }

    /// Gets an iterator of mutable references to the vertices in this Graph
    pub fn iter_mut(&mut self) -> GraphIteratorMut<'_, T> {
        GraphIteratorMut::new(self)
    }

    /// Finds the weight of the shortest path from start to all other vertices in graph
    ///
    /// Returns a Vec where tuple.0 is the ID of a Vertex and tuple.1 is the weight
    /// of the shortest path
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
}

/* GRAPH TRAITS */

impl<T: Ord + Clone> Default for Graph<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Clone> Display for Graph<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for v in self.vertices.values() {
            writeln!(
                f,
                "{}: {:?}",
                v.label,
                v.edges
                    .iter()
                    .map(|e| self.get_vertex(e.to).unwrap().label.as_str())
                    .collect::<Vec<&str>>()
            )?;
        }

        Ok(())
    }
}

/* GRAPH ITERATOR TRAITS */

pub struct GraphIterator<'a, T: Ord + Clone> {
    vals: Values<'a, GraphID, Vertex<T>>,
}

pub struct GraphIteratorMut<'a, T: Ord + Clone> {
    vals: ValuesMut<'a, GraphID, Vertex<T>>,
}

impl<'a, T: Ord + Clone> GraphIterator<'a, T> {
    fn new(graph: &'a Graph<T>) -> Self {
        GraphIterator {
            vals: graph.vertices.values(),
        }
    }
}

impl<'a, T: Ord + Clone> GraphIteratorMut<'a, T> {
    fn new(graph: &'a mut Graph<T>) -> Self {
        GraphIteratorMut {
            vals: graph.vertices.values_mut(),
        }
    }
}

impl<'a, T: Ord + Clone> Iterator for GraphIterator<'a, T> {
    type Item = &'a Vertex<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.vals.next()
    }
}

impl<'a, T: Ord + Clone> Iterator for GraphIteratorMut<'a, T> {
    type Item = &'a mut Vertex<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.vals.next()
    }
}

/* DJIKSTRA STATE IMPLEMENTATION */

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
