use graph::Graph;

pub fn extended_block_cut_tree(g: &Graph<(), ()>) -> Graph<(), ()> {
    let n = g.len();
    let mut next = vec![!0; n];
    let mut d = vec![!0; n];
    let mut imos = vec![0; n];
    for i in 0..n {
        if d[i] == !0 {
            d[i] = 0;
            dfs1(i, &g, &mut next, &mut d, &mut imos);
        }
    }
    let mut cnt = 0;
    let mut used = vec![false; n];
    let mut edges = vec![];
    for i in 0..n {
        if d[i] == 0 {
            dfs2(
                i, cnt, &g, &mut d, &mut imos, &mut used, &mut cnt, &mut edges,
            );
        }
        if g[i].is_empty() {
            edges.push((i, n + cnt));
            cnt += 1;
        }
    }
    Graph::from_unweighted_undirected_edges(n + cnt, &edges)
}

fn dfs1(v: usize, g: &Graph<(), ()>, next: &mut [usize], d: &mut [usize], imos: &mut [i32]) {
    for &(u, _) in &g[v] {
        if d[u] == !0 {
            d[u] = d[v] + 1;
            next[v] = u;
            dfs1(u, g, next, d, imos);
            imos[v] += imos[u];
        } else if d[u] + 1 < d[v] {
            imos[v] += 1;
            imos[next[u]] -= 1;
        }
    }
}

fn dfs2(
    v: usize,
    b: usize,
    g: &Graph<(), ()>,
    d: &mut [usize],
    imos: &mut [i32],
    used: &mut [bool],
    cnt: &mut usize,
    edges: &mut Vec<(usize, usize)>,
) {
    let n = g.len();
    used[v] = true;
    let mut ok = false;
    for &(u, _) in &g[v] {
        if d[u] == d[v] + 1 && !used[u] {
            if imos[u] > 0 {
                if !ok {
                    ok = true;
                    edges.push((v, n + b));
                }
                dfs2(u, b, g, d, imos, used, cnt, edges);
            } else {
                edges.push((v, n + *cnt));
                *cnt += 1;
                dfs2(u, *cnt - 1, g, d, imos, used, cnt, edges);
            }
        }
    }
    if !ok && d[v] > 0 {
        edges.push((v, n + b));
    }
}
