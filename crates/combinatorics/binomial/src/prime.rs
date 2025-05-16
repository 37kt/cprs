use modint::ModInt;
use prime_factorization::is_prime;

pub struct BinomialPrime<M: ModInt<Value = u32>> {
    fact: Vec<M>,
    fact_inv: Vec<M>,
    inv: Vec<M>,
}

impl<M: ModInt<Value = u32>> Default for BinomialPrime<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: ModInt<Value = u32>> BinomialPrime<M> {
    pub fn new() -> Self {
        assert!(is_prime(M::modulus()));

        Self {
            fact: vec![M::from_raw(1); 2],
            fact_inv: vec![M::from_raw(1); 2],
            inv: vec![M::from_raw(1); 2],
        }
    }

    pub fn expand(&mut self, n: usize) {
        let prev_n = self.fact.len() - 1;
        let new_n = n.max(prev_n * 2).min(M::modulus() as usize - 1);
        if prev_n > new_n {
            return;
        }

        let p = M::modulus() as usize;
        self.fact.reserve(new_n + 1);
        self.fact_inv.reserve(new_n + 1);
        self.inv.reserve(new_n + 1);
        for i in prev_n + 1..=new_n {
            self.fact.push(self.fact[i - 1] * M::from_raw(i as _));
            self.inv
                .push(self.inv[p % i] * M::from_raw((p - p / i) as _));
            self.fact_inv.push(self.fact_inv[i - 1] * self.inv[i]);
        }
    }

    #[inline]
    pub fn fact(&mut self, n: usize) -> M {
        if n >= self.fact.len() {
            self.expand(n);
        }
        self.fact[n]
    }

    #[inline]
    pub fn fact_inv(&mut self, n: usize) -> M {
        if n >= self.fact_inv.len() {
            self.expand(n);
        }
        self.fact_inv[n]
    }

    #[inline]
    pub fn inv(&mut self, n: usize) -> M {
        assert!(n != 0);
        if n >= self.inv.len() {
            self.expand(n);
        }
        self.inv[n]
    }

    #[inline]
    pub fn nck(&mut self, n: usize, k: usize) -> M {
        if n < k {
            return M::from_raw(0);
        }
        if n >= self.fact.len() {
            self.expand(n);
        }
        self.fact[n] * self.fact_inv[k] * self.fact_inv[n - k]
    }
}
