use super::Graph;
use crate::algebra::Group;
use crate::data_structure::{MergingUnionFind, UnionFind};

#[cargo_snippet::snippet("minimum_spanning_tree")]
impl Graph {
    pub fn minimum_spanning_tree<T: Ord, F: Fn(&usize) -> T>(&self, weight: F) -> Vec<bool> {
        let mut idx: Vec<_> = (0..self.esize).collect();
        idx.sort_by_key(weight);
        let mut uf = UnionFind::new(self.vsize);
        let cache = self.eid_cache();
        let mut res = vec![false; self.esize];
        for eid in idx.into_iter() {
            let (u, v) = cache[eid];
            res[eid] = uf.unite(u, v);
        }
        res
    }
}

#[cargo_snippet::snippet("minimum_spanning_arborescence")]
impl Graph {
    /// tarjan
    pub fn minimum_spanning_arborescence<G: Group, F: Fn(usize) -> G::T>(
        &self,
        root: usize,
        group: G,
        weight: F,
    ) -> Option<(G::T, Vec<usize>)>
    where
        G::T: Ord,
    {
        use std::{cmp::Reverse, collections::BinaryHeap};
        let mut uf = MergingUnionFind::new(
            self.vsize,
            |_| (BinaryHeap::new(), group.unit()),
            |x, y| {
                let ny = group.rinv_operate(&y.1, &x.1);
                x.0.extend(
                    (y.0)
                        .drain()
                        .map(|(Reverse(ref w), i)| (Reverse(group.operate(w, &ny)), i)),
                )
            },
        );
        let mut state = vec![0; self.vsize]; // 0: unprocessed, 1: in process, 2: completed
        state[root] = 2;
        for u in self.vertices() {
            for a in self.adjacency(u) {
                uf.find_root_mut(a.to)
                    .data
                    .0
                    .push((Reverse(weight(a.id)), a.id));
            }
        }
        let mut paredge = vec![0; self.esize];
        let mut ord = vec![];
        let mut leaf = vec![self.esize; self.vsize];
        let mut cycle = 0usize;
        let mut acc = group.unit();
        let cache = self.eid_cache();
        for mut cur in self.vertices() {
            if state[cur] != 0 {
                continue;
            }
            let mut path = vec![];
            let mut ch = vec![];
            while state[cur] != 2 {
                path.push(cur);
                state[cur] = 1;
                let (w, eid) = {
                    let (heap, lazy) = &mut uf.find_root_mut(cur).data;
                    match heap.pop() {
                        Some((Reverse(w), eid)) => (group.operate(&w, &lazy), eid),
                        None => return None,
                    }
                };
                {
                    let curw = &mut uf.find_root_mut(cur).data.1;
                    *curw = group.rinv_operate(curw, &w);
                }
                acc = group.operate(&acc, &w);
                ord.push(eid);
                let (u, v) = cache[eid];
                if leaf[v] >= self.esize {
                    leaf[v] = eid;
                }
                while cycle > 0 {
                    paredge[ch.pop().unwrap()] = eid;
                    cycle -= 1;
                }
                ch.push(eid);
                if state[uf.find(u)] == 1 {
                    while let Some(t) = path.pop() {
                        state[t] = 2;
                        cycle += 1;
                        if !uf.unite(u, t) {
                            break;
                        }
                    }
                    state[uf.find(u)] = 1;
                }
                cur = uf.find(u);
            }
            for u in path.into_iter() {
                state[u] = 2;
            }
        }
        let mut tree = vec![root; self.vsize];
        let mut used = vec![false; self.esize];
        for eid in ord.into_iter().rev() {
            if !used[eid] {
                let (u, v) = cache[eid];
                tree[v] = u;
                let mut x = leaf[v];
                while x != eid {
                    used[x] = true;
                    x = paredge[x];
                }
            }
        }
        Some((acc, tree))
    }
}
