use std::sync::atomic::{self, AtomicU32, AtomicU64};

pub(crate) struct BarrettReduction {
    m: AtomicU32,
    im: AtomicU64,
}

pub(crate) fn barrett_reduction<ID>() -> &'static BarrettReduction {
    static BARRETT_REDUCTION: BarrettReduction = BarrettReduction {
        m: AtomicU32::new(998_244_353),
        im: AtomicU64::new((!0u64 / 998_244_353).wrapping_add(1)),
    };
    &BARRETT_REDUCTION
}

impl BarrettReduction {
    pub(crate) fn set_modulus(&self, m: u32) {
        let im = (!0 / m as u64).wrapping_add(1);
        self.m.store(m, atomic::Ordering::SeqCst);
        self.im.store(im, atomic::Ordering::SeqCst);
    }

    #[inline]
    pub(crate) fn modulus(&self) -> u32 {
        self.m.load(atomic::Ordering::SeqCst)
    }

    #[inline]
    pub(crate) fn mul(&self, a: u32, b: u32) -> u32 {
        let m = self.m.load(atomic::Ordering::SeqCst);
        let im = self.im.load(atomic::Ordering::SeqCst);
        let mut z = a as u64;
        z *= b as u64;
        let x = (((z as u128) * (im as u128)) >> 64) as u64;
        let mut v = z.wrapping_sub(x.wrapping_mul(m as u64)) as u32;
        if m <= v {
            v = v.wrapping_add(m);
        }
        v
    }
}
