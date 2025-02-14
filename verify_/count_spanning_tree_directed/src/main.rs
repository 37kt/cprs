// verification-helper: PROBLEM https://judge.yosupo.jp/problem/counting_spanning_tree_directed

use count_spanning_tree_directed::count_spanning_tree_directed;
use modint::ModInt998244353 as Mint;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        r: usize,
        uv: [(usize, usize); m],
    }
    let mut g = vec![vec![0; n]; n];
    for &(u, v) in &uv {
        g[u][v] += 1;
    }
    let res: Mint = count_spanning_tree_directed(&g, r);
    println!("{}", res);
}
