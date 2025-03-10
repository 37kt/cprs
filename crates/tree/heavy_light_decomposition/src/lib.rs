// https://noya2ruler.github.io/noya2_Library/tree/heavy_light_decomposition.hpp

pub mod compress;
pub mod construct;

use std::iter::FusedIterator;

use csr_array::{CsrArray, CsrArrayBuilder};

pub struct HeavyLightDecomposition {
    n: usize,
    root: usize,
    down: Vec<i32>,
    next: Vec<i32>,
    sub: Vec<i32>,
    tour: Vec<i32>,
}

impl HeavyLightDecomposition {
    pub fn len(&self) -> usize {
        self.n
    }

    pub fn root(&self) -> usize {
        self.root
    }

    /// 属する heavy path の先頭
    pub fn head(&self, v: usize) -> usize {
        if self.next[v] < 0 {
            v
        } else {
            self.next[v] as usize
        }
    }

    /// 頂点 v の d 個親
    pub fn la(&self, mut v: usize, d: usize) -> Option<usize> {
        let mut d = d as i32;
        while v != !0 {
            let u = self.head(v);
            if self.down[v] - d >= self.down[u] {
                v = self.tour[(self.down[v] - d) as usize] as usize;
                break;
            }
            d -= self.down[v] - self.down[u] + 1;
            v = if u == self.root {
                !0
            } else {
                (!self.next[u]) as usize
            };
        }
        if v == !0 {
            None
        } else {
            Some(v)
        }
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        let mut du = self.down[u];
        let mut dv = self.down[v];
        if du > dv {
            std::mem::swap(&mut u, &mut v);
            std::mem::swap(&mut du, &mut dv);
        }
        if dv < du + self.sub[u] {
            return u;
        }
        while du < dv {
            v = !self.next[self.head(v)] as usize;
            dv = self.down[v];
        }
        v
    }

    /// u を根としたときの lca(v, w)
    pub fn meet(&self, u: usize, v: usize, w: usize) -> usize {
        self.lca(u, v) ^ self.lca(v, w) ^ self.lca(w, u)
    }

    pub fn dist(&self, mut u: usize, mut v: usize) -> usize {
        let mut dist = 0;
        while self.head(u) != self.head(v) {
            if self.down[u] > self.down[v] {
                std::mem::swap(&mut u, &mut v);
            }
            dist += self.down[v] - self.down[self.head(v)] + 1;
            v = !self.next[self.head(v)] as usize;
        }
        dist += (self.down[u] - self.down[v]).abs();
        dist as usize
    }

    pub fn jump(&self, mut s: usize, mut t: usize, d: usize) -> Option<usize> {
        let (ss, tt) = (s, t);
        let (mut dist_sl, mut dist_tl) = (0, 0);
        while self.head(s) != self.head(t) {
            if self.down[s] > self.down[t] {
                dist_sl += self.down[s] - self.down[self.head(s)] + 1;
                s = !self.next[self.head(s)] as usize;
            } else {
                dist_tl += self.down[t] - self.down[self.head(t)] + 1;
                t = !self.next[self.head(t)] as usize;
            }
        }
        if self.down[s] > self.down[t] {
            dist_sl += self.down[s] - self.down[t];
        } else {
            dist_tl += self.down[t] - self.down[s];
        }
        let mut d = d as i32;
        if d <= dist_sl {
            return Some(self.la(ss, d as usize).unwrap());
        }
        d -= dist_sl;
        if d <= dist_tl {
            return Some(self.la(tt, (dist_tl - d) as usize).unwrap());
        }
        None
    }

    pub fn parent(&self, v: usize) -> Option<usize> {
        if v == self.root {
            None
        } else if self.next[v] < 0 {
            Some(!self.next[v] as usize)
        } else {
            Some(self.tour[(self.down[v] - 1) as usize] as usize)
        }
    }

    /// \[0, n)
    pub fn vertex_index(&self, v: usize) -> usize {
        self.down[v] as usize
    }

    /// \[0, n-1)
    pub fn edge_index(&self, u: usize, v: usize) -> usize {
        debug_assert!(self.parent(u) == Some(v) || self.parent(v) == Some(u));
        (self.down[u].max(self.down[v]) - 1) as usize
    }

    pub fn subtree_size(&self, v: usize) -> usize {
        self.sub[v] as usize
    }

    // TODO: l..r を返すのとどっちがいい？
    pub fn subtree_range(&self, v: usize) -> (usize, usize) {
        let l = self.down[v] as usize;
        let r = (self.down[v] + self.sub[v]) as usize;
        (l, r)
    }

    pub fn in_subtree(&self, r: usize, v: usize) -> bool {
        let (l1, r1) = self.subtree_range(r);
        let (l2, r2) = self.subtree_range(v);
        l1 <= l2 && r2 <= r1
    }

    pub fn dist_table(&self, mut s: usize) -> Vec<usize> {
        let mut dist = vec![!0; self.n];
        dist[s] = 0;
        while let Some(p) = self.parent(s) {
            dist[p] = dist[s] + 1;
            s = p;
        }
        for v in 0..self.n {
            if dist[v] == !0 {
                dist[v] = dist[self.parent(v).unwrap()] + 1;
            }
        }
        dist
    }

    /// (dist, (u, v))
    pub fn diameter(&self) -> (usize, (usize, usize)) {
        let depth = self.dist_table(self.root);
        let (_, u) = depth.iter().zip(0..).max().unwrap();
        let from_u = self.dist_table(u);
        let (_, v) = from_u.iter().zip(0..).max().unwrap();
        (from_u[v], (u, v))
    }

    /// \[s, .., t\]
    pub fn path(&self, mut s: usize, mut t: usize) -> Vec<usize> {
        let d = self.dist(s, t);
        let mut path = vec![!0; d + 1];
        let (mut i, mut j) = (0, d);
        while s != t {
            if self.down[s] > self.down[t] {
                path[i] = s;
                i += 1;
                s = self.parent(s).unwrap();
            } else {
                path[j] = t;
                j -= 1;
                t = self.parent(t).unwrap();
            }
        }
        path[i] = s;
        path
    }

    /// f: (l, r, reverse)
    pub fn path_query(
        &self,
        mut s: usize,
        mut t: usize,
        vertex_query: bool,
        mut f: impl FnMut(usize, usize, bool),
    ) {
        let mut f = |l, r, reverse| {
            if vertex_query {
                f(l, r, reverse);
            } else {
                f(l - 1, r - 1, reverse);
            }
        };

        let mut down_query = vec![];
        while self.head(s) != self.head(t) {
            if self.down[s] < self.down[t] {
                let l = self.down[self.head(t)];
                let r = self.down[t] + 1;
                down_query.push((l, r));
                t = !self.next[self.head(t)] as _;
            } else {
                let l = self.down[self.head(s)];
                let r = self.down[s] + 1;
                f(l as _, r as _, true);
                s = !self.next[self.head(s)] as _;
            }
        }

        if vertex_query {
            if self.down[s] < self.down[t] {
                let l = self.down[s];
                let r = self.down[t] + 1;
                f(l as _, r as _, false);
            } else {
                let l = self.down[t];
                let r = self.down[s] + 1;
                f(l as _, r as _, true);
            }
        } else if self.down[s] < self.down[t] {
            let l = self.down[s] + 1;
            let r = self.down[t] + 1;
            f(l as _, r as _, false);
        } else if self.down[s] > self.down[t] {
            let l = self.down[t] + 1;
            let r = self.down[s] + 1;
            f(l as _, r as _, true);
        }

        for &(l, r) in down_query.iter().rev() {
            f(l as _, r as _, false);
        }
    }

    pub fn euler_tour(
        &self,
    ) -> impl Iterator<Item = usize> + FusedIterator + ExactSizeIterator + DoubleEndedIterator + '_
    {
        self.tour.iter().map(|&v| v as usize)
    }

    /// 各頂点の子のリスト  
    /// heavy child が先頭
    pub fn children(&self) -> CsrArray<usize> {
        let mut csr = CsrArrayBuilder::new(self.n);
        for v in self.euler_tour().skip(1) {
            csr.push(self.parent(v).unwrap(), v);
        }
        csr.build()
    }
}

#[doc(hidden)]
pub trait Edge {
    fn endpoints(&self) -> (usize, usize);
}

#[doc(hidden)]
impl Edge for (usize, usize) {
    fn endpoints(&self) -> (usize, usize) {
        *self
    }
}

#[doc(hidden)]
impl<T> Edge for (usize, usize, T) {
    fn endpoints(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}
