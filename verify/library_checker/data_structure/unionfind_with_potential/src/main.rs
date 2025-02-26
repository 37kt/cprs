// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential

use algebraic_structure::magma::AddOperator;
use proconio::fastout;
use proconio::input;
use static_modint::ModInt998244353 as Mint;
use union_find::PotentializedUnionFind;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut uf = PotentializedUnionFind::<AddOperator<Mint>>::new(n);
    for _ in 0..q {
        input! {
            t: usize,
            u: usize,
            v: usize,
        }
        match t {
            0 => {
                input! {
                    x: Mint,
                }
                uf.merge(v, u, x);
                println!("{}", (uf.diff(v, u).unwrap() == x) as i32);
            }
            1 => {
                if let Some(x) = uf.diff(v, u) {
                    println!("{}", x);
                } else {
                    println!("{}", -1);
                }
            }
            _ => unreachable!(),
        }
    }
}
