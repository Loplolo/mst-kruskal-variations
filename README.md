# mst-kruskal-variants

Kruskal's minimum spanning tree algorithm and several of its variants, in Rust.

Developed for the **Graph Optimization** course of the Computer Science Master's degree at the University of Milan.

## Algorithms

| Type                   | Description                                                                                                                                                             |
|------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `Kruskal`              | Kruskal: edges are extracted in weight order from a binary heap.                                                                                                        |
| `QuickSortKruskal`     | Edges are sorted lazily by a randomized QuickSort and sorting stops as soon as the tree is complete.                                                                    |
| `FilterKruskal`        | QuickSort Kruskal that discards edges whose endpoints are already connected before partitioning each range (Osipov, Sanders, Singler 2009).                             |
| `SkewedFilterKruskal`  | Filter Kruskal using as pivot the lightest of `clamp(len / 100, 1, 5)` random samples (Righini, Righini 2022).                                                          |
| `StarQuickSortKruskal` | Star QuickSort Kruskal (SQSK): every adjacency list is sorted lazily by its own QuickSort and a heap selects the lightest edge across all lists. Requires `GraphStars`. |

Every algorithm returns a `SpanningForest`: a minimum spanning tree when the
graph is connected, otherwise one minimum spanning tree per connected component.

## Graph representations

| Type          | Storage                                                                          |
|---------------|----------------------------------------------------------------------------------|
| `GraphMatrix` | Flattened lower triangular adjacency matrix, `O(n²)` memory, `O(1)` edge lookup. |
| `GraphStars`  | Adjacency lists, `O(n + m)` memory.                            | 

Both implement the `Graph` trait and can generate random `G(n, p)` graphs.
Graphs are undirected, adding an existing edge replaces its cost and
self-loops are ignored.

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
mst-kruskal-variants = { git = "https://github.com/Loplolo/mst-kruskal-variation" }
```

Build a graph and compute its minimum spanning tree:

```rust
use mst_kruskal_variants::{Graph, GraphMatrix, Kruskal};

let mut graph: GraphMatrix<&str> = ["a", "b", "c"].into_iter().collect();
graph.add_edge(0, 1, 4)?;
graph.add_edge(1, 2, 2)?;
graph.add_edge(0, 2, 1)?;

let mst = Kruskal::new(&graph).run();
assert_eq!(mst.cost, 3);
assert!(mst.is_spanning_tree(graph.num_vertices()));
```

The randomized variants take any `rand` generator:

```rust
use mst_kruskal_variants::{FilterKruskal, GraphStars, StarQuickSortKruskal};
use rand::SeedableRng;
use rand::rngs::StdRng;

let mut rng = StdRng::seed_from_u64(42);
let graph = GraphStars::new_random(0..1_000, 0.01, 1..=1_000, &mut rng)?;

let filter = FilterKruskal::new(&graph).run(&mut rng);
let sqsk = StarQuickSortKruskal::new(&graph).run();
assert_eq!(filter.cost, sqsk.cost);
```

## Development

Requires Rust 1.85 or newer.

```bash
cargo test
cargo run --release --example random_graph
cargo bench
```

## License

Licensed under the [GNU General Public License v3.0 or later](LICENSE).
