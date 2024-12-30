// verification-helper: PROBLEM https://judge.yosupo.jp/problem/discrete_logarithm_mod

use discrete_logarithm::discrete_logarithm;
use modint::DynamicModInt as Mint;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            x: u32,
            y: u32,
            m: u32,
        }
        Mint::set_modulus(m);
        let x = Mint::new(x);
        let y = Mint::new(y);
        if let Some(k) =
            discrete_logarithm(Mint::new(1), y, x, |&f, &x| f * x, |&f, &g| f * g, m as _)
        {
            println!("{}", k);
        } else {
            println!("-1");
        }
    }
}
