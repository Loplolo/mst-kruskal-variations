use super::forest::{ForestBuilder, SpanningForest};
use super::partition::partition;
use crate::graph::{Edge, Graph};
use rand::{Rng, RngExt};

/// Kruskal over edges sorted lazily by a randomized QuickSort.
///
/// Subset ranges are processed lightest first, sorting stops when the
/// forest is complete.
pub struct QuickSortKruskal {
    num_vertices: usize,
    edges: Vec<Edge>,
}

impl QuickSortKruskal {
    /// Prepares the algorithm by collecting the edges of `graph`.
    pub fn new<T>(graph: &impl Graph<T>) -> Self {
        QuickSortKruskal {
            num_vertices: graph.num_vertices(),
            edges: graph.edges(),
        }
    }

    /// Computes the minimum spanning forest with random pivots.
    pub fn run<R: Rng + ?Sized>(mut self, rng: &mut R) -> SpanningForest {
        let mut forest = ForestBuilder::new(self.num_vertices);
        let mut stack = vec![(0, self.edges.len())];

        while let Some((lo, hi)) = stack.pop() {
            if forest.is_complete() {
                break;
            }
            match hi - lo {
                0 => {}
                1 => forest.try_add(self.edges[lo]),
                _ => {
                    let pivot = rng.random_range(..hi - lo);
                    let k = lo + partition(&mut self.edges[lo..hi], pivot);
                    if k + 1 < hi {
                        stack.push((k + 1, hi));
                    }
                    stack.push((k, k + 1));
                    if lo < k {
                        stack.push((lo, k));
                    }
                }
            }
        }
        forest.finish()
    }
}
