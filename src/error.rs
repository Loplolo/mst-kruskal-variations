use crate::types::{Cost, VertexId};
use std::fmt;
use std::ops::RangeInclusive;

/// Errors returned when building a graph.
#[derive(Debug, Clone, PartialEq)]
pub enum GraphError {
    /// The edge probability is not in `0.0..=1.0`.
    InvalidProbability(f64),
    /// The cost range contains no values.
    EmptyCostRange(RangeInclusive<Cost>),
    /// A vertex id does not belong to the graph.
    VertexOutOfBounds {
        /// The offending vertex id.
        vertex: VertexId,
        /// The number of vertices in the graph.
        num_vertices: usize,
    },
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphError::InvalidProbability(p) => {
                write!(f, "probability must be between 0.0 and 1.0, got {p}")
            }
            GraphError::EmptyCostRange(range) => {
                write!(f, "cost range {}..={} is empty", range.start(), range.end())
            }
            GraphError::VertexOutOfBounds {
                vertex,
                num_vertices,
            } => write!(
                f,
                "vertex {vertex} is out of bounds for a graph with {num_vertices} vertices"
            ),
        }
    }
}

impl std::error::Error for GraphError {}
