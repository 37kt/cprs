use numeric_traits::Integer;

pub struct Eratosthenes {
    n: usize,
    fs: Vec<u8>,
}

impl Eratosthenes {
    pub fn new(n: usize) -> Self {
        if n == 0 {
            return Self { n: 0, fs: vec![] };
        }

        let sz = n.ceil_div(30);
        let mut fs = vec![0xff_u8; sz];
        match n % 30 {
            29.. => fs[sz - 1] = 0xff,
            23.. => fs[sz - 1] = 0x7f,
            19.. => fs[sz - 1] = 0x3f,
            17.. => fs[sz - 1] = 0x1f,
            13.. => fs[sz - 1] = 0x0f,
            11.. => fs[sz - 1] = 0x07,
            7.. => fs[sz - 1] = 0x03,
            1.. => fs[sz - 1] = 0x01,
            _ => {}
        }
        fs[0] &= !1;

        'a: for i0 in 0..sz {
            let mut f = fs[i0];
            while f != 0 {
                let i1 = f.lsb_index();
                let x1 = MOD_30[i1];
                if (i0 * 30 + x1).pow(2) > n {
                    break 'a;
                }
                let mut j0 = i0 * (30 * i0 + 2 * x1) + (x1 * x1) / 30;
                let mut j1 = i1;
                while j0 < sz {
                    fs[j0] &= MUL_MASK[i1][j1];
                    j0 += i0 * D1[j1] + D2[i1][j1];
                    j1 = (j1 + 1) % 8;
                }
                f &= f - 1;
            }
        }

        Self { n, fs }
    }

    pub fn primes(&self) -> impl Iterator<Item = usize> + '_ {
        [2, 3, 5]
            .into_iter()
            .filter(|&p| p <= self.n)
            .chain(self.fs.iter().enumerate().flat_map(|(i, &f)| {
                let mut f = f;
                std::iter::from_fn(move || {
                    (f != 0).then(|| {
                        let j = f.lsb_index();
                        f &= f - 1;
                        i * 30 + MOD_30[j as usize]
                    })
                })
            }))
    }

    pub fn is_prime(&self, x: usize) -> bool {
        assert!(x <= self.n);
        match x {
            2 | 3 | 5 => true,
            _ => Self::id(x).map_or(false, |i| self.fs[i / 30] >> (i % 30) & 1 != 0),
        }
    }

    fn id(x: usize) -> Option<usize> {
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
        Some(res)
    }
}

const MOD_30: [usize; 8] = [1, 7, 11, 13, 17, 19, 23, 29];
const D1: [usize; 8] = [6, 4, 2, 4, 2, 4, 6, 2];
const D2: [[usize; 8]; 8] = init_d2();
const MUL_MASK: [[u8; 8]; 8] = init_mul_mask();

const fn init_mul_mask() -> [[u8; 8]; 8] {
    let mut mul = [[0; 8]; 8];
    let mut i = 0;
    while i < 8 {
        let mut j = 0;
        while j < 8 {
            let x = MOD_30[i] * MOD_30[j] % 30;
            let k = match x {
                1 => 0,
                7 => 1,
                11 => 2,
                13 => 3,
                17 => 4,
                19 => 5,
                23 => 6,
                29 => 7,
                _ => unreachable!(),
            };
            mul[i][j] = !(1 << k);
            j += 1;
        }
        i += 1;
    }
    mul
}

const fn init_d2() -> [[usize; 8]; 8] {
    let mut d2 = [[0; 8]; 8];
    let mut i = 0;
    while i < 8 {
        let mut j = 0;
        while j < 8 {
            let x = MOD_30[i] * (MOD_30[j] + D1[j]) / 30;
            let y = MOD_30[i] * MOD_30[j] / 30;
            d2[i][j] = x - y;
            j += 1;
        }
        i += 1;
    }
    d2
}
