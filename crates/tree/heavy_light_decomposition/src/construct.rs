use crate::{Edge, HeavyLightDecomposition};

impl HeavyLightDecomposition {
    /// 親配列から構築  
    /// `0` が根。 `i > 0` について `par[i] < i`  
    /// `par[0]` の値は無視される
    pub fn from_parents(par: &[usize]) -> Self {
        let n = par.len();
        let root = 0;

        let mut down = vec![-1; n];
        let mut next = std::iter::once(-1)
            .chain(par.iter().skip(1).map(|&p| p as i32))
            .collect::<Vec<_>>();
        let mut sub = vec![1; n];

        for v in (1..n).rev() {
            let p = next[v] as usize;
            sub[p] += sub[v];
            down[p] = down[p].max(sub[v]);
        }
        for v in (1..n).rev() {
            let p = next[v] as usize;
            if down[p] == sub[v] {
                sub[v] = !sub[v];
                down[p] = !down[p];
            }
        }

        sub[root] = !down[root] + 1;
        down[root] = 0;

        for v in 1..n {
            let p = next[v] as usize;
            let nsub = !down[v] + 1;
            if sub[v] < 0 {
                down[v] = down[p] + 1;
                next[v] = if next[p] < 0 { p as i32 } else { next[p] };
            } else {
                down[v] = down[p] + sub[p];
                sub[p] += sub[v];
                next[v] = !(p as i32);
            }
            sub[v] = nsub;
        }

        let mut tour = vec![-1; n];
        for (v, &d) in down.iter().enumerate() {
            let t = d as usize;
            tour[t] = v as i32;
        }

        let mut edge_ord = vec![0; n - 1];
        for (e, &d) in down.iter().enumerate().skip(1) {
            let i = d as usize - 1;
            edge_ord[i] = e;
        }

        Self {
            n,
            root,
            down,
            next,
            sub,
            tour,
            edge_ord,
        }
    }

    /// &[(usize, usize)] か &[(usize, usize, T)] から構築
    pub fn from_edges(edges: &[impl Edge], root: usize) -> Self {
        let n = edges.len() + 1;
        assert!(root < n);

        let mut down = vec![0; n];
        let mut next = vec![0; n];
        for e in edges {
            let (u, v) = e.endpoints();
            down[u] += 1;
            down[v] += 1;
            next[u] ^= v as i32;
            next[v] ^= u as i32;
        }

        let mut tour = Vec::with_capacity(n);
        for (v, &d) in down.iter().enumerate() {
            if v != root && d == 1 {
                tour.push(v as i32);
            }
        }
        for i in 0..n - 1 {
            let v = tour[i] as usize;
            down[v] = -1;
            let u = next[v] as usize;
            next[u] ^= v as i32;
            down[u] -= 1;
            if down[u] == 1 && u != root {
                tour.push(u as i32);
            }
        }

        let mut sub = vec![1; n];
        for &v in &tour {
            let v = v as usize;
            let p = next[v] as usize;
            sub[p] += sub[v];
            down[p] = down[p].max(sub[v]);
        }
        for &v in &tour {
            let v = v as usize;
            let p = next[v] as usize;
            if down[p] == sub[v] {
                sub[v] = !sub[v];
                down[p] = !down[p];
            }
        }

        sub[root] = !down[root] + 1;
        down[root] = 0;
        next[root] = -1;

        for &v in tour.iter().rev() {
            let v = v as usize;
            let p = next[v] as usize;
            let nsub = !down[v] + 1;
            if sub[v] < 0 {
                down[v] = down[p] + 1;
                next[v] = if next[p] < 0 { p as i32 } else { next[p] };
            } else {
                down[v] = down[p] + sub[p];
                sub[p] += sub[v];
                next[v] = !(p as i32);
            }
            sub[v] = nsub;
        }

        tour.resize(n, -1);
        for (v, &d) in down.iter().enumerate() {
            let t = d as usize;
            tour[t] = v as i32;
        }

        let mut edge_ord = vec![0; n - 1];
        for (e, edge) in edges.iter().enumerate() {
            let (u, v) = edge.endpoints();
            let i = down[u].max(down[v]) as usize - 1;
            edge_ord[i] = e;
        }

        Self {
            n,
            root,
            down,
            next,
            sub,
            tour,
            edge_ord,
        }
    }
}
