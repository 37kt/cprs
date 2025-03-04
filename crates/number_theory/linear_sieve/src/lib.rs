use std::iter::FusedIterator;

pub struct LinearSieve {
    n: u32,
    ps: Vec<u32>,
    lpf: Vec<u32>,
}

impl LinearSieve {
    pub fn new(n: usize) -> Self {
        let sz = n / 30 * 8
            + match n % 30 {
                29.. => 8,
                23.. => 7,
                19.. => 6,
                17.. => 5,
                13.. => 4,
                11.. => 3,
                7.. => 2,
                1.. => 1,
                _ => 0,
            };
        let n = n as u32;

        let mut ps = vec![2, 3, 5];
        ps.retain(|&p| p <= n);

        let mut lpf = vec![1; sz];
        for i in 1..sz {
            let x = Self::x(i);
            if lpf[i] == 1 {
                lpf[i] = x;
                ps.push(x);
            }
            let lpf_i = lpf[i];
            for &y in ps.iter().skip(3).take_while(|&&y| y <= lpf_i && x * y <= n) {
                let j = Self::id(x * y).unwrap();
                lpf[j] = y;
            }
        }
        Self { n, ps, lpf }
    }

    pub fn is_prime(&self, x: usize) -> bool {
        let x = x as u32;
        assert!(x <= self.n);
        match x {
            2 | 3 | 5 => true,
            _ => Self::id(x).map_or(false, |i| self.lpf[i] == x),
        }
    }

    pub fn primes(
        &self,
    ) -> impl Iterator<Item = usize> + FusedIterator + ExactSizeIterator + DoubleEndedIterator + '_
    {
        self.ps.iter().copied().map(|p| p as _)
    }

    pub fn least_factor(&self, x: usize) -> Option<usize> {
        let x = x as u32;
        assert!(x <= self.n);
        match x {
            ..=1 => None,
            _ if x % 2 == 0 => Some(2),
            _ if x % 3 == 0 => Some(3),
            _ if x % 5 == 0 => Some(5),
            _ => Some(self.lpf[Self::id(x).unwrap()] as usize),
        }
    }

    pub fn factors(&self, mut x: usize) -> impl Iterator<Item = usize> + '_ {
        std::iter::from_fn(move || {
            let p = self.least_factor(x)?;
            x /= p;
            Some(p)
        })
    }

    fn id(x: u32) -> Option<usize> {
        if x <= 6 {
            return None;
        }
        let offset = x / 30 * 8;
        let res = match x % 30 {
            1 => 0,
            7 => 1,
            11 => 2,
            13 => 3,
            17 => 4,
            19 => 5,
            23 => 6,
            29 => 7,
            _ => return None,
        } + offset;
        Some(res as usize)
    }

    fn x(id: usize) -> u32 {
        const CANDS: [u32; 8] = [1, 7, 11, 13, 17, 19, 23, 29];
        id as u32 / 8 * 30 + CANDS[id % 8]
    }
}
