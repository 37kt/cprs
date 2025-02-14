use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    ops::{Add, AddAssign},
};

use algebraic::{algebra, monoid};
use graph::UndirectedGraph;
use union_find_component_sum::UnionFindComponentSum;

algebra!(ZeroOneMonoid, ZeroOne);
monoid!(ZeroOneMonoid, ZeroOne::new(0, 0), |a, b| a + b);

/// 01 on Tree
pub fn zero_one_on_tree(g: &UndirectedGraph<ZeroOne, ()>) -> usize {
    let n = g.len();
    let mut par = vec![!0; n];
    dfs(0, g, &mut par);
    let mut res = 0;
    let a: Vec<_> = (0..n).map(|i| *g.vertex(i)).collect();
    let mut uf = UnionFindComponentSum::<ZeroOneMonoid, false>::new(&a);
    let mut pq = BinaryHeap::new();
    for v in 1..n {
        pq.push((a[v], v));
    }
    while let Some((x, v)) = pq.pop() {
        if uf.sum(v) != x {
            continue;
        }
        let p = uf.leader(par[v]);
        res += uf.sum(p).s1 * uf.sum(v).s0;
        uf.merge(p, v);
        if !uf.same(0, p) {
            pq.push((uf.sum(p), p));
        }
    }
    res
}

fn dfs(v: usize, g: &UndirectedGraph<ZeroOne, ()>, p: &mut [usize]) {
    for &(u, _) in &g[v] {
        if u == p[v] {
            continue;
        }
        p[u] = v;
        dfs(u, g, p);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ZeroOne {
    s0: usize,
    s1: usize,
}

impl ZeroOne {
    pub fn new(s0: usize, s1: usize) -> Self {
        Self { s0, s1 }
    }
}

impl Ord for ZeroOne {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.s0 * other.s1).cmp(&(other.s0 * self.s1))
    }
}

impl PartialOrd for ZeroOne {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for ZeroOne {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            s0: self.s0 + other.s0,
            s1: self.s1 + other.s1,
        }
    }
}

impl Add for &ZeroOne {
    type Output = ZeroOne;

    fn add(self, other: Self) -> ZeroOne {
        ZeroOne {
            s0: self.s0 + other.s0,
            s1: self.s1 + other.s1,
        }
    }
}

impl Add<&ZeroOne> for ZeroOne {
    type Output = ZeroOne;

    fn add(self, other: &Self) -> ZeroOne {
        ZeroOne {
            s0: self.s0 + other.s0,
            s1: self.s1 + other.s1,
        }
    }
}

impl Add<ZeroOne> for &ZeroOne {
    type Output = ZeroOne;

    fn add(self, other: ZeroOne) -> ZeroOne {
        ZeroOne {
            s0: self.s0 + other.s0,
            s1: self.s1 + other.s1,
        }
    }
}

impl AddAssign for ZeroOne {
    fn add_assign(&mut self, other: Self) {
        self.s0 += other.s0;
        self.s1 += other.s1;
    }
}

impl AddAssign<&ZeroOne> for ZeroOne {
    fn add_assign(&mut self, other: &Self) {
        self.s0 += other.s0;
        self.s1 += other.s1;
    }
}
