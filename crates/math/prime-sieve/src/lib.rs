pub struct PrimeSieve {
    primes: Vec<usize>,
    div: Vec<usize>,
}

impl PrimeSieve {
    pub fn new(n: usize) -> Self {
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
        for i in 0..=n {
            if div[i] == i {
                primes.push(i);
            }
        }
        Self { primes, div }
    }

    pub fn is_prime(&self, x: usize) -> bool {
        let n = self.primes.len() - 1;
        assert!(n * n <= x);
        if x <= n {
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

    pub fn factorize(&self, x: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::<(usize, usize)>::new();
        let n = self.primes.len() - 1;
        assert!(n * n <= x);
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
        res.sort();
        res
    }

    pub fn primes(&self) -> Vec<usize> {
        self.primes.clone()
    }
}
