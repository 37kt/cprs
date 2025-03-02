// 必要になったら quo_rem 等を実装する

#[derive(Clone, Copy)]
pub struct BarrettReduction64 {
    m: u64,
    imh: u64,
    iml: u64,
}

impl BarrettReduction64 {
    pub fn new(m: u64) -> Self {
        let mut im = !0 / m as u128;
        if (im * m as u128).wrapping_add(m as u128) == 0 {
            im = im.wrapping_add(1);
        }
        Self {
            m,
            imh: (im >> 64) as u64,
            iml: im as u64,
        }
    }

    pub fn modulus(&self) -> u64 {
        self.m
    }

    pub fn mul(&self, a: u64, b: u64) -> u64 {
        const MASK: u128 = (1 << 64) - 1;
        let imh = self.imh as u128;
        let iml = self.iml as u128;
        let m = self.m as u128;
        let add = |x: u128, y: u128| -> u128 { x.wrapping_add(y) };
        let sub = |x: u128, y: u128| -> u128 { x.wrapping_sub(y) };
        let mul = |x: u128, y: u128| -> u128 { x.wrapping_mul(y) };
        let mut x = a as u128 * b as u128;
        let mut z = mul(x & MASK, iml);
        z = add(add(mul(x & MASK, imh), mul(x >> 64, iml)), z >> 64);
        z = add(mul(x >> 64, imh), z >> 64);
        x = sub(x, mul(z, m));
        if m <= x {
            x = sub(x, m);
        }
        x as u64
    }
}
