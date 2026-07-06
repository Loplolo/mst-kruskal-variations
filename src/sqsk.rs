// # Stars QuickSort Kruskal (SQSK)
//
// Implementation of the QuickSort Kruskal algorithm for
// adjacency list graphs.
use crate::constants::Cost;
use crate::graph::{Edge, Graph};
use crate::graph_stars::GraphStars;
use crate::union_find::UnionFind;
use crate::VertexId;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct SqskHeapItem {
    cost: Cost,
    vertex_id: VertexId,
    edge_index: usize,
}
impl Ord for SqskHeapItem {
    // Lowest cost has highest priority
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.vertex_id.cmp(&other.vertex_id))
    }
}
impl PartialOrd for SqskHeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct StarQuickSortKruskal {
    union_find: UnionFind,
    heap: BinaryHeap<SqskHeapItem>,
    stacks: Vec<Vec<(usize, usize)>>,
    stars: Vec<Vec<Edge>>,
    last_sorted_pos: Vec<usize>,
    mst_edges: Vec<Edge>,
    mst_cost: Cost,
}

impl StarQuickSortKruskal {
    pub fn new<T: Clone + Eq>(graph: &GraphStars<T>) -> Self {
        let num_vertices = graph.num_vertices();

        let stars_as_vecs = graph.stars();

        let mut sqsk = StarQuickSortKruskal {
            union_find: UnionFind::new(num_vertices),
            heap: BinaryHeap::with_capacity(num_vertices),
            stacks: vec![Vec::new(); num_vertices],
            stars: stars_as_vecs,
            last_sorted_pos: vec![0; num_vertices],
            mst_edges: Vec::new(),
            mst_cost: 0,
        };

        for id in 0..num_vertices {
            if !sqsk.stars[id].is_empty() {

                let initial_interval = (0, sqsk.stars[id].len() - 1);
                sqsk.stacks[id].push(initial_interval);

                sqsk.qs_step(id);

                let cost = sqsk.stars[id][0].weight;

                sqsk.heap.push(SqskHeapItem {
                    cost,
                    vertex_id: id,
                    edge_index: 0,
                });
            }
        }
        sqsk
    }

    pub fn qs_step(&mut self, id: usize) {
        if self.last_sorted_pos[id] >= self.stars[id].len() {
            return;
        }

        if let Some((mut p, mut q)) = self.stacks[id].pop() {
            let target_index = self.last_sorted_pos[id];

            while p < q {
                let pivot = p + (q - p) / 2; // pivot in the middle

                self.stars[id].swap(pivot, q);

                let pivot_weight = self.stars[id][q].weight;

                let mut i = p;
                for j in p..q {
                    if self.stars[id][j].weight < pivot_weight {
                        self.stars[id].swap(i, j);
                        i += 1;
                    }
                }
                self.stars[id].swap(i, q); // Place pivot element in the end of the range.

                if i == target_index {
                    if i < q {
                        self.stacks[id].push((i + 1, q));
                    }
                    return;
                } else if i < target_index {
                    p = i + 1;
                } else {
                    self.stacks[id].push((i, q));
                    q = i - 1;
                }
            }
        }
    }

    pub fn run(&mut self) -> (Vec<Edge>, Cost) {
        let num_vertices = self.stars.len();
        if num_vertices == 0 {
            return (self.mst_edges.clone(), self.mst_cost);
        }
        let mut count = 0;

        while count < num_vertices - 1 {
            if let Some(heap_item) = self.heap.pop() {
                let i = heap_item.vertex_id; 

                if heap_item.edge_index != self.last_sorted_pos[i] {
                    continue;
                }

                let edge = self.stars[i][self.last_sorted_pos[i]];
                let j = edge.to;
                let w = edge.weight;

                if self.union_find.union(i, j) {
                    self.mst_edges.push(Edge::new(i, j, w));
                    self.mst_cost += w;
                    count += 1;
                }
                self.last_sorted_pos[i] += 1;

                if self.last_sorted_pos[i] < self.stars[i].len() {
                    self.qs_step(i);
                    let new_cost = self.stars[i][self.last_sorted_pos[i]].weight;

                    self.heap.push(SqskHeapItem {
                        cost: new_cost,
                        vertex_id: i,
                        edge_index: self.last_sorted_pos[i],
                    });
                }
            } else {
                // ff the heap is empty (and |T| < n-1) the graph is disconnected
                break;
            }
        }
        (self.mst_edges.clone(), self.mst_cost)
    }
}
