//! Kruskal's minimum spanning tree algorithm and some of its variants.
//!
//! Build a graph with [`GraphMatrix`] or [`GraphStars`], then run one of the
//! algorithms on it. Every algorithm returns a [`SpanningForest`], which is a
//! minimum spanning tree when the graph is connected.
//!
//! ```
//! use mst_kruskal_variants::{Graph, GraphMatrix, Kruskal};
//!
//! let mut graph: GraphMatrix<&str> = ["a", "b", "c"].into_iter().collect();
//! graph.add_edge(0, 1, 4)?;
//! graph.add_edge(1, 2, 2)?;
//! graph.add_edge(0, 2, 1)?;
//!
//! let mst = Kruskal::new(&graph).run();
//! assert_eq!(mst.cost, 3);
//! assert!(mst.is_spanning_tree(graph.num_vertices()));
//! # Ok::<(), mst_kruskal_variants::GraphError>(())
//! ```
#![warn(missing_docs)]

pub mod graph;
pub mod mst;

mod error;
mod types;
mod union_find;

pub use error::GraphError;
pub use graph::{Edge, Graph, GraphMatrix, GraphStars};
pub use mst::{
    FilterKruskal, Kruskal, QuickSortKruskal, SkewedFilterKruskal, SpanningForest,
    StarQuickSortKruskal,
};
pub use types::{Cost, VertexId};
