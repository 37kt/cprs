use std::cell::Cell;

#[derive(Clone, Copy)]
pub(crate) struct BarrettReduction {
    m: u32,
    im: u64,
}

pub(crate) fn barrett_reduction<Id, Ret>(f: impl FnOnce(&Cell<BarrettReduction>) -> Ret) -> Ret {
    thread_local! {
        static BARRETT_REDUCTION: Cell<BarrettReduction> = Cell::new(BarrettReduction::new(1_000_000_009));
    }

    BARRETT_REDUCTION.with(|br| f(br))
}

impl BarrettReduction {
    pub(crate) fn new(m: u32) -> Self {
        let im = (!0 / m as u64).wrapping_add(1);
        Self { m, im }
    }

    pub(crate) fn modulus(&self) -> u32 {
        self.m
    }

    pub(crate) fn mul(&self, a: u32, b: u32) -> u32 {
        let mut z = a as u64;
        z *= b as u64;
        let x = (((z as u128) * (self.im as u128)) >> 64) as u64;
        let mut v = z.wrapping_sub(x.wrapping_mul(self.m as u64)) as u32;
        if self.m <= v {
            v = v.wrapping_add(self.m);
        }
        v
    }
}
