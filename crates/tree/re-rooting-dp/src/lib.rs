use algebraic::{Act, Monoid};
use graph::Graph;

/// 全方位木 dp
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
    /// 全方位木 dp を実行する
    pub fn build<M, V, E>(g: &Graph<V::S, E::S>) -> Self
    where
        M: Monoid<S = T>,
        M::S: Clone,
        V: Act<X = M::S>,
        V::S: Clone,
        E: Act<X = M::S>,
        E::S: Clone,
    {
        let n = g.len();
        let mut ord = vec![];
        let mut par = vec![!0; n];
        let mut st = vec![0];
        while let Some(v) = st.pop() {
            ord.push(v);
            for &(u, _) in &g[v] {
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
            let m = g[v].len();
            let mut xl = vec![M::e(); m + 1];
            let mut xr = vec![M::e(); m + 1];
            for i in 0..m {
                let (u, _) = g[v][i];
                if u == par[v] {
                    xl[i + 1] = xl[i].clone();
                } else {
                    xl[i + 1] = M::op(&xl[i], &dph[u]);
                }
            }
            for i in (0..m).rev() {
                let (u, _) = g[v][i];
                if u == par[v] {
                    xr[i] = xr[i + 1].clone();
                } else {
                    xr[i] = M::op(&dph[u], &xr[i + 1]);
                }
            }
            for i in 0..m {
                let (u, _) = g[v][i];
                if u != par[v] {
                    dph[u] = M::op(&xl[i], &xr[i + 1]);
                }
            }
            dp[v] = xr[0].clone();
            dpl[v] = V::act(&g.vertex(v), &dp[v]);
            for (u, w) in &g[v] {
                if *u == par[v] {
                    dph[v] = E::act(&w, &dpl[v]);
                }
            }
        }
        dp[0] = V::act(&g.vertex(0), &dp[0]);
        for &(u, _) in &g[0] {
            dph[u] = V::act(&g.vertex(0), &dph[u]);
        }
        for &v in &ord {
            for (u, w) in &g[v] {
                if *u == par[v] {
                    continue;
                }
                let mut x = E::act(&w, &dph[*u]);
                for &(vv, _) in &g[*u] {
                    if vv == v {
                        continue;
                    }
                    dph[vv] = M::op(&dph[vv], &x);
                    dph[vv] = V::act(&g.vertex(*u), &dph[vv]);
                }
                x = M::op(&dp[*u], &x);
                dp[*u] = V::act(&g.vertex(*u), &x);
            }
        }
        Self { par, dp, dpl, dph }
    }

    /// `v` を根としたときの `dp[v]` の値を返す
    pub fn prod(&self, v: usize) -> T {
        self.dp[v].clone()
    }

    /// `p` を根としたときの `dp[v]` の値を返す  
    /// `p` = `!0` のときは `v` を根としたときの `dp[v]` の値を返す  
    /// `v` と `p` は直接辺で結ばれている必要がある
    pub fn prod_subtree(&self, v: usize, p: usize) -> T {
        if p == !0 {
            self.dp[v].clone()
        } else if self.par[v] == p {
            self.dpl[v].clone()
        } else if self.par[p] == v {
            self.dph[p].clone()
        } else {
            panic!("v and p are not connected by an edge");
        }
    }

    /// 0 を根としたときの `v` の親を返す  
    /// `v` が根のときは `!0` を返す
    pub fn par(&self, v: usize) -> usize {
        self.par[v]
    }
}
