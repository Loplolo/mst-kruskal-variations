// # Skewed Filter Kruskal (Quick Sort Kruskal)
//
use crate::constants::*;
use crate::graph::{Edge, Graph};
use crate::graph_matrix::GraphMatrix;
use crate::union_find::UnionFind;
use rand::Rng;

pub struct SkewedFilterKruskal {
    num_vertices: usize,
    num_edges: usize,
    edges: Vec<Edge>,
    union_find: UnionFind,
    mst_edges: Vec<Edge>,
    mst_cost: Cost,
}

impl SkewedFilterKruskal {
    // Constructs the algorithm structures
    pub fn new(graph: &GraphMatrix<usize>) -> Self {
        let num_vertices = graph.num_vertices();
        let edges = graph.all_edges();
        let num_edges = edges.len();

        SkewedFilterKruskal {
            num_vertices,
            num_edges,
            edges,
            union_find: UnionFind::new(num_vertices),
            mst_edges: Vec::new(),
            mst_cost: 0,
        }
    }

    // Runs the algorithm and returns a set of edges representing the minimum
    // spanning tree and its associated total cost.
    //
    pub fn run<R: Rng>(&mut self, rng: &mut R) -> (Vec<Edge>, Cost) {
        if self.num_edges == 0 {
            return (Vec::new(), 0);
        }

        let mut count = 0;
        let mut stack: Vec<(usize, usize)> = Vec::new();

        stack.push((0, self.num_edges - 1));

        while let Some((p, q)) = stack.pop() {
            if count >= self.num_vertices - 1 {
                break;
            }

            if p == q {
                let edge = self.edges[p];
                if self.union_find.union(edge.from, edge.to) {
                    self.mst_edges.push(edge);
                    self.mst_cost += edge.weight;
                    count += 1;
                }
                continue;
            }

            if p < q {
                let len = q - p + 1;
                // max (1,  min (len / 100, 5))
                let r = (len / 100).clamp(1, 5);

                let mut best_pivot_idx = p;

                for _ in 0..r {
                    let candidate = rng.random_range(p..=q);
                    if self.edges[candidate].weight < self.edges[best_pivot_idx].weight {
                        best_pivot_idx = candidate;
                    }
                }

                self.edges.swap(p, best_pivot_idx);
            }

            let mut e_plus = q;
            let mut e_minus = p;

            let pivot_weight = self.edges[p].weight;

            while e_minus <= e_plus {
                while e_plus >= p && self.edges[e_plus].weight > pivot_weight {
                    if e_plus == 0 {
                        break;
                    }
                    e_plus -= 1;
                }

                while e_minus <= e_plus && self.edges[e_minus].weight <= pivot_weight {
                    e_minus += 1;
                }

                if e_minus < e_plus {
                    self.edges.swap(e_minus, e_plus);
                    e_minus += 1;
                    e_plus = e_plus.saturating_sub(1);
                }
            }

            self.edges.swap(p, e_plus);

            if (count < self.num_vertices - 1) && (e_plus < q) {
                stack.push((e_plus + 1, q));
            }

            stack.push((e_plus, e_plus));

            if e_plus > p {
                stack.push((p, e_plus - 1));
            }
        }

        (self.mst_edges.clone(), self.mst_cost)
    }
}
