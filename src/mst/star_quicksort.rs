use super::forest::{ForestBuilder, SpanningForest};
use crate::graph::Edge;
use crate::graph::GraphStars;
use crate::types::{Cost, VertexId};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Star QuickSort Kruskal: every star is sorted lazily by its own
/// incremental QuickSort and a heap holds the lightest unprocessed edge of
/// each star.
pub struct StarQuickSortKruskal {
    /// Lightest unprocessed edge of each star.
    heap: BinaryHeap<Reverse<(Cost, VertexId)>>,
    /// The ranges to be sorted per star with the lowest first.
    stacks: Vec<Vec<(usize, usize)>>,
    stars: Vec<Vec<Edge>>,
    /// The position of the next edge to process.
    next: Vec<usize>,
}

impl StarQuickSortKruskal {
    /// Prepares the algorithm by copying the stars of `graph` and sorting
    /// the first edge of each.
    pub fn new<T>(graph: &GraphStars<T>) -> Self {
        let stars = graph.stars().to_vec();
        let num_vertices = stars.len();
        let mut sqsk = StarQuickSortKruskal {
            heap: BinaryHeap::with_capacity(num_vertices),
            stacks: vec![Vec::new(); num_vertices],
            stars,
            next: vec![0; num_vertices],
        };
        for id in 0..num_vertices {
            if !sqsk.stars[id].is_empty() {
                sqsk.stacks[id].push((0, sqsk.stars[id].len() - 1));
                sqsk.push_next(id);
            }
        }
        sqsk
    }

    /// Places the next edge of star `id` in its sorted position and pushes it
    /// on the heap.
    fn push_next(&mut self, id: VertexId) {
        let target = self.next[id];
        let star = &mut self.stars[id];
        if target >= star.len() {
            return;
        }
        let stack = &mut self.stacks[id];
        if let Some((mut p, mut q)) = stack.pop() {
            while p < q {
                let pivot = p + (q - p) / 2;
                star.swap(pivot, q);
                let pivot_weight = star[q].weight;

                let mut i = p;
                for j in p..q {
                    if star[j].weight < pivot_weight {
                        star.swap(i, j);
                        i += 1;
                    }
                }
                star.swap(i, q);

                if i == target {
                    if i < q {
                        stack.push((i + 1, q));
                    }
                    break;
                } else if i < target {
                    p = i + 1;
                } else {
                    stack.push((i, q));
                    q = i - 1;
                }
            }
        }
        debug_assert!(
            star[target + 1..]
                .iter()
                .all(|e| e.weight >= star[target].weight),
            "star {id} is not sorted up to position {target}"
        );
        self.heap.push(Reverse((star[target].weight, id)));
    }

    /// Computes the minimum spanning forest.
    pub fn run(mut self) -> SpanningForest {
        let mut forest = ForestBuilder::new(self.stars.len());
        while !forest.is_complete() {
            let Some(Reverse((_, id))) = self.heap.pop() else {
                break;
            };
            forest.try_add(self.stars[id][self.next[id]]);
            self.next[id] += 1;
            self.push_next(id);
        }
        forest.finish()
    }
}
