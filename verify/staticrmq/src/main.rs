// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use algebraic::{algebra, monoid};
use disjoint_sparse_table::DisjointSparseTable;
use proconio::input;

algebra!(M, i64);
monoid!(M, 1 << 60, |x: &i64, y: &i64| *x.min(y));

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let spt = DisjointSparseTable::<M>::new(&a);
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        println!("{}", spt.prod(l, r));
    }
}
