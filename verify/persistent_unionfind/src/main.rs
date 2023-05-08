// verification-helper: PROBLEM https://judge.yosupo.jp/problem/persistent_unionfind

use persistent_union_find::PersistentUnionFind;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut uf = vec![PersistentUnionFind::new(n); q + 1];
    for i in 1..=q {
        input! {
            t: usize,
            k: isize,
            u: usize,
            v: usize,
        }
        let k = (k + 1) as usize;
        if t == 0 {
            uf[i] = uf[k].merge(u, v);
        } else {
            println!("{}", uf[k].same(u, v) as usize);
        }
    }
}
