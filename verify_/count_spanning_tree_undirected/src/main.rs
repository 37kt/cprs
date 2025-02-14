// verification-helper: PROBLEM https://judge.yosupo.jp/problem/counting_spanning_tree_undirected

use count_spanning_tree_undirected::count_spanning_tree_undirected;
use modint::ModInt998244353 as Mint;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }
    let mut g = vec![vec![0; n]; n];
    for &(u, v) in &uv {
        g[u][v] += 1;
        g[v][u] += 1;
    }
    let res: Mint = count_spanning_tree_undirected(&g);
    println!("{}", res);
}
