use super::{add_random_edges, Edge, Graph};
use crate::error::GraphError;
use crate::types::{Cost, VertexId};
use rand::Rng;
use std::ops::RangeInclusive;

/// A graph stored as forward stars.
///
/// Every undirected edge `{u, v}`  as `u -> v` in the star of
/// `u` and as `v -> u` in the star of `v`.
#[derive(Clone, Debug)]
pub struct GraphStars<T> {
    vertices: Vec<T>,
    stars: Vec<Vec<Edge>>,
}

impl<T> Default for GraphStars<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> GraphStars<T> {
    /// Creates an empty graph.
    pub fn new() -> Self {
        GraphStars {
            vertices: Vec::new(),
            stars: Vec::new(),
        }
    }

    /// Creates a random `G(n, p)` graph with `vertices`, with each edge cost
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

    /// Returns the star outgoing edges of every vertex, indexed by id.
    pub fn stars(&self) -> &[Vec<Edge>] {
        &self.stars
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

    /// Inserts `from -> to` in the star of `from` or updates its cost.
    fn insert_arc(&mut self, from: VertexId, to: VertexId, cost: Cost) {
        let star = &mut self.stars[from];
        match star.iter_mut().find(|e| e.to == to) {
            Some(edge) => edge.weight = cost,
            None => star.push(Edge::new(from, to, cost)),
        }
    }
}

impl<T> FromIterator<T> for GraphStars<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut graph = GraphStars::new();
        for data in iter {
            graph.add_vertex(data);
        }
        graph
    }
}

impl<T> Graph<T> for GraphStars<T> {
    fn add_vertex(&mut self, data: T) -> VertexId {
        let id = self.vertices.len();
        self.vertices.push(data);
        self.stars.push(Vec::new());
        id
    }

    fn add_edge(&mut self, from: VertexId, to: VertexId, cost: Cost) -> Result<(), GraphError> {
        self.check_vertex(from)?;
        self.check_vertex(to)?;
        if from != to {
            self.insert_arc(from, to, cost);
            self.insert_arc(to, from, cost);
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
        self.stars
            .iter()
            .flatten()
            .filter(|e| e.from < e.to)
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edges_are_stored_in_both_stars() {
        let mut g: GraphStars<()> = std::iter::repeat_n((), 3).collect();
        g.add_edge(2, 0, 5).unwrap();
        assert_eq!(g.stars()[0], vec![Edge::new(0, 2, 5)]);
        assert_eq!(g.stars()[2], vec![Edge::new(2, 0, 5)]);
        assert_eq!(g.edges().collect::<Vec<_>>(), vec![Edge::new(0, 2, 5)]);
    }

    #[test]
    fn add_edge_overwrites_cost() {
        let mut g: GraphStars<()> = std::iter::repeat_n((), 2).collect();
        g.add_edge(0, 1, 5).unwrap();
        g.add_edge(1, 0, 3).unwrap();
        assert_eq!(g.stars()[0], vec![Edge::new(0, 1, 3)]);
        assert_eq!(g.stars()[1], vec![Edge::new(1, 0, 3)]);
    }

    #[test]
    fn self_loops_are_ignored_and_bounds_are_checked() {
        let mut g: GraphStars<()> = std::iter::repeat_n((), 2).collect();
        g.add_edge(1, 1, 7).unwrap();
        assert_eq!(g.edges().count(), 0);
        assert!(matches!(
            g.add_edge(5, 0, 1),
            Err(GraphError::VertexOutOfBounds { vertex: 5, .. })
        ));
        assert!(g.stars().iter().all(Vec::is_empty));
    }
}
