use graph::Graph;

fn dfs(
    v: usize,
    g: &Graph<(), ()>,
    ord: &mut Vec<usize>,
    par: &mut Vec<usize>,
    imos: &mut Vec<i32>,
    used: &mut Vec<bool>,
) {
    ord.push(v);
    for i in 0..g[v].len() {
        let u = g[v][i].0;
        let e = g.edge_id(v, i);
        if used[e] {
            continue;
        }
        if par[u] != !1 {
            imos[v] += 1;
            imos[u] -= 1;
            used[e] = true;
        } else {
            used[e] = true;
            par[u] = v;
            dfs(u, g, ord, par, imos, used);
        }
    }
}

/// 二辺連結成分分解をする  
/// 橋を除いたときの連結成分を求める
///
/// # 戻り値
///
/// (groups, comp)
/// - groups: 二辺連結成分のグループ
/// - comp: 各頂点が属する二辺連結成分の番号
pub fn two_edge_connected_components(g: &Graph<(), ()>) -> (Vec<Vec<usize>>, Vec<usize>) {
    let n = g.len();
    let m = g.edges_count();
    let mut ord = vec![];
    let mut par = vec![!1; n];
    let mut imos = vec![0; n];
    let mut used = vec![false; m];
    for v in 0..n {
        if par[v] == !1 {
            par[v] = !0;
            dfs(v, g, &mut ord, &mut par, &mut imos, &mut used);
        }
    }
    for &v in ord.iter().rev() {
        if par[v] != !0 {
            imos[par[v]] += imos[v];
        }
    }
    let mut comp = vec![!0; n];
    let mut comp_cnt = 0;
    for &v in &ord {
        if imos[v] == 0 {
            comp[v] = comp_cnt;
            comp_cnt += 1;
        } else {
            comp[v] = comp[par[v]];
        }
    }
    let mut groups = vec![vec![]; comp_cnt];
    for v in 0..n {
        groups[comp[v]].push(v);
    }
    (groups, comp)
}
