// verification-helper: PROBLEM https://judge.yosupo.jp/problem/discrete_logarithm_mod

use ac_library::ModInt;
use algebraic::{act, algebra, monoid};
use discrete_logarithm::discrete_logarithm;
use proconio::input;

algebra!(M, ModInt);
monoid!(M, ModInt::new(0), |f, g| f * g);
act!(M, ModInt, |f, x| f * x);

#[proconio::fastout]
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
        ModInt::set_modulus(m);
        let x = ModInt::new(x);
        let y = ModInt::new(y);
        if let Some(k) = discrete_logarithm::<M>(ModInt::new(1), y, x, m as usize) {
            println!("{}", k);
        } else {
            println!("-1");
        }
    }
}
