// verification-helper: PROBLEM https://judge.yosupo.jp/problem/aplusb

use numeric_traits::{Inf, NegInf};
use proconio::input;
use rand::Rng;
use rational::Rational;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    println!("{}", a + b);

    type Q1 = Rational<false>;
    const M: i64 = 1 << 60;
    let mut rng = rand::thread_rng();
    for _ in 0..200000 {
        let a = Q1::new(rng.gen_range(-M..=M), rng.gen_range(-M..=M));
        let b = Q1::new(rng.gen_range(-M..=M), rng.gen_range(-M..=M));
        let x: f64 = a.into();
        let y: f64 = b.into();
        assert_eq!(a.cmp(&b), x.partial_cmp(&y).unwrap());
    }

    type Q2 = Rational<true>;
    let mut rng = rand::thread_rng();
    for _ in 0..200000 {
        let a = Q2::new(rng.gen_range(-M..=M), rng.gen_range(-M..=M));
        let b = Q2::new(rng.gen_range(-M..=M), rng.gen_range(-M..=M));
        let x: f64 = a.into();
        let y: f64 = b.into();
        assert_eq!(a.cmp(&b), x.partial_cmp(&y).unwrap());
    }

    assert!(Q1::inf() > Q1::from_raw(10, 1));
    assert!(Q1::neg_inf() < Q1::from_raw(-10, 1));
    assert!(Q1::neg_inf() < Q1::inf());
    assert!((Q1::inf() + Q1::neg_inf()).is_nan());
    assert!((Q1::inf() - Q1::inf()).is_nan());
    assert!((Q1::neg_inf() - Q1::neg_inf()).is_nan());

    eprintln!("OK");
}
