pub struct PrimeSieve {
    primes: Vec<usize>,
    div: Vec<usize>,
}

/// エラトステネスの篩 (osa_k 法)
impl PrimeSieve {
    /// n 以下の正整数について最小の素因数を計算
    pub fn new(n: usize) -> Self {
        let n = n.max(2);
        let mut div = (0..=n).collect::<Vec<_>>();
        div[1] = 0;
        for i in 2..=n {
            if div[i] != i {
                continue;
            }
            for j in (i * 2..=n).step_by(i) {
                if div[j] != j {
                    continue;
                }
                div[j] = i;
            }
        }
        let mut primes = vec![];
        for i in 2..=n {
            if div[i] == i {
                primes.push(i);
            }
        }
        Self { primes, div }
    }

    /// 素数判定  
    ///
    /// # 計算量
    ///  - O(1)  (x <= n)
    ///  - O(φ(√x)) (x > n)
    pub fn is_prime(&self, x: usize) -> bool {
        let n = self.primes.len() - 1;
        assert!(x <= n * n);
        if x == 0 {
            false
        } else if x <= n {
            self.div[x] == x
        } else {
            for &p in &self.primes {
                if p * p > x {
                    break;
                }
                if x % p == 0 {
                    return false;
                }
            }
            true
        }
    }

    /// 素因数分解  
    /// (素因数, 指数) のペアを昇順に列挙
    pub fn factorize(&self, x: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::<(usize, usize)>::new();
        let n = self.div.len() - 1;
        assert!(x <= n * n);
        if x <= n {
            let mut y = x;
            while y > 1 {
                if res.len() == 0 || res.last().unwrap().0 != self.div[y] {
                    res.push((self.div[y], 1));
                } else {
                    res.last_mut().unwrap().1 += 1;
                }
                y /= self.div[y];
            }
        } else {
            let mut y = x;
            for &p in &self.primes {
                if y % p == 0 {
                    res.push((p, 0));
                    while y % p == 0 {
                        res.last_mut().unwrap().1 += 1;
                        y /= p;
                    }
                }
            }
            if y > 1 {
                res.push((y, 1));
            }
        }
        res
    }

    /// 約数を昇順に列挙
    pub fn divisors(&self, x: usize) -> Vec<usize> {
        let mut res = vec![1];
        for (p, k) in self.factorize(x) {
            for i in 0..res.len() {
                let mut t = res[i];
                for _ in 0..k {
                    t *= p;
                    res.push(t);
                }
            }
        }
        res.sort_unstable();
        res
    }

    /// n 以下の素数を昇順に列挙
    pub fn primes(&self) -> Vec<usize> {
        self.primes.clone()
    }

    /// x の最小の素因数
    pub fn min_factor(&self, x: usize) -> usize {
        self.div[x]
    }
}
