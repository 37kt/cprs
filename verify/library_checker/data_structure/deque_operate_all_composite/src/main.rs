// verification-helper: PROBLEM https://judge.yosupo.jp/problem/deque_operate_all_composite

use algebraic_structure::magma::Affine;
use algebraic_structure::magma::AffineOperator;
use foldable_deque::FoldableDeque;
use proconio::fastout;
use proconio::input;
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut dq = FoldableDeque::<AffineOperator<Mint>>::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            0 => {
                input! {
                    a: Mint,
                    b: Mint,
                }
                dq.push_front(Affine(a, b));
            }
            1 => {
                input! {
                    a: Mint,
                    b: Mint,
                }
                dq.push_back(Affine(a, b));
            }
            2 => {
                dq.pop_front();
            }
            3 => {
                dq.pop_back();
            }
            4 => {
                input! {
                    x: Mint,
                }
                let f = dq.fold_all();
                println!("{}", f.eval(x));
            }
            _ => unreachable!(),
        }
    }
}
