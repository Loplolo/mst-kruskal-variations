// # Filter Kruskal
//
// Implementation of the Kruskal algorithm using a filtered quickselect approach.
use crate::constants::*;
use crate::graph::{Edge, Graph};
use crate::graph_matrix::GraphMatrix;
use crate::union_find::UnionFind;
use rand::Rng;

pub struct FilterKruskal {
    num_vertices: usize,
    num_edges: usize,
    edges: Vec<Edge>,
    union_find: UnionFind,
    mst_edges: Vec<Edge>,
    mst_cost: Cost,
}

impl FilterKruskal {
    pub fn new(graph: &GraphMatrix<usize>) -> Self {
        let num_vertices = graph.num_vertices();
        let edges = graph.all_edges();
        let num_edges = edges.len();
        FilterKruskal {
            num_vertices,
            num_edges,
            edges,
            union_find: UnionFind::new(num_vertices),
            mst_edges: Vec::new(),
            mst_cost: 0,
        }
    }

    pub fn run<R: Rng>(&mut self, rng: &mut R) -> (Vec<Edge>, Cost) {
        if self.num_edges == 0 {
            return (Vec::new(), 0);
        }

        let mut count = 0;
        let m: usize = self.num_edges;
        let mut mem: Vec<(usize, usize)> = Vec::new();

        mem.push((0, m - 1));
        while let Some((p, mut q)) = mem.pop() {
            if count >= self.num_vertices - 1 {
                break;
            }

            // Before partitioning, compress the range by removing edges
            // that are already connected in the UnionFind structure.
            let mut write_idx = p;
            for read_idx in p..=q {
                let e = self.edges[read_idx];
                if self.union_find.find(e.from) != self.union_find.find(e.to) {
                    self.edges[write_idx] = e;
                    write_idx += 1;
                }
            }
            if write_idx == p {
                continue;
            }
            q = write_idx - 1;

            if p == q {
                let edge = self.edges[p];
                if self.union_find.union(edge.from, edge.to) {
                    self.mst_edges.push(edge);
                    self.mst_cost += edge.weight;
                    count += 1;
                }
                continue;
            }

            let mut e_plus = q;
            if p < q {
                let pivot_idx = rng.random_range(p..=q);
                self.edges.swap(p, pivot_idx);

                let mut e_minus = p;
                while e_minus <= e_plus {
                    while self.edges[e_plus].weight > self.edges[p].weight {
                        if e_plus == 0 {
                            break;
                        }
                        e_plus -= 1;
                    }
                    while (e_minus <= e_plus)
                        && (self.edges[e_minus].weight <= self.edges[p].weight)
                    {
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
                    mem.push((e_plus + 1, q));
                }

                mem.push((e_plus, e_plus));

                if e_plus > p {
                    mem.push((p, e_plus - 1));
                }
            }
        }

        (self.mst_edges.clone(), self.mst_cost)
    }
}
