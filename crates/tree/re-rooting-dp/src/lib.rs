use algebraic::{Act, Monoid};
use graph::Graph;

pub struct ReRootingDP<S>
where
    S: Clone,
{
    par: Vec<usize>,
    dp: Vec<S>,
    dpl: Vec<S>,
    dph: Vec<S>,
}

impl<T> ReRootingDP<T>
where
    T: Clone,
{
    pub fn build<M, V, E>(g: &Graph<V::S, E::S>) -> Self
    where
        M: Monoid<S = T>,
        M::S: Clone,
        V: Act<X = M::S>,
        V::S: Copy,
        E: Act<X = M::S>,
        E::S: Copy,
    {
        let n = g.size();
        let mut ord = vec![];
        let mut par = vec![!0; n];
        let mut st = vec![0];
        while let Some(v) = st.pop() {
            ord.push(v);
            for &(u, _) in g.out_edges(v) {
                if u != 0 && par[u] == !0 {
                    par[u] = v;
                    st.push(u);
                }
            }
        }

        let mut dpl = vec![M::e(); n];
        let mut dph = vec![M::e(); n];
        let mut dp = vec![M::e(); n];
        for &v in ord.iter().rev() {
            let m = g.out_edges(v).len();
            let mut xl = vec![M::e(); m + 1];
            let mut xr = vec![M::e(); m + 1];
            for i in 0..m {
                let u = g.out_edges(v)[i].0;
                if u == par[v] {
                    xl[i + 1] = xl[i].clone();
                } else {
                    xl[i + 1] = M::op(&xl[i], &dph[u]);
                }
            }
            for i in (0..m).rev() {
                let (u, _) = g.out_edges(v)[i];
                if u == par[v] {
                    xr[i] = xr[i + 1].clone();
                } else {
                    xr[i] = M::op(&dph[u], &xr[i + 1]);
                }
            }
            for i in 0..m {
                let (u, _) = g.out_edges(v)[i];
                if u != par[v] {
                    dph[u] = M::op(&xl[i], &xr[i + 1]);
                }
            }
            dp[v] = xr[0].clone();
            dpl[v] = V::act(&g.vertex(v), &dp[v]);
            for &(u, w) in g.out_edges(v) {
                if u == par[v] {
                    dph[v] = E::act(&w, &dpl[v]);
                }
            }
        }
        dp[0] = V::act(&g.vertex(0), &dp[0]);
        for &(u, _) in g.out_edges(0) {
            dph[u] = V::act(&g.vertex(0), &dph[u]);
        }
        for &v in &ord {
            for &(u, w) in g.out_edges(v) {
                if u == par[v] {
                    continue;
                }
                let mut x = E::act(&w, &dph[u]);
                for &(vv, _) in g.out_edges(u) {
                    if vv == v {
                        continue;
                    }
                    dph[vv] = M::op(&dph[vv], &x);
                    dph[vv] = V::act(&g.vertex(u), &dph[vv]);
                }
                x = M::op(&dp[u], &x);
                dp[u] = V::act(&g.vertex(u), &x);
            }
        }
        Self { par, dp, dpl, dph }
    }

    pub fn prod(&self, v: usize) -> T {
        self.dp[v].clone()
    }

    pub fn prod_subtree(&self, v: usize, p: usize) -> T {
        if p == !0 {
            self.dp[v].clone()
        } else if self.par[v] == p {
            self.dpl[v].clone()
        } else {
            self.dph[p].clone()
        }
    }
}
