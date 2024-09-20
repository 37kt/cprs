use graph::Graph;

fn dfs1(v: usize, p: usize, g: &Graph<(), ()>, sz: &mut [usize]) {
    sz[v] = 1;
    for &(u, _) in &g[v] {
        if u == p {
            continue;
        }
        dfs1(u, v, g, sz);
        sz[v] += sz[u];
    }
}

fn dfs2(v: usize, p: usize, mid: usize, g: &Graph<(), ()>, sz: &[usize]) -> usize {
    for &(u, _) in &g[v] {
        if u == p {
            continue;
        }
        if sz[u] > mid {
            return dfs2(u, v, mid, g, sz);
        }
    }
    v
}

fn dfs4(
    v: usize,
    p: usize,
    g: &Graph<(), ()>,
    pre_idx: &[usize],
    idx: &mut Vec<usize>,
    par: &mut Vec<usize>,
) {
    idx.push(pre_idx[v]);
    par.push(pre_idx[p]);
    for &(u, _) in &g[v] {
        if u == p {
            continue;
        }
        dfs4(u, v, g, pre_idx, idx, par);
    }
}

fn dfs3(
    v: usize,
    g: &Graph<(), ()>,
    pre_idx: &[usize],
    conv: &mut [usize],
    f: &mut impl FnMut(&[usize], &[usize], usize),
) {
    let n = g.len();
    let mut sz = vec![0; n];
    dfs1(v, !0, g, &mut sz);
    let c = dfs2(v, !0, sz[v] / 2, g, &sz);
    dfs1(c, !0, g, &mut sz);
    let n = sz[c];
    if n <= 2 {
        return;
    }
    let mut szsum = 0;
    let mut rl = vec![];
    let mut rr = vec![];
    for &(u, _) in &g[c] {
        if szsum + sz[u] <= (n - 1) / 2 {
            szsum += sz[u];
            rl.push(u);
        } else {
            rr.push(u);
        }
    }
    conv[pre_idx[c]] = 0;
    let mut idx_l = vec![];
    let mut par_l = vec![];
    let mut es_l = vec![];
    for &u in &rl {
        dfs4(u, c, g, pre_idx, &mut idx_l, &mut par_l);
    }
    for i in 0..idx_l.len() {
        conv[idx_l[i]] = i + 1;
        es_l.push((conv[par_l[i]], i + 1));
    }
    let mut idx_r = vec![];
    let mut par_r = vec![];
    let mut es_r = vec![];
    for &u in &rr {
        dfs4(u, c, g, pre_idx, &mut idx_r, &mut par_r);
    }
    for i in 0..idx_r.len() {
        conv[idx_r[i]] = i + 1;
        es_r.push((conv[par_r[i]], i + 1));
    }
    let gl = Graph::from_unweighted_undirected_edges(idx_l.len() + 1, &es_l);
    let gr = Graph::from_unweighted_undirected_edges(idx_r.len() + 1, &es_r);
    let mut idx = vec![];
    idx.append(&mut idx_l.clone());
    idx.append(&mut idx_r.clone());
    let mut par = vec![];
    par.append(&mut par_l);
    par.append(&mut par_r);
    f(&idx, &par, idx_l.len());
    idx_l.insert(0, pre_idx[c]);
    idx_r.insert(0, pre_idx[c]);
    dfs3(0, &gl, &idx_l, conv, f);
    dfs3(0, &gr, &idx_r, conv, f);
}

/// f: (idx, par, m)
/// par[0] を根とする部分木がトポロジカル順序で渡される
/// idx: 頂点番号
/// par: 親の頂点番号
/// idx[..m] が赤，idx[m..] が青
pub fn centroid_decomposition(g: &Graph<(), ()>, f: &mut impl FnMut(&[usize], &[usize], usize)) {
    let n = g.len();
    let mut conv = vec![!0; n];
    dfs3(0, g, &(0..n).collect::<Vec<_>>(), &mut conv, f);
}
