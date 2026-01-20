// # Graph
//
// Data structures adjacency list graph representations.
use crate::constants::{Cost, EdgeId, VertexId};
use crate::error::GraphError;
use crate::graph::{Edge, Graph, Vertex};
use rand::distr::{Distribution, Uniform};
use rand::Rng;

// Graph representation using nodes' outgoing stars.
pub struct GraphStars<T> {
    vertices: Vec<Vertex<T>>,
    stars: Vec<Vec<Edge>>,
}

impl<T: Clone + Eq> Default for GraphStars<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Eq> GraphStars<T> {
    pub fn new() -> Self {
        GraphStars {
            vertices: Vec::new(),
            stars: Vec::new(),
        }
    }

    // Constructs a graph without vertices from a generic collection's iterator
    // # Example: let g = Graph::new_from_collection(vec![1,2,3,4]);
    // # Note: O(n) but allows generic structures to be converted easily.
    pub fn new_from_collection<K: IntoIterator<Item = T>>(collection: K) -> Self {
        let mut this = GraphStars::new();
        for v in collection {
            this.add_vertex(v);
        }
        this
    }

    // Constructs a random graph using Erdős–Rényi model G(n, p) with uniform
    // random costs and a generic collection.
    // Input:
    // - p probability of selecting an arc
    // - a minimum cost value
    // - b maximum cost value
    //
    // # Example: let g = Graph::new_random(vec![1, 2, 3], 0.5, 0, 100 );
    //
    // # Note: O(n^2) complexity.
    //
    // # Panic: Probabilities must be expressed with a f32 between 0.0 and 1.0,
    // #        edge's cost range must be valid (a <= b).
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

        let mut graph = GraphStars::new_from_collection(collection);
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

    pub fn stars(&self) -> Vec<Vec<Edge>> {
        self.stars.clone()
    }
}

impl<T: Clone + Eq> Graph<T> for GraphStars<T> {
    // Adds a node to the structure and creates a new adjacency list.
    fn add_vertex(&mut self, data: T) -> usize {
        let id = self.vertices.len();
        self.vertices.push(Vertex { id, data });
        self.stars.push(Vec::new());
        id
    }

    // Adds a weighted edge between two nodes adding each node to the
    // other's adjacency list.
    fn add_edge(&mut self, from: VertexId, to: EdgeId, cost: Cost) {
        if from == to {
            return;
        }

        let exists = self.stars[from].iter().any(|e| e.to == to);

        if !exists {
            let edge_fwd = Edge::new(from, to, cost);
            let edge_bwd = Edge::new(to, from, cost);
            self.stars[from].push(edge_fwd);
            self.stars[to].push(edge_bwd);
        }
    }

    // Returns a vertex structure from a vertex identifier.
    fn vertex(&self, id: VertexId) -> Option<&Vertex<T>> {
        self.vertices.get(id)
    }

    // Returns the contained vertices.
    fn vertices(&self) -> &[Vertex<T>] {
        &self.vertices
    }

    // Returns the total number of vertices
    // # Note: It's O(1) since Vec uses an internal counter.
    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    // Returns a vector of all edges
    fn all_edges(&self) -> Vec<Edge> {
        let mut edges = Vec::new();
        for (from_id, star) in self.stars.iter().enumerate() {
            for edge in star {
                // Avoid duplicates
                if from_id < edge.to {
                    edges.push(*edge);
                }
            }
        }
        edges
    }
}
