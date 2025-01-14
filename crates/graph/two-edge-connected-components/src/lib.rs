use csr_array::CSRArray;
use graph::UndirectedGraph;

fn dfs(
    v: usize,
    p: usize,
    g: &UndirectedGraph<(), ()>,
    comp: &mut [usize],
    comp_cnt: &mut usize,
    ord: &mut [usize],
    low: &mut [usize],
    vs: &mut Vec<usize>,
    t: &mut usize,
) {
    ord[v] = *t;
    low[v] = *t;
    *t += 1;
    let mut f = false;
    for &(u, _) in &g[v] {
        if ord[u] == !0 {
            dfs(u, v, g, comp, comp_cnt, ord, low, vs, t);
            low[v] = low[v].min(low[u]);
        } else if u == p && !f {
            f = true;
        } else {
            low[v] = low[v].min(ord[u]);
        }
    }
    if ord[v] == low[v] {
        while comp[v] == !0 {
            comp[vs.pop().unwrap()] = *comp_cnt;
        }
        *comp_cnt += 1;
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
pub fn two_edge_connected_components(g: &UndirectedGraph<(), ()>) -> (CSRArray<usize>, Vec<usize>) {
    let n = g.len();
    let mut comp = vec![!0; n];
    let mut comp_cnt = 0;
    let mut ord = vec![!0; n];
    let mut low = vec![!0; n];
    let mut vs = vec![];
    let mut t = 0;
    for v in 0..n {
        if ord[v] == !0 {
            dfs(
                v,
                !0,
                g,
                &mut comp,
                &mut comp_cnt,
                &mut ord,
                &mut low,
                &mut vs,
                &mut t,
            );
        }
    }

    let groups = comp
        .iter()
        .enumerate()
        .map(|(v, &c)| (c, v))
        .collect::<Vec<_>>();
    let groups = CSRArray::new(comp_cnt, &groups);

    (groups, comp)
}
