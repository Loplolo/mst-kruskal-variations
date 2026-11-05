use super::forest::SpanningForest;
use super::partition::filter_kruskal;
use crate::graph::{Edge, Graph};
use rand::{Rng, RngExt};

/// Filter-Kruskal: QuickSort Kruskal that discards edges whose endpoints are
/// already connected before partitioning each range.
pub struct FilterKruskal {
    num_vertices: usize,
    edges: Vec<Edge>,
}

impl FilterKruskal {
    /// Initialize the algorithm by collecting the edges of `graph`.
    pub fn new<T>(graph: &impl Graph<T>) -> Self {
        FilterKruskal {
            num_vertices: graph.num_vertices(),
            edges: graph.edges(),
        }
    }

    /// Computes the minimum spanning forest with random pivots.
    pub fn run<R: Rng + ?Sized>(self, rng: &mut R) -> SpanningForest {
        filter_kruskal(self.edges, self.num_vertices, |range| {
            rng.random_range(..range.len())
        })
    }
}
