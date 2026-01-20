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
    // Track the index of the edge for lazy deletion
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

// Structures to apply the SQSK algorithm on a generic graph.
pub struct StarQuickSortKruskal {
    union_find: UnionFind,
    heap: BinaryHeap<SqskHeapItem>,
    stacks: Vec<Vec<(usize, usize)>>, // (start, end) indices
    stars: Vec<Vec<Edge>>,
    last_sorted_pos: Vec<usize>,
    mst_edges: Vec<Edge>,
    mst_cost: Cost,
}

impl StarQuickSortKruskal {
    // Constructs the algorithm's structures and initializes it.
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
                // Insert the initial interval in the stack
                let initial_interval = (0, sqsk.stars[id].len() - 1);
                sqsk.stacks[id].push(initial_interval);

                // First quickselect step
                sqsk.qs_step(id);

                // Add to the heap the best candidate for each node
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

    // Executes a single step of the quickselect algorithm
    //
    pub fn qs_step(&mut self, id: usize) {
        if self.last_sorted_pos[id] >= self.stars[id].len() {
            return;
        }

        if let Some((mut p, mut q)) = self.stacks[id].pop() {
            let target_index = self.last_sorted_pos[id];

            // Quickselect
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
                    // We found the element for the current target position.
                    // We only need to push the right partition for future steps.
                    // The left partition (p..i-1) contains elements < target, which are already done.
                    if i < q {
                        self.stacks[id].push((i + 1, q));
                    }
                    return;
                } else if i < target_index {
                    // Target is in the right partition.
                    // Left partition elements are smaller than what we need now, discard them.
                    p = i + 1;
                } else {
                    // i > target_index
                    // Target is in the left partition.

                    // We must push the right partition to the stack for later including the pivot i
                    self.stacks[id].push((i, q));

                    q = i - 1;
                }
            }
        }
    }

    // Runs the algorithm and returns a set of edges representing the minimum
    // spanning tree and its associated total cost.
    pub fn run(&mut self) -> (Vec<Edge>, Cost) {
        let num_vertices = self.stars.len();
        if num_vertices == 0 {
            return (self.mst_edges.clone(), self.mst_cost);
        }
        let mut count = 0;

        // Loop until there are n-1 nodes in the minimum spanning tree
        while count < num_vertices - 1 {
            // Get the best candidate from the heap
            if let Some(heap_item) = self.heap.pop() {
                let i = heap_item.vertex_id; // Get the node id from the heap item

                // Verify validity (lazy insertion)
                if heap_item.edge_index != self.last_sorted_pos[i] {
                    continue;
                }

                let edge = self.stars[i][self.last_sorted_pos[i]];
                let j = edge.to;
                let w = edge.weight;

                // Union between the two MST with representative i and j
                if self.union_find.union(i, j) {
                    self.mst_edges.push(Edge::new(i, j, w));
                    self.mst_cost += w;
                    count += 1;
                }
                // Next candidate
                self.last_sorted_pos[i] += 1;

                // If there are more arcs not yet ordered wake up
                // quickselect again.
                if self.last_sorted_pos[i] < self.stars[i].len() {
                    self.qs_step(i);
                    let new_cost = self.stars[i][self.last_sorted_pos[i]].weight;

                    // Push the new candidate without removing the old one (lazy insertion).
                    self.heap.push(SqskHeapItem {
                        cost: new_cost,
                        vertex_id: i,
                        edge_index: self.last_sorted_pos[i],
                    });
                }
            } else {
                // If the heap is empty we got the solution (graph is disconnected).
                break;
            }
        }
        (self.mst_edges.clone(), self.mst_cost)
    }
}
