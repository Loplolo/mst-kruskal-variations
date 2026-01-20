use crate::graph::{Edge, Graph};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
// # Heap Kruskal
//
// Implementation of the Kruskal algorithm using an heap.
use crate::constants::*;
use crate::graph_matrix::GraphMatrix;
use crate::union_find::UnionFind;

pub struct Kruskal {
    num_vertices: usize,
    union_find: UnionFind,
    heap: BinaryHeap<Reverse<Edge>>,
    mst_edges: Vec<Edge>,
    mst_cost: Cost,
}

impl Kruskal {
    // Constructs the algorithm structures
    pub fn new(graph: &GraphMatrix<usize>) -> Self {
        let edges = graph.all_edges();
        let num_vertices = graph.num_vertices();
        let heap: BinaryHeap<Reverse<Edge>> = edges.into_iter().map(Reverse).collect();

        Kruskal {
            num_vertices,
            union_find: UnionFind::new(num_vertices),
            heap,
            mst_cost: 0,
            mst_edges: Vec::new(),
        }
    }
    // Runs the algorithm and returns a set of edges representing the minimum
    // spanning tree and its associated totale cost.
    pub fn run(&mut self) -> (Vec<Edge>, Cost) {
        while self.mst_edges.len() < self.num_vertices - 1 {
            if let Some(Reverse(edge)) = self.heap.pop() {
                if self.union_find.union(edge.from, edge.to) {
                    self.mst_edges.push(edge);
                    self.mst_cost += edge.weight;
                }
            } else {
                break;
            }
        }
        (self.mst_edges.clone(), self.mst_cost)
    }
}
