// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group

use algebraic_traits::define_algebra;
use proconio::fastout;
use proconio::input;
use static_modint::ModInt998244353 as Mint;
use union_find::PotentializedUnionFind;

define_algebra! {
    name: M,
    value: [[Mint; 2]; 2],
    op: |x: &[[Mint; 2]; 2], y: &[[Mint; 2]; 2]| {
        [
            [
                x[0][0] * y[0][0] + x[0][1] * y[1][0],
                x[0][0] * y[0][1] + x[0][1] * y[1][1],
            ],
            [
                x[1][0] * y[0][0] + x[1][1] * y[1][0],
                x[1][0] * y[0][1] + x[1][1] * y[1][1],
            ],
        ]
    },
    unit: [[Mint::new(1), Mint::new(0)], [Mint::new(0), Mint::new(1)]],
    inv: |x: &[[Mint; 2]; 2]| {
        [[x[1][1], -x[0][1]], [-x[1][0], x[0][0]]]
    },
    associative,
    commutative,
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut uf = PotentializedUnionFind::<M>::new(n);
    for _ in 0..q {
        input! {
            t: usize,
            u: usize,
            v: usize,
        }
        if t == 0 {
            input! {
                x: [[Mint; 2]; 2],
            }
            let x = [[x[0][0], x[0][1]], [x[1][0], x[1][1]]];
            uf.merge(v, u, x);
            let f = uf.diff(v, u).unwrap() == x;
            println!("{}", f as i32);
        } else {
            if let Some(d) = uf.diff(v, u) {
                println!("{} {} {} {}", d[0][0], d[0][1], d[1][0], d[1][1]);
            } else {
                println!("-1");
            }
        }
    }
}
