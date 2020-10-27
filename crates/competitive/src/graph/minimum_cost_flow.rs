#[codesnip::skip]
use super::BidirectionalSparseGraph;

#[derive(Debug, Clone)]
pub struct PrimalDualBuilder {
    vsize: usize,
    edges: Vec<(usize, usize)>,
    capacities: Vec<u64>,
    costs: Vec<i64>,
}
impl PrimalDualBuilder {
    pub fn new(vsize: usize, esize_expect: usize) -> Self {
        Self {
            vsize,
            edges: Vec::with_capacity(esize_expect),
            capacities: Vec::with_capacity(esize_expect * 2),
            costs: Vec::with_capacity(esize_expect * 2),
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cap: u64, cost: i64) {
        self.edges.push((from, to));
        self.capacities.push(cap);
        self.capacities.push(0);
        debug_assert!(
            cost >= 0,
            "To use negative edge, comment out the early break of PrimalDual::dijkstra."
        );
        self.costs.push(cost);
        self.costs.push(-cost);
    }
    pub fn gen_graph(&mut self) -> BidirectionalSparseGraph {
        let edges = std::mem::take(&mut self.edges);
        BidirectionalSparseGraph::from_edges(self.vsize, edges)
    }
    pub fn build(self, graph: &BidirectionalSparseGraph) -> PrimalDual<'_> {
        let PrimalDualBuilder {
            vsize,
            capacities,
            costs,
            ..
        } = self;
        PrimalDual {
            graph,
            capacities,
            costs,
            potential: std::iter::repeat(0).take(vsize).collect(),
            dist: Vec::with_capacity(vsize),
            prev_vertex: std::iter::repeat(0).take(vsize).collect(),
            prev_edge: std::iter::repeat(0).take(vsize).collect(),
        }
    }
}
impl Extend<(usize, usize, u64, i64)> for PrimalDualBuilder {
    fn extend<T: IntoIterator<Item = (usize, usize, u64, i64)>>(&mut self, iter: T) {
        for (from, to, cap, cost) in iter {
            self.add_edge(from, to, cap, cost)
        }
    }
}

#[derive(Debug)]
pub struct PrimalDual<'a> {
    graph: &'a BidirectionalSparseGraph,
    capacities: Vec<u64>,
    costs: Vec<i64>,
    potential: Vec<i64>,
    dist: Vec<i64>,
    prev_vertex: Vec<usize>,
    prev_edge: Vec<usize>,
}
impl<'a> PrimalDual<'a> {
    fn dijkstra(&mut self, s: usize, t: usize) -> bool {
        use std::{cmp::Reverse, collections::BinaryHeap};
        self.dist.clear();
        self.dist.resize(self.graph.vertices_size(), std::i64::MAX);
        self.dist[s] = 0;
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), s));
        while let Some((Reverse(d), u)) = heap.pop() {
            if self.dist[u] + self.potential[u] < d {
                continue;
            }
            if u == t {
                break; // early break
            }
            for a in self.graph.adjacencies(u) {
                let ncost =
                    self.dist[u] + self.costs[a.id] + self.potential[u] - self.potential[a.to];
                if self.capacities[a.id] > 0 && self.dist[a.to] > ncost {
                    self.dist[a.to] = ncost;
                    self.prev_vertex[a.to] = u;
                    self.prev_edge[a.to] = a.id;
                    heap.push((Reverse(d + self.costs[a.id]), a.to));
                }
            }
        }
        self.dist[t] != std::i64::MAX
    }
    /// Return (flow, cost).
    pub fn minimum_cost_flow_limited(&mut self, s: usize, t: usize, limit: u64) -> (u64, i64) {
        let mut flow = 0;
        let mut cost = 0;
        while flow < limit && self.dijkstra(s, t) {
            for (p, d) in self.potential.iter_mut().zip(self.dist.iter()) {
                *p += *d;
            }
            let mut f = limit - flow;
            let mut v = t;
            while v != s {
                f = f.min(self.capacities[self.prev_edge[v]]);
                v = self.prev_vertex[v];
            }
            flow += f;
            cost += f as i64 * self.potential[t];
            let mut v = t;
            while v != s {
                self.capacities[self.prev_edge[v]] -= f;
                self.capacities[self.prev_edge[v] ^ 1] += f;
                v = self.prev_vertex[v];
            }
        }
        (flow, cost)
    }
    /// Return (flow, cost).
    pub fn minimum_cost_flow(&mut self, s: usize, t: usize) -> (u64, i64) {
        self.minimum_cost_flow_limited(s, t, std::u64::MAX)
    }
    pub fn get_flow(&self, eid: usize) -> u64 {
        self.capacities[eid * 2 + 1]
    }
}