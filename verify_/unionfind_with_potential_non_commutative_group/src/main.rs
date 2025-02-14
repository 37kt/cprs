// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group

use std::ops::{Add, Neg};

use modint::ModInt998244353 as Mint;
use potentialized_union_find::PotentializedUnionFind;
use proconio::input;

#[derive(Clone, Copy, Eq, PartialEq)]
struct T([[Mint; 2]; 2]);

impl Add for T {
    type Output = T;
    fn add(self, rhs: Self) -> Self::Output {
        T([
            [
                self.0[0][0] * rhs.0[0][0] + self.0[0][1] * rhs.0[1][0],
                self.0[0][0] * rhs.0[0][1] + self.0[0][1] * rhs.0[1][1],
            ],
            [
                self.0[1][0] * rhs.0[0][0] + self.0[1][1] * rhs.0[1][0],
                self.0[1][0] * rhs.0[0][1] + self.0[1][1] * rhs.0[1][1],
            ],
        ])
    }
}

impl Neg for T {
    type Output = T;
    fn neg(self) -> Self::Output {
        T([[self.0[1][1], -self.0[0][1]], [-self.0[1][0], self.0[0][0]]])
    }
}

impl Default for T {
    fn default() -> Self {
        T([[Mint::new(1), Mint::new(0)], [Mint::new(0), Mint::new(1)]])
    }
}

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut uf = PotentializedUnionFind::<T>::new(n);
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                u: usize,
                v: usize,
                x: [[Mint; 2]; 2],
            }
            let valid = uf.merge(u, v, T([[x[0][0], x[0][1]], [x[1][0], x[1][1]]]));
            println!("{}", valid as i32);
        } else {
            input! {
                u: usize,
                v: usize,
            }
            if let Some(T(diff)) = uf.diff(u, v) {
                println!(
                    "{} {} {} {}",
                    diff[0][0], diff[0][1], diff[1][0], diff[1][1]
                );
            } else {
                println!("-1");
            }
        }
    }
}
