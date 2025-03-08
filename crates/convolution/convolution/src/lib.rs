mod naive;
pub use naive::*;

mod ntt;
pub use ntt::*;

mod ntt_friendly;
pub use ntt_friendly::*;

mod arbitrary_mod;
pub use arbitrary_mod::*;

mod mod_2_64;
pub use mod_2_64::*;

pub(crate) const fn pow(x: u32, mut exp: u32, m: u32) -> u32 {
    let m = m as u64;
    let mut res = 1u64;
    let mut x = x as u64;
    while exp != 0 {
        if exp & 1 != 0 {
            res = res * x % m;
        }
        x = x * x % m;
        exp >>= 1;
    }
    res as u32
}

pub(crate) const fn inv(x: u32, m: u32) -> u32 {
    pow(x, m - 2, m)
}
