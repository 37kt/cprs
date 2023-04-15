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
        for &v in ord.iter().rev() {
            for &(u, w) in g.out_edges(v) {
                if u == par[v] {
                    continue;
                }
                dpl[v] = merge(&dpl[v], &add_edge(&dpl[u], w));
            }
            dpl[v] = add_vertex(&dpl[v], g.vertex(v));
        }
        let mut dph = vec![identity.clone(); n];
        let mut dp = vec![identity.clone(); n];
        for &v in &ord {
            let p = par[v];
            let m = g.out_edges(v).len();
            let mut dpsl = vec![identity.clone(); m + 1];
            let mut dpsr = vec![identity.clone(); m + 1];
            for i in 0..m {
                let (u, w) = g.out_edges(v)[i];
                if u == p {
                    dpsl[i + 1] = dpsl[i].clone();
                } else {
                    dpsl[i + 1] = merge(&dpsl[i], &add_edge(&dpl[u], w));
                }
            }
            for i in (0..m).rev() {
                let (u, w) = g.out_edges(v)[i];
                if u == p {
                    dpsr[i] = dpsr[i + 1].clone();
                } else {
                    dpsr[i] = merge(&dpsr[i + 1], &add_edge(&dpl[u], w));
                }
            }
            dp[v] = add_vertex(&merge(&dpsl[m], &dph[v]), g.vertex(v));
            for i in 0..m {
                let (u, _) = g.out_edges(v)[i];
                if u == p {
                    continue;
                }
                for &(vv, w) in g.out_edges(u) {
                    if vv != v {
                        continue;
                    }
                    let t = merge(&merge(&dpsl[i], &dpsr[i + 1]), &dph[v]);
                    dph[u] = add_edge(&add_vertex(&t, g.vertex(v)), w);
                }
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
