//! Minimum spanning tree algorithms.
//!
//! Every algorithm is split into `new`, which copies what it needs from the
//! graph, and `run`, which consumes it and returns a [`SpanningForest`].

mod filter;
mod forest;
mod kruskal;
mod partition;
mod quicksort;
mod skewed_filter;
mod star_quicksort;

pub use filter::FilterKruskal;
pub use forest::SpanningForest;
pub use kruskal::Kruskal;
pub use quicksort::QuickSortKruskal;
pub use skewed_filter::SkewedFilterKruskal;
pub use star_quicksort::StarQuickSortKruskal;
