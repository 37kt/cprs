// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use proconio::input;
use union_find::UnionFind;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut uf = UnionFind::<true>::new(n);
    for _ in 0..q {
        input! {
            t: usize,
            u: usize,
            v: usize,
        }
        if t == 0 {
            uf.merge(u, v);
        } else {
            println!("{}", uf.same(u, v) as i64);
        }
    }
}
