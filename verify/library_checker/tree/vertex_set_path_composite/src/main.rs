// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite

use algebraic_structure::magma::{Affine, AffineOperator};
use algebraic_traits::{Magma, Unital};
use heavy_light_decomposition::HeavyLightDecomposition;
use proconio::{fastout, input};
use segment_tree::SegmentTree;
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(Mint, Mint); n],
        uv: [(usize, usize); n - 1],
    }
    let (a, b): (Vec<_>, Vec<_>) = ab.into_iter().unzip();
    let hld = HeavyLightDecomposition::new(&uv, 0);
    let tour = hld.euler_tour().collect::<Vec<_>>();
    let mut seg = SegmentTree::<AffineOperator<_>>::from_fn(n, |i| Affine(a[tour[i]], b[tour[i]]));
    let mut ges = SegmentTree::<AffineOperator<_>>::from_fn(n, |i| {
        Affine(a[tour[n - 1 - i]], b[tour[n - 1 - i]])
    });
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                p: usize,
                a: Mint,
                b: Mint,
            }
            let i = hld.vertex_index(p);
            seg.set(i, Affine(a, b));
            ges.set(n - 1 - i, Affine(a, b));
        } else {
            input! {
                s: usize,
                t: usize,
                x: Mint,
            }
            let mut f = AffineOperator::unit();
            hld.path_query(s, t, true, |l, r, rev| {
                let g = if rev {
                    ges.fold(n - r..n - l)
                } else {
                    seg.fold(l..r)
                };
                f = AffineOperator::op(&f, &g);
            });
            let res = f.eval(x);
            println!("{}", res);
        }
    }
}
