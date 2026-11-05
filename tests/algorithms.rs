//! Checks every algorithm against an independent Prim implementation.

use mst_kruskal_variants::{
    Cost, Edge, FilterKruskal, Graph, GraphMatrix, GraphStars, Kruskal, QuickSortKruskal,
    SkewedFilterKruskal, SpanningForest, StarQuickSortKruskal,
};
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use std::collections::HashSet;

/// O(n²) Prim, restarted from every unvisited vertex.
fn prim(n: usize, edges: &[Edge]) -> (Cost, usize) {
    let mut adj = vec![vec![None; n]; n];
    for e in edges {
        adj[e.from][e.to] = Some(e.weight);
        adj[e.to][e.from] = Some(e.weight);
    }
    let mut in_tree = vec![false; n];
    let mut dist: Vec<Option<Cost>> = vec![None; n];
    let (mut cost, mut count) = (0, 0);
    for root in 0..n {
        if in_tree[root] {
            continue;
        }
        dist[root] = Some(0);
        while let Some(u) = (0..n)
            .filter(|&v| !in_tree[v] && dist[v].is_some())
            .min_by_key(|&v| dist[v])
        {
            in_tree[u] = true;
            if u != root {
                cost += dist[u].unwrap();
                count += 1;
            }
            for v in 0..n {
                if let Some(w) = adj[u][v] {
                    if !in_tree[v] && dist[v].is_none_or(|d| w < d) {
                        dist[v] = Some(w);
                    }
                }
            }
        }
    }
    (cost, count)
}

/// Asserts that `forest` is a minimum spanning forest of the graph.
fn assert_minimum_forest(name: &str, n: usize, edges: &[Edge], forest: &SpanningForest) {
    let (expected_cost, expected_len) = prim(n, edges);
    assert_eq!(forest.cost, expected_cost, "{name}: wrong cost");
    assert_eq!(forest.edges.len(), expected_len, "{name}: wrong edge count");
    assert_eq!(
        forest.cost,
        forest.edges.iter().map(|e| e.weight).sum::<Cost>(),
        "{name}: cost does not match edges"
    );

    let graph_edges: HashSet<_> = edges
        .iter()
        .map(|e| (e.from.min(e.to), e.from.max(e.to), e.weight))
        .collect();
    let mut components: Vec<usize> = (0..n).collect();
    fn root(c: &mut [usize], mut v: usize) -> usize {
        while c[v] != v {
            v = c[v];
        }
        v
    }
    for e in &forest.edges {
        let key = (e.from.min(e.to), e.from.max(e.to), e.weight);
        assert!(
            graph_edges.contains(&key),
            "{name}: {e:?} is not in the graph"
        );
        let (a, b) = (root(&mut components, e.from), root(&mut components, e.to));
        assert_ne!(a, b, "{name}: {e:?} closes a cycle");
        components[a] = b;
    }
}

fn check_all(matrix: &GraphMatrix<usize>, stars: &GraphStars<usize>, seed: u64) {
    let n = matrix.num_vertices();
    let edges: Vec<Edge> = matrix.edges().collect();
    let mut sorted_matrix = edges.clone();
    let mut sorted_stars: Vec<Edge> = stars.edges().collect();
    sorted_matrix.sort();
    sorted_stars.sort();
    assert_eq!(sorted_matrix, sorted_stars, "graph representations differ");

    let mut rng = StdRng::seed_from_u64(seed);
    let results = [
        ("Kruskal (matrix)", Kruskal::new(matrix).run()),
        ("Kruskal (stars)", Kruskal::new(stars).run()),
        ("QuickSort", QuickSortKruskal::new(matrix).run(&mut rng)),
        ("Filter", FilterKruskal::new(matrix).run(&mut rng)),
        (
            "SkewedFilter",
            SkewedFilterKruskal::new(matrix).run(&mut rng),
        ),
        ("Filter (stars)", FilterKruskal::new(stars).run(&mut rng)),
        ("StarQS", StarQuickSortKruskal::new(stars).run()),
    ];
    for (name, forest) in &results {
        assert_minimum_forest(name, n, &edges, forest);
    }
}

#[test]
fn random_graphs_match_prim() {
    let mut rng = StdRng::seed_from_u64(0xC0FFEE);
    for seed in 0..300 {
        let n = rng.random_range(0..40);
        let p = rng.random::<f64>();
        let max_cost = [1, 3, 20, 1000][rng.random_range(..4usize)];
        let matrix =
            GraphMatrix::new_random(0..n, p, 1..=max_cost, &mut StdRng::seed_from_u64(seed))
                .unwrap();
        let stars = GraphStars::new_random(0..n, p, 1..=max_cost, &mut StdRng::seed_from_u64(seed))
            .unwrap();
        check_all(&matrix, &stars, seed);
    }
}

#[test]
fn larger_sparse_graph_matches_prim() {
    let matrix =
        GraphMatrix::new_random(0..300, 0.02, 0..=500, &mut StdRng::seed_from_u64(1)).unwrap();
    let stars =
        GraphStars::new_random(0..300, 0.02, 0..=500, &mut StdRng::seed_from_u64(1)).unwrap();
    check_all(&matrix, &stars, 1);
}

/// An edge list `(from, to, cost)`.
type EdgeList<'a> = &'a [(usize, usize, Cost)];

fn build(n: usize, edges: EdgeList) -> (GraphMatrix<usize>, GraphStars<usize>) {
    let mut matrix: GraphMatrix<usize> = (0..n).collect();
    let mut stars: GraphStars<usize> = (0..n).collect();
    for &(u, v, w) in edges {
        matrix.add_edge(u, v, w).unwrap();
        stars.add_edge(u, v, w).unwrap();
    }
    (matrix, stars)
}

#[test]
fn degenerate_graphs() {
    let cases: &[(usize, EdgeList)] = &[
        (0, &[]),
        (1, &[]),
        (5, &[]),
        (2, &[(0, 1, 0)]),
        (4, &[(0, 1, 2), (2, 3, 1)]),
        (4, &[(0, 1, 7), (1, 2, 7), (2, 3, 7), (3, 0, 7), (0, 2, 7)]),
        (3, &[(0, 1, 9), (0, 1, 1), (1, 2, 4), (0, 2, 2)]),
    ];
    for (seed, &(n, edges)) in cases.iter().enumerate() {
        let (matrix, stars) = build(n, edges);
        check_all(&matrix, &stars, seed as u64);
    }
}

#[test]
fn known_tree() {
    let edges = [
        (0, 1, 4),
        (0, 7, 8),
        (1, 2, 8),
        (1, 7, 11),
        (2, 3, 7),
        (2, 8, 2),
        (2, 5, 4),
        (3, 4, 9),
        (3, 5, 14),
        (4, 5, 10),
        (5, 6, 2),
        (6, 7, 1),
        (6, 8, 6),
        (7, 8, 7),
    ];
    let (matrix, stars) = build(9, &edges);
    let forest = Kruskal::new(&matrix).run();
    assert_eq!(forest.cost, 37);
    assert!(forest.is_spanning_tree(9));
    check_all(&matrix, &stars, 0);
}
