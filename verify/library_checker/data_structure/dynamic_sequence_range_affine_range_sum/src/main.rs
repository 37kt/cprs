// verification-helper: PROBLEM https://judge.yosupo.jp/problem/dynamic_sequence_range_affine_range_sum

use algebraic_structure::act::CountsumAffineOperator;
use algebraic_structure::magma::{Affine, CountSum};
use proconio::fastout;
use proconio::input;
use splay_tree::{FoldAct, SplaySequence};
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Mint; n],
    }

    let mut seq =
        SplaySequence::<FoldAct<CountsumAffineOperator<Mint>>>::from_fn(n, |i| CountSum {
            count: 1.into(),
            sum: a[i],
        });

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            0 => {
                input! {
                    i: usize,
                    x: Mint,
                }
                seq.insert(
                    i,
                    CountSum {
                        count: 1.into(),
                        sum: x,
                    },
                );
            }
            1 => {
                input! {
                    i: usize,
                }
                seq.remove(i);
            }
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }
                seq.reverse(l..r);
            }
            3 => {
                input! {
                    l: usize,
                    r: usize,
                    a: Mint,
                    b: Mint,
                }
                seq.apply(l..r, Affine(a, b));
            }
            4 => {
                input! {
                    l: usize,
                    r: usize,
                }
                println!("{}", seq.fold(l..r).sum);
            }
            _ => unreachable!(),
        }
    }
}
