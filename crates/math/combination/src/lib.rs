use ac_library::modint::ModIntBase;

pub struct Combination<M: ModIntBase> {
    inv: Vec<M>,
    fact: Vec<M>,
    fact_inv: Vec<M>,
}

impl<M: ModIntBase> Combination<M> {
    pub fn new() -> Self {
        Self {
            inv: vec![M::new(0), M::new(1)],
            fact: vec![M::new(1); 2],
            fact_inv: vec![M::new(1); 2],
        }
    }

    fn expand(&mut self, n: usize) {
        let m = self.fact.len();
        if n < m {
            return;
        }
        self.inv.resize(n + 1, M::default());
        self.fact.resize(n + 1, M::default());
        self.fact_inv.resize(n + 1, M::default());
        let p = M::modulus() as usize;
        for i in m..=n {
            self.inv[i] = -self.inv[p % i] * M::new((p / i) as u32);
            self.fact[i] = self.fact[i - 1] * M::new(i);
            self.fact_inv[i] = self.fact_inv[i - 1] * self.inv[i];
        }
    }

    pub fn inv(&mut self, n: usize) -> M {
        self.expand(n);
        self.inv[n]
    }

    pub fn fact(&mut self, n: usize) -> M {
        self.expand(n);
        self.fact[n]
    }

    pub fn fact_inv(&mut self, n: usize) -> M {
        self.expand(n);
        self.fact_inv[n]
    }

    pub fn nck(&mut self, n: usize, k: usize) -> M {
        if n < k {
            M::new(0)
        } else {
            self.expand(n);
            self.fact[n] * self.fact_inv[k] * self.fact_inv[n - k]
        }
    }

    pub fn npk(&mut self, n: usize, k: usize) -> M {
        if n < k {
            M::new(0)
        } else {
            self.expand(n);
            self.fact[n] * self.fact_inv[n - k]
        }
    }

    pub fn nhk(&mut self, n: usize, k: usize) -> M {
        if n == 0 && k == 0 {
            M::new(1)
        } else {
            self.nck(n + k - 1, k)
        }
    }

    pub fn catalan(&mut self, n: usize) -> M {
        self.expand(n * 2);
        self.fact[n * 2] * self.fact_inv[n + 1] * self.fact_inv[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ac_library::ModInt998244353 as Mint;

    #[test]
    fn catalan() {
        let mut comb = Combination::<Mint>::new();
        let mut a = vec![];
        for n in 0..8 {
            a.push(comb.catalan(n).val());
        }
        assert_eq!(&a, &[1, 1, 2, 5, 14, 42, 132, 429]);
    }
}
