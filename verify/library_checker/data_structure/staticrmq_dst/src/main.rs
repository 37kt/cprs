// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use algebraic_structure::magma::MinOperator;
use disjoint_sparse_table::DisjointSparseTable;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i32; n],
    }
    let dst = DisjointSparseTable::<MinOperator<i32>>::from_iter(a);
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        println!("{}", dst.fold(l..r));
    }
}
