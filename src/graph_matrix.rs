use crate::constants::{Cost, VertexId};
use crate::error::GraphError;
use crate::graph::{Edge, Graph, Vertex};
use crate::MAX_COST;
use rand::distr::{Distribution, Uniform};
use rand::Rng;
use std::mem;

pub struct GraphMatrix<T> {
    vertices: Vec<Vertex<T>>,
    adj_matrix: Vec<Cost>,
    cached_edges: Vec<Edge>,
}

impl<T: Clone + Eq> Default for GraphMatrix<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Eq> GraphMatrix<T> {
    pub fn new() -> Self {
        GraphMatrix {
            vertices: Vec::new(),
            adj_matrix: Vec::new(),
            cached_edges: Vec::new(),
        }
    }

    pub fn new_from_collection<K: IntoIterator<Item = T>>(collection: K) -> Self {
        let mut this = GraphMatrix::new();
        for v in collection {
            this.add_vertex(v);
        }
        this
    }

    pub fn new_random<K, R>(
        collection: K,
        p: f64,
        min_cost: usize,
        max_cost: usize,
        no_self_loops: bool,
        rng: &mut R,
    ) -> Result<Self, GraphError>
    where
        K: IntoIterator<Item = T>,
        R: Rng,
    {
        if !(0.0..=1.0).contains(&p) {
            return Err(GraphError::InvalidProbability(p));
        }
        if min_cost > max_cost {
            return Err(GraphError::InvalidCostRange {
                min: min_cost,
                max: max_cost,
            });
        }

        let mut graph = GraphMatrix::new_from_collection(collection);
        let cost_dist = Uniform::new_inclusive(min_cost, max_cost).unwrap();

        let num_vertices = graph.num_vertices();

        for from_idx in 0..num_vertices {
            let start = if no_self_loops {
                from_idx + 1
            } else {
                from_idx
            };

            for to_idx in start..num_vertices {
                if rng.random::<f64>() < p {
                    let cost = cost_dist.sample(rng);
                    graph.add_edge(from_idx, to_idx, cost);
                }
            }
        }
        Ok(graph)
    }

    fn index(&self, mut row: VertexId, mut col: VertexId) -> VertexId {
        if row > col {
            mem::swap(&mut row, &mut col);
        }
        col * (col - 1) / 2 + row
    }

    pub fn adj_matrix(self) -> Vec<Cost> {
        self.adj_matrix.clone()
    }
}

impl<T: Clone + Eq> Graph<T> for GraphMatrix<T> {
    fn add_vertex(&mut self, data: T) -> usize {
        let last_row = self.vertices.len();
        self.vertices.push(Vertex { id: last_row, data });

        self.adj_matrix
            .resize(self.index(last_row + 1, last_row + 1), MAX_COST);
        last_row
    }

    fn add_edge(&mut self, from: VertexId, to: VertexId, cost: Cost) {
        let index = self.index(from, to);
        if self.adj_matrix[index] == MAX_COST {
            self.cached_edges.push(Edge::new(from, to, cost));
        }
        self.adj_matrix[index] = cost;
    }

    fn vertex(&self, id: VertexId) -> Option<&Vertex<T>> {
        self.vertices.get(id)
    }

    fn vertices(&self) -> &[Vertex<T>] {
        &self.vertices
    }

    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn all_edges(&self) -> Vec<Edge> {
        self.cached_edges.clone()
    }
}
