#[derive(Clone, Copy)]
pub struct BarrettReduction32 {
    m: u32,
    im: u64,
}

impl BarrettReduction32 {
    pub fn new(m: u32) -> Self {
        let im = (!0 / m as u64).wrapping_add(1);
        Self { m, im }
    }

    pub fn modulus(&self) -> u32 {
        self.m
    }

    pub fn quo_rem(&self, a: u64) -> (u64, u32) {
        let mut x = (((a as u128) * (self.im as u128)) >> 64) as u64;
        let mut r = a.wrapping_sub(x.wrapping_mul(self.m as u64)) as u32;
        if self.m <= r {
            r = r.wrapping_add(self.m);
            x -= 1;
        }
        (x, r)
    }

    pub fn quo(&self, a: u64) -> u64 {
        self.quo_rem(a).0
    }

    pub fn rem(&self, a: u64) -> u32 {
        self.quo_rem(a).1
    }

    pub fn mul(&self, a: u32, b: u32) -> u32 {
        self.rem(a as u64 * b as u64)
    }

    pub fn pow(&self, a: u32, mut exp: u64) -> u32 {
        let mut a = self.rem(a as u64);
        let mut r = if self.m == 1 { 0 } else { 1 };
        while exp != 0 {
            if exp & 1 != 0 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            exp >>= 1;
        }
        r
    }
}
