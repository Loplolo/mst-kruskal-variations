use super::forest::SpanningForest;
use super::partition::filter_kruskal;
use crate::graph::{Edge, Graph};
use rand::{Rng, RngExt};

/// Filter-Kruskal with a skewed pivot: the lightest of `r` random samples,
/// where `r = clamp(len / 100, 1, 5)` (Righini, Righini 2022).
pub struct SkewedFilterKruskal {
    num_vertices: usize,
    edges: Vec<Edge>,
}

impl SkewedFilterKruskal {
    /// Prepares the algorithm by collecting the edges of `graph`.
    pub fn new<T>(graph: &impl Graph<T>) -> Self {
        SkewedFilterKruskal {
            num_vertices: graph.num_vertices(),
            edges: graph.edges(),
        }
    }

    /// Computes the minimum spanning forest, drawing pivots from `rng`.
    pub fn run<R: Rng + ?Sized>(self, rng: &mut R) -> SpanningForest {
        filter_kruskal(self.edges, self.num_vertices, |range| {
            let samples = (range.len() / 100).clamp(1, 5);
            (0..samples)
                .map(|_| rng.random_range(..range.len()))
                .min_by_key(|&i| range[i].weight)
                .expect("at least one sample is drawn")
        })
    }
}
