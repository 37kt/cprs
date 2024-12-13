use std::cell::RefCell;

use combination::Combination;
use modint::ModInt;

pub struct QBinomial<M: ModInt> {
    q: M,
    num: RefCell<Vec<M>>,
    inv: RefCell<Vec<M>>,
    fact: RefCell<Vec<M>>,
    fact_inv: RefCell<Vec<M>>,
    comb: Combination<M>,
}

impl<M: ModInt> QBinomial<M> {
    pub fn new(q: M) -> Self {
        Self {
            q,
            num: RefCell::new(vec![M::from(0), M::from(1)]),
            inv: RefCell::new(vec![M::from(0), M::from(1)]),
            fact: RefCell::new(vec![M::from(1); 2]),
            fact_inv: RefCell::new(vec![M::from(1); 2]),
            comb: Combination::new(),
        }
    }

    fn expand(&self, n: usize) {
        let mut num = self.num.borrow_mut();
        let mut inv = self.inv.borrow_mut();
        let mut fact = self.fact.borrow_mut();
        let mut fact_inv = self.fact_inv.borrow_mut();
        let m = inv.len();
        let mut nn = m;
        while nn <= n {
            nn *= 2;
        }
        num.reserve(nn - m);
        for i in m..nn {
            let x = num[i - 1] * self.q + M::from(1);
            if x.val() == 0 {
                break;
            }
            num.push(x);
        }
        let nn = num.len();
        inv.resize(nn, M::default());
        fact.resize(nn, M::default());
        fact_inv.resize(nn, M::default());
        for i in m..nn {
            fact[i] = fact[i - 1] * num[i];
        }
        fact_inv[nn - 1] = fact[nn - 1].inv();
        inv[nn - 1] = fact_inv[nn - 1] * fact[nn - 2];
        for i in (m..nn - 1).rev() {
            fact_inv[i] = fact_inv[i + 1] * num[i + 1];
            inv[i] = fact_inv[i] * fact[i - 1];
        }
    }

    pub fn q(&self) -> M {
        self.q
    }

    pub fn num(&self, n: usize) -> M {
        self.expand(n);
        let num = self.num.borrow();
        num[n % num.len()]
    }

    pub fn inv(&self, n: usize) -> M {
        self.expand(n);
        let inv = self.inv.borrow();
        inv[n % inv.len()]
    }

    pub fn fact(&self, n: usize) -> M {
        self.expand(n);
        let fact = self.fact.borrow();
        if n < fact.len() {
            fact[n]
        } else {
            0.into()
        }
    }

    pub fn fact_inv(&self, n: usize) -> M {
        self.expand(n);
        let fact_inv = self.fact_inv.borrow();
        if n < fact_inv.len() {
            fact_inv[n]
        } else {
            0.into()
        }
    }

    pub fn binom(&self, n: usize, k: usize) -> M {
        if n < k {
            return 0.into();
        }
        self.expand(n);
        let m = self.num.borrow().len();
        if n < m {
            return self.fact(n) * self.fact_inv(k) * self.fact_inv(n - k);
        }
        self.comb.nck(n / m, k / m) * self.binom(n % m, k % m)
    }
}
