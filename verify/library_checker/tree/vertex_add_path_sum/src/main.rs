// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum

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
        uv: [(usize, usize); n - 1],
    }

    let hld = HeavyLightDecomposition::new(&uv, n - 1);
    let tour = hld.euler_tour().collect::<Vec<_>>();
    let mut ft = FenwickTree::<AddOperator<_>>::from_fn(n, |i| a[tour[i]]);

    for _ in 0..q {
        if read_value!(usize) == 0 {
            input! {
                p: usize,
                x: i64,
            }
            ft.add(hld.vertex_index(p), x);
        } else {
            input! {
                s: usize,
                t: usize,
            }
            let mut sum = 0;
            hld.path_query(s, t, true, |l, r, _| {
                sum += ft.fold(l..r);
            });
            println!("{}", sum);
        }
    }
}
