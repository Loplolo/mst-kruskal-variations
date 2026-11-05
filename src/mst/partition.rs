use super::forest::{ForestBuilder, SpanningForest};
use crate::graph::Edge;

/// Hoare partition of `edges` around `edges[pivot]`.
///
/// Returns the final position `k` of the pivot: afterwards every edge in
/// `edges[..k]` weighs at most the pivot and every edge in `edges[k + 1..]`
/// weighs more.
pub(crate) fn partition(edges: &mut [Edge], pivot: usize) -> usize {
    debug_assert!(pivot < edges.len(), "pivot {pivot} out of range");
    edges.swap(0, pivot);
    let weight = edges[0].weight;
    let (mut i, mut j) = (1, edges.len() - 1);
    loop {
        while i <= j && edges[i].weight <= weight {
            i += 1;
        }
        while edges[j].weight > weight {
            j -= 1;
        }
        if i >= j {
            break;
        }
        edges.swap(i, j);
    }
    edges.swap(0, j);
    debug_assert!(edges[..j].iter().all(|e| e.weight <= weight));
    debug_assert!(edges[j + 1..].iter().all(|e| e.weight > weight));
    j
}

/// Filter-Kruskal driven by an incremental QuickSort over `edges`.
///
/// Subset ranges are processed lightest first, so edges reach the forest in
/// non-decreasing weight order and sorting ends when the forest is
/// complete. Before each range is partitioned, edges whose endpoints are
/// already connected are dropped from it.
pub(crate) fn filter_kruskal(
    mut edges: Vec<Edge>,
    num_vertices: usize,
    mut choose_pivot: impl FnMut(&[Edge]) -> usize,
) -> SpanningForest {
    let mut forest = ForestBuilder::new(num_vertices);
    let mut stack = vec![(0, edges.len())];

    while let Some((lo, mut hi)) = stack.pop() {
        if forest.is_complete() {
            break;
        }
        let mut kept = lo;
        for read in lo..hi {
            if forest.connects(&edges[read]) {
                edges[kept] = edges[read];
                kept += 1;
            }
        }
        hi = kept;
        match hi - lo {
            0 => {}
            1 => forest.try_add(edges[lo]),
            _ => {
                let range = &mut edges[lo..hi];
                let pivot = choose_pivot(range);
                debug_assert!(pivot < range.len(), "pivot {pivot} out of range");
                let k = lo + partition(range, pivot);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn edges(weights: &[usize]) -> Vec<Edge> {
        weights
            .iter()
            .enumerate()
            .map(|(i, &w)| Edge::new(i, i + 1, w))
            .collect()
    }

    #[test]
    fn partition_splits_pivot() {
        let weights = [5, 3, 8, 1, 5, 9, 2, 5];
        for pivot in 0..weights.len() {
            let mut es = edges(&weights);
            let pivot_weight = es[pivot].weight;
            let k = partition(&mut es, pivot);
            assert_eq!(es[k].weight, pivot_weight);
            assert!(es[..k].iter().all(|e| e.weight <= pivot_weight));
            assert!(es[k + 1..].iter().all(|e| e.weight > pivot_weight));
        }
    }

    #[test]
    fn partition_handles_single_and_equal_elements() {
        let mut one = edges(&[7]);
        assert_eq!(partition(&mut one, 0), 0);

        let mut equal = edges(&[4, 4, 4, 4]);
        assert_eq!(partition(&mut equal, 2), 3);
    }

    #[test]
    fn filter_kruskal_handles_empty_edge_list() {
        let forest = filter_kruskal(Vec::new(), 3, |_| 0);
        assert_eq!(forest, SpanningForest::default());
    }

    #[test]
    fn filter_kruskal_skips_cycle_edges() {
        let edges = vec![
            Edge::new(0, 1, 1),
            Edge::new(1, 2, 1),
            Edge::new(0, 2, 1),
            Edge::new(2, 3, 5),
        ];
        let forest = filter_kruskal(edges, 4, |range| range.len() / 2);
        assert_eq!(forest.edges.len(), 3);
        assert_eq!(forest.cost, 7);
    }
}
