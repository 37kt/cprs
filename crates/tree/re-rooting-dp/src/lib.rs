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

impl<S> ReRootingDP<S>
where
    S: Clone,
{
    pub fn build<V, E, Merge, AddVertex, AddEdge>(
        g: &Graph<V, E>,
        identity: S,
        merge: Merge,
        add_vertex: AddVertex,
        add_edge: AddEdge,
    ) -> Self
    where
        V: Copy,
        E: Copy,
        Merge: Fn(&S, &S) -> S,
        AddVertex: Fn(&S, V) -> S,
        AddEdge: Fn(&S, E) -> S,
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

        let mut dpl = vec![identity.clone(); n];
        let mut dph = vec![identity.clone(); n];
        let mut dp = vec![identity.clone(); n];
        for &v in ord.iter().rev() {
            let m = g.out_edges(v).len();
            let mut xl = vec![identity.clone(); m + 1];
            let mut xr = vec![identity.clone(); m + 1];
            for i in 0..m {
                let u = g.out_edges(v)[i].0;
                if u == par[v] {
                    xl[i + 1] = xl[i].clone();
                } else {
                    xl[i + 1] = merge(&xl[i], &dph[u]);
                }
            }
            for i in (0..m).rev() {
                let u = g.out_edges(v)[i].0;
                if u == par[v] {
                    xr[i] = xr[i + 1].clone();
                } else {
                    xr[i] = merge(&dph[u], &xr[i + 1]);
                }
            }
            for i in 0..m {
                let u = g.out_edges(v)[i].0;
                if u != par[v] {
                    dph[u] = merge(&xl[i], &xr[i + 1]);
                }
            }
            dp[v] = xr[0].clone();
            dpl[v] = add_vertex(&dp[v], g.vertex(v));
            for &(u, w) in g.out_edges(v) {
                if u == par[v] {
                    dph[v] = add_edge(&dpl[v], w);
                }
            }
        }
        dp[0] = add_vertex(&dp[0], g.vertex(0));
        for &(u, _) in g.out_edges(0) {
            dph[u] = add_vertex(&dph[u], g.vertex(0));
        }
        for &v in &ord {
            for &(u, w) in g.out_edges(v) {
                if u == par[v] {
                    continue;
                }
                let mut x = add_edge(&dph[u], w);
                for &(vv, _) in g.out_edges(u) {
                    if vv == v {
                        continue;
                    }
                    dph[vv] = merge(&dph[vv], &x);
                    dph[vv] = add_vertex(&dph[vv], g.vertex(u));
                }
                x = merge(&dp[u], &x);
                dp[u] = add_vertex(&x, g.vertex(u));
            }
        }
        Self { dp, par, dpl, dph }
    }

    pub fn prod(&self, v: usize) -> S {
        self.dp[v].clone()
    }

    pub fn prod_subtree(&self, v: usize, p: usize) -> S {
        assert_ne!(p, !0);
        if self.par[v] == p {
            self.dpl[v].clone()
        } else {
            self.dph[p].clone()
        }
    }
}
