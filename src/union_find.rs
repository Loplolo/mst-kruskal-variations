use crate::types::VertexId;

/// Disjoint-set forest with union by size and path compression.
pub(crate) struct UnionFind {
    parent: Vec<VertexId>,
    size: Vec<usize>,
}

impl UnionFind {
    pub(crate) fn new(num: usize) -> Self {
        UnionFind {
            parent: (0..num).collect(),
            size: vec![1; num],
        }
    }

    pub(crate) fn find(&mut self, i: VertexId) -> VertexId {
        let mut root = i;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        let mut curr = i;
        while curr != root {
            let next = self.parent[curr];
            self.parent[curr] = root;
            curr = next;
        }
        root
    }

    /// Merges the sets of `i` and `j` and returns `false` if they are already joined.
    pub(crate) fn union(&mut self, i: VertexId, j: VertexId) -> bool {
        let (mut big, mut small) = (self.find(i), self.find(j));
        if big == small {
            return false;
        }
        if self.size[big] < self.size[small] {
            std::mem::swap(&mut big, &mut small);
        }
        self.parent[small] = big;
        self.size[big] += self.size[small];
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_joins_sets_once() {
        let mut uf = UnionFind::new(4);
        assert!(uf.union(0, 1));
        assert!(uf.union(2, 3));
        assert!(!uf.union(1, 0));
        assert_ne!(uf.find(0), uf.find(2));
        assert!(uf.union(1, 3));
        assert_eq!(uf.find(0), uf.find(2));
        assert!(!uf.union(0, 3));
    }
}
