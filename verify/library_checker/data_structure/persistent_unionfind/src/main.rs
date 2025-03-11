// verification-helper: PROBLEM https://judge.yosupo.jp/problem/persistent_unionfind

use persistent_union_find::PersistentUnionFind;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut ufs = Vec::with_capacity(q + 1);
    ufs.push(PersistentUnionFind::new(n));

    for _ in 0..q {
        input! {
            t: usize,
            k: isize,
            u: usize,
            v: usize,
        }
        let uf = ufs[(k + 1) as usize];
        if t == 0 {
            let (uf, _) = uf.merge(u, v);
            ufs.push(uf);
        } else {
            println!("{}", uf.same(u, v) as i32);
            ufs.push(uf);
        }
    }
}
