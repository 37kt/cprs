// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use algebraic_structure::magma::AddOperator;
use fenwick_tree::FenwickTree;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let mut ft: FenwickTree<AddOperator<i64>> = a.into_iter().collect();
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                p: usize,
                x: i64,
            }
            ft.add(p, x);
        } else {
            input! {
                l: usize,
                r: usize,
            }
            let s = ft.fold(l..r);
            println!("{}", s);
        }
    }
}
