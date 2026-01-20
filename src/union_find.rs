// # Union Find
//
// Union find structure implementation for the use with Kruskal algorithms.

use crate::constants::UnionFindRep;

// A structure to memorize the union find forest.
pub struct UnionFind {
    rep: Vec<UnionFindRep>,
    size: Vec<usize>,
}

impl UnionFind {
    // Constructs the union find from the maximum number or of elements
    // # Note: In the beginning every tree is a single node.
    // # Todo: Add a `new_from_collection()` constructor.
    pub fn new(num: usize) -> Self {
        UnionFind {
            rep: (0..num).collect(),
            size: vec![1; num],
        }
    }

    // Returns the representative (root) of a given node
    // # Note: Takes O(n) to visit all nodes until the root is reached.
    pub fn find(&mut self, i: UnionFindRep) -> usize {
        let mut root = i;
        // Find root
        while self.rep[root] != root {
            root = self.rep[root];
        }
        // Compress path
        let mut curr = i;
        while curr != root {
            let next = self.rep[curr];
            self.rep[curr] = root;
            curr = next;
        }
        root
    }

    // Merges the two union-find's tree with a policy that
    // unites the representative of the larger tree with the smaller
    // one and returns a boolean flag to signify success.
    // # Note: if i and j representative were already searched
    // #       their find is O(1)
    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.rep[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.rep[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            true
        } else {
            false
        }
    }
}
