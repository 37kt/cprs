// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_subtree_sum

use algebraic_structure::magma::AddOperator;
use fenwick_tree::FenwickTree;
use heavy_light_decomposition::HeavyLightDecomposition;
use proconio::{fastout, input, read_value};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        mut p: [usize; n - 1],
    }
    p.insert(0, !0);

    let hld = HeavyLightDecomposition::from_parents(&p);
    let tour = hld.euler_tour().collect::<Vec<_>>();
    let mut ft = FenwickTree::<AddOperator<_>>::from_fn(n, |i| a[tour[i]]);

    for _ in 0..q {
        if read_value!(usize) == 0 {
            input! {
                v: usize,
                x: i64,
            }
            ft.add(hld.vertex_index(v), x);
        } else {
            input! {
                v: usize,
            }
            let (l, r) = hld.subtree_range(v);
            let sum = ft.fold(l..r);
            println!("{}", sum);
        }
    }
}
