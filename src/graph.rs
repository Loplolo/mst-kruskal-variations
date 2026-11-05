//! Graph representations.

mod matrix;
mod stars;

pub use matrix::GraphMatrix;
pub use stars::GraphStars;

use crate::error::GraphError;
use crate::types::{Cost, VertexId};
use rand::distr::{Distribution, Uniform};
use rand::{Rng, RngExt};
use std::cmp::Ordering;
use std::ops::RangeInclusive;

/// An undirected, weighted graph whose vertices carry data of type `T`.
pub trait Graph<T> {
    /// Adds a vertex and returns its id.
    fn add_vertex(&mut self, data: T) -> VertexId;

    /// Adds the undirected edge `{from, to}`.
    ///
    /// Adding an edge that already exists replaces its cost.
    /// Self-loops are ignored as they never belong to a spanning tree.
    fn add_edge(&mut self, from: VertexId, to: VertexId, cost: Cost) -> Result<(), GraphError>;

    /// Returns the data of vertex `id`.
    fn vertex(&self, id: VertexId) -> Option<&T>;

    /// Returns the data of all vertices.
    fn vertices(&self) -> &[T];

    /// Returns the number of vertices.
    fn num_vertices(&self) -> usize {
        self.vertices().len()
    }

    /// Returns every edge once, with `from < to`.
    fn edges(&self) -> impl Iterator<Item = Edge> + '_;
}

/// An undirected weighted edge.
///
/// Edges are ordered by weight
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    /// First endpoint.
    pub from: VertexId,
    /// Second endpoint.
    pub to: VertexId,
    /// Cost of the edge.
    pub weight: Cost,
}

impl Edge {
    /// Creates a new edge.
    pub fn new(from: VertexId, to: VertexId, weight: Cost) -> Edge {
        Edge { from, to, weight }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.weight, self.from, self.to).cmp(&(other.weight, other.from, other.to))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Adds every edge `{i, j}`, `i < j`, with probability `p` and a cost drawn
/// uniformly from `costs` (Erdős–Rényi graph).
pub(crate) fn add_random_edges<T, G, R>(
    graph: &mut G,
    p: f64,
    costs: RangeInclusive<Cost>,
    rng: &mut R,
) -> Result<(), GraphError>
where
    G: Graph<T>,
    R: Rng + ?Sized,
{
    if !(0.0..=1.0).contains(&p) {
        return Err(GraphError::InvalidProbability(p));
    }
    let cost_dist =
        Uniform::try_from(costs.clone()).map_err(|_| GraphError::EmptyCostRange(costs))?;

    let n = graph.num_vertices();
    for from in 0..n {
        for to in from + 1..n {
            if rng.random::<f64>() < p {
                graph.add_edge(from, to, cost_dist.sample(rng))?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Edge, Graph, GraphError, GraphMatrix, GraphStars};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn edge_ordering_is_consistent_with_equality() {
        let a = Edge::new(0, 1, 5);
        let b = Edge::new(2, 3, 5);
        assert_ne!(a, b);
        assert!(a < b);
        assert!(Edge::new(9, 9, 1) < a);
    }

    #[test]
    fn random_graph_rejects_invalid_parameters() {
        let mut rng = StdRng::seed_from_u64(0);
        for p in [-0.1, 1.1, f64::NAN] {
            assert!(matches!(
                GraphMatrix::new_random(0..3, p, 1..=2, &mut rng),
                Err(GraphError::InvalidProbability(_))
            ));
        }
        #[allow(clippy::reversed_empty_ranges)]
        let empty = 5..=1;
        assert_eq!(
            GraphStars::new_random(0..3, 0.5, empty.clone(), &mut rng).err(),
            Some(GraphError::EmptyCostRange(empty))
        );
    }

    #[test]
    fn random_graph_respects_extremes() {
        let mut rng = StdRng::seed_from_u64(0);
        let empty = GraphMatrix::new_random(0..10, 0.0, 1..=9, &mut rng).unwrap();
        assert_eq!(empty.edges().count(), 0);

        let complete = GraphStars::new_random(0..10, 1.0, 3..=7, &mut rng).unwrap();
        let edges: Vec<Edge> = complete.edges().collect();
        assert_eq!(edges.len(), 45);
        assert!(edges
            .iter()
            .all(|e| (3..=7).contains(&e.weight) && e.from < e.to));
    }
}
