// verification-helper: PROBLEM https://yukicoder.me/problems/no/1479

use max_flow::MaxFlow;
use proconio::{fastout, input};

const M: usize = 500_001;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let mut zh = vec![vec![]; M];
    let mut zw = vec![vec![]; M];
    let mut b = vec![vec![]; M];
    for i in 0..h {
        for j in 0..w {
            zh[a[i][j]].push(i);
            zw[a[i][j]].push(j);
            b[a[i][j]].push((i, j));
        }
    }
    for i in 0..M {
        zh[i].sort_unstable();
        zh[i].dedup();
        zw[i].sort_unstable();
        zw[i].dedup();
    }
    let mut res = 0;
    for x in 1..M {
        if zh[x].is_empty() {
            continue;
        }
        let mut g = MaxFlow::new();
        let ls = g.add_vertices(zh[x].len());
        let rs = g.add_vertices(zw[x].len());
        let src = g.add_vertex();
        let dst = g.add_vertex();
        for &v in &ls {
            g.add_edge(src, v, 1);
        }
        for &v in &rs {
            g.add_edge(v, dst, 1);
        }
        for &(i, j) in &b[x] {
            let p = zh[x].binary_search(&i).unwrap();
            let q = zw[x].binary_search(&j).unwrap();
            g.add_edge(ls[p], rs[q], 1);
        }
        res += g.max_flow(src, dst);
    }
    println!("{}", res);
}
