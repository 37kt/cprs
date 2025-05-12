use modint::ModInt;
use numeric_traits::Integer;
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
        let new_n = n.ceil_pow2();

        self.fact.resize(new_n + 1, M::from_raw(0));
        self.fact_inv.resize(new_n + 1, M::from_raw(0));
        self.inv.resize(new_n + 1, M::from_raw(0));

        for i in prev_n + 1..=new_n {
            self.fact[i] = self.fact[i - 1] * M::from_raw(i as _);
        }
        self.fact_inv[new_n] = self.fact[new_n].recip();
        self.inv[new_n] = self.fact_inv[new_n] * self.fact[new_n - 1];
        for i in (prev_n + 1..new_n).rev() {
            self.fact_inv[i] = self.fact_inv[i + 1] * M::from_raw((i + 1) as _);
            self.inv[i] = self.fact_inv[i] * self.fact[i - 1];
        }
    }

    pub fn fact(&mut self, n: usize) -> M {
        if n >= self.fact.len() {
            self.expand(n);
        }
        self.fact[n]
    }

    pub fn fact_inv(&mut self, n: usize) -> M {
        if n >= self.fact_inv.len() {
            self.expand(n);
        }
        self.fact_inv[n]
    }

    pub fn inv(&mut self, n: usize) -> M {
        assert!(n != 0);
        if n >= self.inv.len() {
            self.expand(n);
        }
        self.inv[n]
    }

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
