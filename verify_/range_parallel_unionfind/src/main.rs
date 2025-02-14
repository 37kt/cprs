// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_parallel_unionfind

use modint::ModInt998244353 as Mint;
use proconio::input;
use range_union_find::RangeUnionFind;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut x: [Mint; n],
    }
    let mut sum = Mint::new(0);
    let mut uf = RangeUnionFind::new(n);
    for _ in 0..q {
        input! {
            k: usize,
            a: usize,
            b: usize,
        }
        for (u, v) in uf.merge_range(a..a + k, b..b + k) {
            sum += x[u] * x[v];
            x[u] = x[u] + x[v];
        }
        println!("{}", sum);
    }
}
