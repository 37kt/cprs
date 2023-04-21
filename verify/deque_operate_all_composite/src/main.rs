// verification-helper: PROBLEM https://judge.yosupo.jp/problem/deque_operate_all_composite

use ac_library::ModInt998244353 as Mint;
use algebraic::{monoid, Monoid};
use proconio::input;
use sliding_window_aggregation::SlidingWindowAggregation;

monoid! {
    M,
    (Mint, Mint),
    (1.into(), 0.into()),
    |&(a, b), &(c, d)| (a * c, b * c + d)
}

#[proconio::fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut swag = SlidingWindowAggregation::<M>::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                f: (Mint, Mint),
            }
            swag.push_front(f);
        } else if t == 1 {
            input! {
                f: (Mint, Mint),
            }
            swag.push_back(f);
        } else if t == 2 {
            swag.pop_front();
        } else if t == 3 {
            swag.pop_back();
        } else {
            input! {
                x: Mint,
            }
            let (a, b) = swag.prod();
            println!("{}", a * x + b);
        }
    }
}
