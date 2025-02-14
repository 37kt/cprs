// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rational_approximation

use proconio::input;
use rational::Rational;
use stern_brocot_tree::SternBrocotTreeNode;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: i64,
            x: i64,
            y: i64,
        }
        let r = Rational::new(x, y);
        let (Rational { num: a, den: b }, _) = SternBrocotTreeNode::binary_search(|x| x <= r, n);
        let (_, Rational { num: c, den: d }) = SternBrocotTreeNode::binary_search(|x| x >= r, n);
        println!("{} {} {} {}", a, b, c, d);
    }
}
