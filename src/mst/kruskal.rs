use super::forest::{ForestBuilder, SpanningForest};
use crate::graph::{Edge, Graph};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Kruskal: edges are extracted in weight order from a binary heap.
pub struct Kruskal {
    num_vertices: usize,
    heap: BinaryHeap<Reverse<Edge>>,
}

impl Kruskal {
    /// Prepares the algorithm by building a min-heap of the edges of `graph`.
    pub fn new<T>(graph: &impl Graph<T>) -> Self {
        Kruskal {
            num_vertices: graph.num_vertices(),
            heap: graph.edges().map(Reverse).collect(),
        }
    }

    /// Computes the minimum spanning forest.
    pub fn run(mut self) -> SpanningForest {
        let mut forest = ForestBuilder::new(self.num_vertices);
        while !forest.is_complete() {
            let Some(Reverse(edge)) = self.heap.pop() else {
                break;
            };
            forest.try_add(edge);
        }
        forest.finish()
    }
}
