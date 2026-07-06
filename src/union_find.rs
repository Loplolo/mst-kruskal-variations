use crate::constants::UnionFindRep;

pub struct UnionFind {
    rep: Vec<UnionFindRep>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(num: usize) -> Self {
        UnionFind {
            rep: (0..num).collect(),
            size: vec![1; num],
        }
    }

    pub fn find(&mut self, i: UnionFindRep) -> usize {
        let mut root = i;
        while self.rep[root] != root {
            root = self.rep[root];
        }
        let mut curr = i;
        while curr != root {
            let next = self.rep[curr];
            self.rep[curr] = root;
            curr = next;
        }
        root
    }

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
