// verification-helper: PROBLEM https://judge.yosupo.jp/problem/discrete_logarithm_mod

use ac_library::ModInt;
use discrete_logarithm::discrete_logarithm;
use proconio::input;

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
        if let Some(k) =
            discrete_logarithm(ModInt::new(1), y, x, |f, x| f * x, |f, g| f * g, m as usize)
        {
            println!("{}", k);
        } else {
            println!("-1");
        }
    }
}
