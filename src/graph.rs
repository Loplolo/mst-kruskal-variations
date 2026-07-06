use crate::constants::{Cost, VertexId};
use std::cmp::Ordering;


pub trait Graph<T> {

    fn add_vertex(&mut self, data: T) -> VertexId;
    fn add_edge(&mut self, from: VertexId, to: VertexId, cost: Cost);
    fn vertex(&self, id: VertexId) -> Option<&Vertex<T>>;
    fn vertices(&self) -> &[Vertex<T>];
    fn num_vertices(&self) -> usize;
    fn all_edges(&self) -> Vec<Edge>;
}


#[derive(Clone, Debug)]
pub struct Vertex<T> {
    pub id: usize,
    pub data: T,
}


#[derive(Copy, Clone, Debug)]
pub struct Edge {
    pub from: VertexId,
    pub to: VertexId,
    pub weight: Cost,
}

impl Edge {

    pub fn new(from: VertexId, to: VertexId, weight: Cost) -> Edge {
        Edge { from, to, weight }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for Edge {}
