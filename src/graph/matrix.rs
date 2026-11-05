use super::{add_random_edges, Edge, Graph};
use crate::error::GraphError;
use crate::types::{Cost, VertexId};
use rand::Rng;
use std::ops::RangeInclusive;

/// An empty cell of the adjacency matrix.
const EMPTY: usize = usize::MAX;

/// A graph stored as lower triangular adjacency matrix.
///
/// Each cell holds the index of the edge in an edge list, edge lookups are `O(1)`.
#[derive(Clone, Debug)]
pub struct GraphMatrix<T> {
    vertices: Vec<T>,
    adj_matrix: Vec<usize>,
    edges: Vec<Edge>,
}

impl<T> Default for GraphMatrix<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> GraphMatrix<T> {
    /// Creates an empty graph.
    pub fn new() -> Self {
        GraphMatrix {
            vertices: Vec::new(),
            adj_matrix: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Creates a random `G(n, p)` graph from `vertices`, with each edge cost
    /// drawn uniformly from `costs`.
    pub fn new_random<I, R>(
        vertices: I,
        p: f64,
        costs: RangeInclusive<Cost>,
        rng: &mut R,
    ) -> Result<Self, GraphError>
    where
        I: IntoIterator<Item = T>,
        R: Rng + ?Sized,
    {
        let mut graph: Self = vertices.into_iter().collect();
        add_random_edges(&mut graph, p, costs, rng)?;
        Ok(graph)
    }

    /// Returns the cost of edge `{from, to}`.
    pub fn cost(&self, from: VertexId, to: VertexId) -> Option<Cost> {
        if from == to || from.max(to) >= self.vertices.len() {
            return None;
        }
        match self.adj_matrix[Self::index(from, to)] {
            EMPTY => None,
            edge => Some(self.edges[edge].weight),
        }
    }

    /// Position of cell `{row, col}` with `row != col`.
    fn index(row: VertexId, col: VertexId) -> usize {
        let (row, col) = if row < col { (row, col) } else { (col, row) };
        col * (col - 1) / 2 + row
    }

    fn check_vertex(&self, vertex: VertexId) -> Result<(), GraphError> {
        let num_vertices = self.vertices.len();
        if vertex < num_vertices {
            Ok(())
        } else {
            Err(GraphError::VertexOutOfBounds {
                vertex,
                num_vertices,
            })
        }
    }
}

impl<T> FromIterator<T> for GraphMatrix<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut graph = GraphMatrix::new();
        for data in iter {
            graph.add_vertex(data);
        }
        graph
    }
}

impl<T> Graph<T> for GraphMatrix<T> {
    fn add_vertex(&mut self, data: T) -> VertexId {
        let id = self.vertices.len();
        self.vertices.push(data);
        let n = self.vertices.len();
        self.adj_matrix.resize(n * (n - 1) / 2, EMPTY);
        id
    }

    fn add_edge(&mut self, from: VertexId, to: VertexId, cost: Cost) -> Result<(), GraphError> {
        self.check_vertex(from)?;
        self.check_vertex(to)?;
        if from == to {
            return Ok(());
        }
        let cell = &mut self.adj_matrix[Self::index(from, to)];
        if *cell == EMPTY {
            *cell = self.edges.len();
            self.edges.push(Edge::new(from.min(to), from.max(to), cost));
        } else {
            self.edges[*cell].weight = cost;
        }
        Ok(())
    }

    fn vertex(&self, id: VertexId) -> Option<&T> {
        self.vertices.get(id)
    }

    fn vertices(&self) -> &[T] {
        &self.vertices
    }

    fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.edges.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_is_a_bijection_on_the_lower_triangle() {
        let n = 20;
        let mut seen = vec![false; n * (n - 1) / 2];
        for col in 0..n {
            for row in 0..col {
                let i = GraphMatrix::<()>::index(row, col);
                assert_eq!(i, GraphMatrix::<()>::index(col, row));
                assert!(!seen[i], "cell {i} reused");
                seen[i] = true;
            }
        }
        assert!(seen.into_iter().all(|s| s));
    }

    #[test]
    fn add_edge_overwrites_cost_and_normalizes_endpoints() {
        let mut g: GraphMatrix<()> = std::iter::repeat_n((), 3).collect();
        g.add_edge(2, 0, 5).unwrap();
        g.add_edge(0, 2, 3).unwrap();
        assert_eq!(g.cost(0, 2), Some(3));
        assert_eq!(g.cost(2, 0), Some(3));
        assert_eq!(g.edges().collect::<Vec<_>>(), vec![Edge::new(0, 2, 3)]);
    }

    #[test]
    fn self_loops_and_missing_edges() {
        let mut g: GraphMatrix<()> = std::iter::repeat_n((), 2).collect();
        g.add_edge(1, 1, 7).unwrap();
        assert_eq!(g.edges().count(), 0);
        assert_eq!(g.cost(0, 1), None);
        assert_eq!(g.cost(1, 1), None);
        assert_eq!(g.cost(0, 9), None);
    }

    #[test]
    fn cost_max_is_valid_weight() {
        let mut g: GraphMatrix<()> = std::iter::repeat_n((), 2).collect();
        g.add_edge(0, 1, Cost::MAX).unwrap();
        assert_eq!(g.cost(0, 1), Some(Cost::MAX));
    }

    #[test]
    fn out_of_bounds_vertex_returns_an_error() {
        let mut g: GraphMatrix<()> = std::iter::repeat_n((), 2).collect();
        assert_eq!(
            g.add_edge(0, 2, 1),
            Err(GraphError::VertexOutOfBounds {
                vertex: 2,
                num_vertices: 2
            })
        );
        assert_eq!(g.edges().count(), 0);
    }
}
