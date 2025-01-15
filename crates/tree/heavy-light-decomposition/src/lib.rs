// https://noya2ruler.github.io/noya2_Library/tree/heavy_light_decomposition.hpp

use csr_array::CSRArray;
use graph::Graph;

#[derive(Clone)]
pub struct HeavyLightDecomposition {
    n: usize,
    root: usize,
    down: Vec<i32>,
    next: Vec<i32>,
    sub: Vec<i32>,
    tour: Vec<i32>,
}

impl HeavyLightDecomposition {
    pub fn new<V: Clone, E: Clone>(g: &Graph<V, E, false>, root: usize) -> Self {
        let n = g.len();
        let mut down = vec![0; n];
        let mut next = vec![0; n];
        let mut sub = vec![1; n];
        let mut tour = vec![-1; n];

        for u in 0..n {
            for &(v, _) in &g[u] {
                if u > v {
                    continue;
                }
                down[u] += 1;
                down[v] += 1;
                next[u] ^= v as i32;
                next[v] ^= u as i32;
            }
        }

        let mut back = 0;
        for u in 0..n {
            if u != root && down[u] == 1 {
                tour[back] = u as i32;
                back += 1;
            }
        }
        for front in 0..n - 1 {
            let u = tour[front] as usize;
            down[u] = -1;
            let v = next[u] as usize;
            next[v] ^= u as i32;
            down[v] -= 1;
            if down[v] == 1 && v != root {
                tour[back] = v as i32;
                back += 1;
            }
        }
        tour.pop();

        for &u in &tour {
            let u = u as usize;
            let v = next[u] as usize;
            sub[v] += sub[u];
            down[v] = down[v].max(sub[u]);
        }
        for &u in &tour {
            let u = u as usize;
            let v = next[u] as usize;
            if down[v] == sub[u] {
                sub[u] = !sub[u];
                down[v] = !down[v];
            }
        }

        sub[root] = !down[root] + 1;
        down[root] = 0;
        next[root] = -1;
        for &u in tour.iter().rev() {
            let u = u as usize;
            let v = next[u] as usize;
            let nsub = !down[u] + 1;
            if sub[u] < 0 {
                down[u] = down[v] + 1;
                next[u] = if next[v] < 0 { v as i32 } else { next[v] };
            } else {
                down[u] = down[v] + sub[v];
                sub[v] += sub[u];
                next[u] = !(v as i32);
            }
            sub[u] = nsub;
        }

        tour.push(0);
        for u in 0..n {
            tour[down[u] as usize] = u as i32;
        }

        Self {
            n,
            root,
            down,
            next,
            sub,
            tour,
        }
    }

    /// par[i] < i, par[0] == !0
    pub fn from_parents(par: &[usize]) -> Self {
        let n = par.len();
        let mut down = vec![-1; n];
        let mut next: Vec<_> = par.iter().map(|&u| u as i32).collect();
        let mut sub = vec![1; n];
        let mut tour = vec![-1; n];

        for u in (1..n).rev() {
            let v = next[u] as usize;
            sub[v] += sub[u];
            down[v] = down[v].max(sub[u]);
        }
        for u in (1..n).rev() {
            let v = next[u] as usize;
            if down[v] == sub[u] {
                sub[u] = !sub[u];
                down[v] = !down[v];
            }
        }

        sub[0] = !down[0] + 1;
        down[0] = 0;
        for u in 1..n {
            let v = next[u] as usize;
            let nsub = !down[u] + 1;
            if sub[u] < 0 {
                down[u] = down[v] + 1;
                next[u] = if next[v] < 0 { v as i32 } else { next[v] };
            } else {
                down[u] = down[v] + sub[v];
                sub[v] += sub[u];
                next[u] = !(v as i32);
            }
            sub[u] = nsub;
        }

        for u in 0..n {
            tour[down[u] as usize] = u as i32;
        }

        Self {
            n,
            root: 0,
            down,
            next,
            sub,
            tour,
        }
    }

    pub fn root(&self) -> usize {
        self.root
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn leader(&self, v: usize) -> usize {
        if self.next[v] < 0 {
            v
        } else {
            self.next[v] as usize
        }
    }

    pub fn la(&self, mut v: usize, mut d: usize) -> usize {
        while v != !0 {
            let u = self.leader(v);
            if self.down[v] as usize >= self.down[u] as usize + d {
                v = self.tour[self.down[v] as usize - d] as usize;
                break;
            }
            d -= 1 + (self.down[v] - self.down[u]) as usize;
            v = if u == self.root {
                !0
            } else {
                (!self.next[u]) as usize
            };
        }
        v
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        let mut du = self.down[u] as usize;
        let mut dv = self.down[v] as usize;
        if du > dv {
            std::mem::swap(&mut du, &mut dv);
            std::mem::swap(&mut u, &mut v);
        }
        if dv < du + self.sub[u] as usize {
            return u;
        }
        while du < dv {
            v = (!self.next[self.leader(v)]) as usize;
            dv = self.down[v] as usize;
        }
        v
    }

    pub fn dist(&self, mut u: usize, mut v: usize) -> usize {
        let mut d = 0;
        while self.leader(u) != self.leader(v) {
            if self.down[u] > self.down[v] {
                std::mem::swap(&mut u, &mut v);
            }
            d += 1 + self.down[v] - self.down[self.leader(v)];
            v = (!self.next[self.leader(v)]) as usize;
        }
        (d + (self.down[u] - self.down[v]).abs()) as usize
    }

    pub fn jump(&self, u: usize, v: usize, d: usize) -> usize {
        let mut d = d as i32;
        let mut uu = u;
        let mut vv = v;
        let mut dist_u_lca = 0;
        let mut dist_v_lca = 0;
        while self.leader(uu) != self.leader(vv) {
            if self.down[uu] > self.down[vv] {
                dist_u_lca += 1 + self.down[uu] - self.down[self.leader(uu)];
                uu = (!self.next[self.leader(uu)]) as usize;
            } else {
                dist_v_lca += 1 + self.down[vv] - self.down[self.leader(vv)];
                vv = (!self.next[self.leader(vv)]) as usize;
            }
        }
        if self.down[uu] > self.down[vv] {
            dist_u_lca += self.down[uu] - self.down[vv];
        } else {
            dist_v_lca += self.down[vv] - self.down[uu];
        }
        if d <= dist_u_lca {
            return self.la(u, d as usize);
        }
        d -= dist_u_lca;
        if d <= dist_v_lca {
            self.la(v, (dist_v_lca - d) as usize)
        } else {
            !0
        }
    }

    pub fn parent(&self, v: usize) -> usize {
        if v == self.root {
            !0
        } else if self.next[v] < 0 {
            (!self.next[v]) as usize
        } else {
            self.tour[self.down[v] as usize - 1] as usize
        }
    }

    pub fn vertex_index(&self, v: usize) -> usize {
        self.down[v] as usize
    }

    pub fn edge_index(&self, u: usize, v: usize) -> usize {
        self.down[u].max(self.down[v]) as usize - 1
    }

    pub fn subtree_range(&self, v: usize) -> (usize, usize) {
        (self.down[v] as usize, (self.down[v] + self.sub[v]) as usize)
    }

    pub fn is_in_subtree(&self, r: usize, v: usize) -> bool {
        let (rl, rr) = self.subtree_range(r);
        let (vl, vr) = self.subtree_range(v);
        rl <= vl && vr <= rr
    }

    pub fn dist_table(&self, mut s: usize) -> Vec<usize> {
        let mut dist = vec![!0; self.n];
        dist[s] = 0;
        while s != self.root {
            dist[self.parent(s)] = dist[s] + 1;
            s = self.parent(s);
        }
        for &v in &self.tour {
            let v = v as usize;
            if dist[v] == !0 {
                dist[v] = dist[self.parent(v)] + 1;
            }
        }
        dist
    }

    pub fn diameter(&self) -> (usize, usize, usize) {
        let depth = self.dist_table(self.root);
        let u = depth.iter().enumerate().max_by_key(|&(_, &d)| d).unwrap().0;
        let dist_u = self.dist_table(u);
        let v = dist_u
            .iter()
            .enumerate()
            .max_by_key(|&(_, &d)| d)
            .unwrap()
            .0;
        (dist_u[v], u, v)
    }

    pub fn path(&self, mut u: usize, mut v: usize) -> Vec<usize> {
        let d = self.dist(u, v);
        let mut path = vec![0; d + 1];
        let mut front = 0;
        let mut back = d;
        while u != v {
            if self.down[u] > self.down[v] {
                path[front] = u;
                front += 1;
                u = self.parent(u);
            } else {
                path[back] = v;
                back -= 1;
                v = self.parent(v);
            }
        }
        path[front] = u;
        path
    }

    /// f: (l, r, reverse)
    pub fn path_query(
        &self,
        mut u: usize,
        mut v: usize,
        mut f: impl FnMut(usize, usize, bool),
        vertex_query: bool,
    ) {
        let mut up_path = vec![];
        let mut down_path = vec![];
        while self.leader(u) != self.leader(v) {
            if self.down[u] < self.down[v] {
                let l = self.down[self.leader(v)] as usize;
                let r = self.down[v] as usize + 1;
                assert!(l < r);
                down_path.push((l, r));
                v = (!self.next[self.leader(v)]) as usize;
            } else {
                let l = self.down[self.leader(u)] as usize;
                let r = self.down[u] as usize + 1;
                assert!(l < r);
                up_path.push((l, r));
                u = (!self.next[self.leader(u)]) as usize;
            }
        }
        if vertex_query {
            let du = self.down[u] as usize;
            let dv = self.down[v] as usize;
            if du < dv {
                down_path.push((du, dv + 1));
            } else {
                up_path.push((dv, du + 1));
            }
        } else {
            let du = self.down[u] as usize;
            let dv = self.down[v] as usize;
            if du < dv {
                down_path.push((du + 1, dv + 1));
            } else {
                up_path.push((dv + 1, du + 1));
            }
        }

        if !vertex_query {
            for (l, r) in up_path.iter_mut() {
                *l -= 1;
                *r -= 1;
            }
            for (l, r) in down_path.iter_mut() {
                *l -= 1;
                *r -= 1;
            }
        }
        for &(l, r) in &up_path {
            f(l, r, true);
        }
        for &(l, r) in down_path.iter().rev() {
            f(l, r, false);
        }
    }

    pub fn euler_tour(&self) -> Vec<usize> {
        self.tour.iter().map(|&u| u as usize).collect()
    }

    pub fn heavy_child(&self, v: usize) -> usize {
        if (self.down[v] + 1) as usize >= self.n {
            return !0;
        }
        let u = self.tour[self.down[v] as usize + 1] as usize;
        if self.parent(u) == v {
            u
        } else {
            !0
        }
    }

    pub fn parents(&self) -> Vec<usize> {
        (0..self.n).map(|i| self.parent(i)).collect()
    }

    pub fn children(&self) -> CSRArray<usize> {
        let children = self
            .tour
            .iter()
            .skip(1)
            .map(|&v| (self.parent(v as usize), v as usize))
            .collect::<Vec<_>>();
        CSRArray::new(self.n, &children)
    }
}
