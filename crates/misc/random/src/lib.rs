#[derive(Copy, Clone)]
pub struct Pcg64Fast(u128);

const R: f64 = 1.0 / 0xffff_ffff_ffff_ffff_u64 as f64;

impl Default for Pcg64Fast {
    fn default() -> Self {
        Self(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                | 1,
        )
    }
}

impl Pcg64Fast {
    #[inline]
    pub fn new(state: u128) -> Self {
        Self(state | 1)
    }

    #[inline]
    pub fn u32(&mut self) -> u32 {
        self.u64() as u32
    }

    #[inline(always)]
    pub fn u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(0x2360_ED05_1FC6_5DA4_4385_DF64_9FCC_F645);
        let rot = (self.0 >> 122) as u32;
        let xsl = (self.0 >> 64) as u64 ^ self.0 as u64;
        xsl.rotate_right(rot)
    }

    #[inline]
    pub fn f64(&mut self) -> f64 {
        self.u64() as f64 * R
    }
}
