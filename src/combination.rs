use super::modint::{ModInt, Modulus};

pub struct Combination<M: Modulus> {
    fact: Vec<ModInt<M>>,
    fact_inv: Vec<ModInt<M>>,
}

impl<M: Modulus> Combination<M> {
    pub fn new(n: usize) -> Self {
        let mut fact: Vec<ModInt<M>> = vec![1.into(); n + 1];
        let mut fact_inv: Vec<ModInt<M>> = vec![1.into(); n + 1];
        for i in 1..=n {
            fact[i] = fact[i - 1] * i;
        }
        fact_inv[n] = fact[n].inv();
        for i in (1..=n).rev() {
            fact_inv[i - 1] = fact_inv[i] * i;
        }
        Self { fact, fact_inv }
    }

    pub fn fact(&self, n: usize) -> ModInt<M> {
        assert!(n < self.fact.len());
        self.fact[n]
    }

    pub fn fact_inv(&self, n: usize) -> ModInt<M> {
        assert!(n < self.fact_inv.len());
        self.fact_inv[n]
    }

    pub fn npk(&self, n: usize, k: usize) -> ModInt<M> {
        if n >= k {
            self.fact[n] * self.fact_inv[n - k]
        } else {
            0.into()
        }
    }

    pub fn nck(&self, n: usize, k: usize) -> ModInt<M> {
        if n >= k {
            self.fact[n] * self.fact_inv[k] * self.fact_inv[n - k]
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
