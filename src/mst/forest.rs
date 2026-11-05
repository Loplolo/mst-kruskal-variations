use crate::graph::Edge;
use crate::types::Cost;
use crate::union_find::UnionFind;

/// A minimum spanning forest is a minimum spanning tree when the graph is
/// connected, otherwise one minimum spanning tree per connected component.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SpanningForest {
    /// The edges of the forest, in the order they were selected.
    pub edges: Vec<Edge>,
    /// The total cost of the forest.
    pub cost: Cost,
}

impl SpanningForest {
    /// Returns `true` if the forest is a spanning tree of a graph with
    /// `num_vertices` vertices.
    pub fn is_spanning_tree(&self, num_vertices: usize) -> bool {
        self.edges.len() + 1 == num_vertices.max(1)
    }
}

/// Accumulates in a forest, rejecting edges that would close a cycle.
pub(crate) struct ForestBuilder {
    union_find: UnionFind,
    forest: SpanningForest,
    target: usize,
}

impl ForestBuilder {
    pub(crate) fn new(num_vertices: usize) -> Self {
        ForestBuilder {
            union_find: UnionFind::new(num_vertices),
            forest: SpanningForest::default(),
            target: num_vertices.saturating_sub(1),
        }
    }

    /// Adds `edge` unless its endpoints are already connected.
    ///
    /// Edges must be offered in non-decreasing weight order.
    ///
    /// # Panics
    ///
    /// Panics if the total cost overflows [`Cost`].
    pub(crate) fn try_add(&mut self, edge: Edge) {
        debug_assert!(
            self.forest
                .edges
                .last()
                .is_none_or(|last| last.weight <= edge.weight),
            "edges must be added in non-decreasing weight order"
        );
        if self.union_find.union(edge.from, edge.to) {
            self.forest.edges.push(edge);
            self.forest.cost = self
                .forest
                .cost
                .checked_add(edge.weight)
                .expect("total cost of the spanning forest overflows `Cost`");
        }
    }

    /// Returns `true` if `edge` connects two different components.
    pub(crate) fn connects(&mut self, edge: &Edge) -> bool {
        self.union_find.find(edge.from) != self.union_find.find(edge.to)
    }

    /// Returns `true` once the forest is a spanning tree.
    pub(crate) fn is_complete(&self) -> bool {
        self.forest.edges.len() >= self.target
    }

    pub(crate) fn finish(self) -> SpanningForest {
        self.forest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_rejects_cycles() {
        let mut builder = ForestBuilder::new(3);
        builder.try_add(Edge::new(0, 1, 1));
        assert!(!builder.is_complete());
        builder.try_add(Edge::new(1, 0, 2));
        builder.try_add(Edge::new(1, 2, 3));
        assert!(builder.is_complete());
        let forest = builder.finish();
        assert_eq!(forest.edges, vec![Edge::new(0, 1, 1), Edge::new(1, 2, 3)]);
        assert_eq!(forest.cost, 4);
        assert!(forest.is_spanning_tree(3));
        assert!(!forest.is_spanning_tree(4));
    }

    #[test]
    fn empty_and_single_vertex_graphs_are_trees() {
        assert!(ForestBuilder::new(0).is_complete());
        assert!(SpanningForest::default().is_spanning_tree(0));
        assert!(SpanningForest::default().is_spanning_tree(1));
    }

    #[test]
    #[should_panic(expected = "overflows")]
    fn cost_overflow_panics() {
        let mut builder = ForestBuilder::new(3);
        builder.try_add(Edge::new(0, 1, Cost::MAX));
        builder.try_add(Edge::new(1, 2, Cost::MAX));
    }
}
