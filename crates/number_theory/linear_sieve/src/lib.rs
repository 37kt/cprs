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
                _ => 1,
            };
        let n = n as _;

        let mut ps = vec![2, 3, 5];
        ps.retain(|&p| p <= n as _);

        let mut lpf = vec![1; sz];
        for i in 1..sz {
            let x = Self::x(i);
            if lpf[i] == 1 {
                lpf[i] = x;
                ps.push(x as _);
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
        assert!(x <= self.n as _);
        match (x, Self::id(x as _)) {
            (2 | 3 | 5, _) => true,
            (_, Some(i)) => self.lpf[i] == x as _,
            _ => false,
        }
    }

    pub fn primes(
        &self,
    ) -> impl Iterator<Item = usize> + FusedIterator + ExactSizeIterator + DoubleEndedIterator + '_
    {
        self.ps.iter().copied().map(|p| p as _)
    }

    pub fn least_factor(&self, x: usize) -> Option<usize> {
        assert!(x <= self.n as _);
        match x {
            ..=1 => None,
            _ if x % 2 == 0 => Some(2),
            _ if x % 3 == 0 => Some(3),
            _ if x % 5 == 0 => Some(5),
            _ => Some(self.lpf[Self::id(x as _).unwrap()] as _),
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
        match x % 30 {
            1 => Some(0),
            7 => Some(1),
            11 => Some(2),
            13 => Some(3),
            17 => Some(4),
            19 => Some(5),
            23 => Some(6),
            29 => Some(7),
            _ => None,
        }
        .map(|i| (i + offset) as _)
    }

    fn x(id: usize) -> u32 {
        const CANDS: [u32; 8] = [1, 7, 11, 13, 17, 19, 23, 29];
        id as u32 / 8 * 30 + CANDS[id % 8]
    }
}
