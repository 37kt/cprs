use std::cell::RefCell;

use modint::ModInt;

pub struct Combination<M: ModInt> {
    inv: RefCell<Vec<M>>,
    fact: RefCell<Vec<M>>,
    fact_inv: RefCell<Vec<M>>,
}

impl<M: ModInt> Combination<M> {
    pub fn new() -> Self {
        Self {
            inv: RefCell::new(vec![M::from(0), M::from(1)]),
            fact: RefCell::new(vec![M::from(1); 2]),
            fact_inv: RefCell::new(vec![M::from(1); 2]),
        }
    }

    fn expand(&self, n: usize) {
        let mut inv = self.inv.borrow_mut();
        let mut fact = self.fact.borrow_mut();
        let mut fact_inv = self.fact_inv.borrow_mut();
        let m = inv.len();
        let mut nn = m;
        while nn <= n {
            nn *= 2;
        }
        inv.resize(nn, M::default());
        fact.resize(nn, M::default());
        fact_inv.resize(nn, M::default());
        let p = M::modulus() as usize;
        for i in m..nn {
            inv[i] = -inv[p % i] * M::from((p / i) as u32);
            fact[i] = fact[i - 1] * M::from(i);
            fact_inv[i] = fact_inv[i - 1] * inv[i];
        }
    }

    pub fn inv(&self, n: usize) -> M {
        self.expand(n);
        self.inv.borrow()[n]
    }

    pub fn fact(&self, n: usize) -> M {
        self.expand(n);
        self.fact.borrow()[n]
    }

    pub fn fact_inv(&self, n: usize) -> M {
        self.expand(n);
        self.fact_inv.borrow()[n]
    }

    pub fn nck(&self, n: usize, k: usize) -> M {
        if n < k {
            M::from(0)
        } else {
            self.expand(n);
            self.fact.borrow()[n] * self.fact_inv.borrow()[k] * self.fact_inv.borrow()[n - k]
        }
    }

    pub fn npk(&self, n: usize, k: usize) -> M {
        if n < k {
            M::from(0)
        } else {
            self.expand(n);
            self.fact.borrow()[n] * self.fact_inv.borrow()[n - k]
        }
    }

    pub fn nhk(&self, n: usize, k: usize) -> M {
        if n == 0 && k == 0 {
            M::from(1)
        } else {
            self.nck(n + k - 1, k)
        }
    }

    pub fn catalan(&self, n: usize) -> M {
        self.expand(n * 2);
        self.fact.borrow()[n * 2] * self.fact_inv.borrow()[n + 1] * self.fact_inv.borrow()[n]
    }
}
