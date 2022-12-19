use std::cell::RefCell;

use super::modint::{ModInt, Modulus};

pub struct Combination<M: Modulus> {
    inv: RefCell<Vec<ModInt<M>>>,
    fact: RefCell<Vec<ModInt<M>>>,
    fact_inv: RefCell<Vec<ModInt<M>>>,
}

impl<M: Modulus> Combination<M> {
    pub fn new() -> Self {
        Self {
            inv: RefCell::new(vec![1.into(), 1.into()]),
            fact: RefCell::new(vec![1.into()]),
            fact_inv: RefCell::new(vec![1.into()]),
        }
    }

    pub fn inv(&self, n: usize) -> ModInt<M> {
        assert!(n > 0);
        let m = M::modulus() as usize;
        let mut inv = self.inv.borrow_mut();
        for i in inv.len()..=n {
            let x = (inv[m % i] * (m - m / i)).into();
            inv.push(x);
        }
        inv[n]
    }

    pub fn fact(&self, n: usize) -> ModInt<M> {
        let mut fact = self.fact.borrow_mut();
        for i in fact.len()..=n {
            let x = fact[i - 1] * i;
            fact.push(x);
        }
        fact[n]
    }

    pub fn fact_inv(&self, n: usize) -> ModInt<M> {
        let mut fact_inv = self.fact_inv.borrow_mut();
        for i in fact_inv.len()..=n {
            let x = fact_inv[i - 1] * self.inv(i);
            fact_inv.push(x);
        }
        fact_inv[n]
    }

    pub fn npk(&self, n: usize, k: usize) -> ModInt<M> {
        if n >= k {
            self.fact(n) * self.fact_inv(n - k)
        } else {
            0.into()
        }
    }

    pub fn nck(&self, n: usize, k: usize) -> ModInt<M> {
        if n >= k {
            self.fact(n) * self.fact_inv(k) * self.fact_inv(n - k)
        } else {
            0.into()
        }
    }

    pub fn nhk(&self, n: usize, k: usize) -> ModInt<M> {
        if k == 0 {
            1.into()
        } else {
            self.nck(n + k - 1, k)
        }
    }
}
