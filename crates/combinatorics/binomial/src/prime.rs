use modint::ModInt;
use numeric_traits::Integer;
use prime_factorization::is_prime;

pub struct BinomialPrime<M: ModInt<Value = u32>> {
    fact: Vec<M>,
    fact_inv: Vec<M>,
    inv: Vec<M>,
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
        if prev_n >= n {
            return;
        }

        let new_n = n.ceil_pow2().min(M::modulus() as usize - 1);
        if prev_n >= new_n {
            return;
        }

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
        self.expand(n);
        if n >= self.fact.len() {
            M::from_raw(0)
        } else {
            self.fact[n]
        }
    }

    pub fn fact_inv(&mut self, n: usize) -> M {
        self.expand(n);
        assert!(n < self.fact_inv.len(), "n! is 0");
        self.fact_inv[n]
    }

    pub fn inv(&mut self, n: usize) -> M {
        self.expand(n);
        let n = n % M::modulus() as usize;
        assert!(n != 0, "n is multiple of modulus");
        self.inv[n]
    }

    pub fn nck(&mut self, mut n: usize, mut k: usize) -> M {
        if n < k {
            return M::from_raw(0);
        }

        let p = M::modulus() as usize;
        let mut res = M::from_raw(1);
        while n > 0 || k > 0 {
            res *= self.fact(n % p) * self.fact_inv(k % p) * self.fact_inv((n - k) % p);
            n /= p;
            k /= p;
        }
        res
    }
}
