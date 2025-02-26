pub trait Inf: PartialOrd {
    fn inf() -> Self;
}

pub trait NegInf: PartialOrd {
    fn neg_inf() -> Self;
}

macro_rules! impl_inf {
    ($($t:ty, $inf:expr,)*) => {
        $(
            impl Inf for $t {
                fn inf() -> Self {
                    $inf
                }
            }
        )*
    }
}

macro_rules! impl_neg_inf {
    ($($t:ty, $neg_inf:expr,)*) => {
        $(
            impl NegInf for $t {
                fn neg_inf() -> Self {
                    $neg_inf
                }
            }
        )*
    }
}

impl_inf! {
    i32, 1_001_000_000,
    u32, 1_001_000_000,
    f32, 1e10,
    i64, 1_001_000_000_000_000_000,
    isize, 1_001_000_000_000_000_000,
    u64, 1_001_000_000_000_000_000,
    usize, 1_001_000_000_000_000_000,
    f64, 1e20,
    i128, 1_001_000_000_000_000_000_000_000_000_000_000_000,
    u128, 1_001_000_000_000_000_000_000_000_000_000_000_000,
}

impl_neg_inf! {
    i32, -1_001_000_000,
    u32, 0,
    f32, -1e10,
    i64, -1_001_000_000_000_000_000,
    isize, -1_001_000_000_000_000_000,
    u64, 0,
    usize, 0,
    f64, -1e20,
    i128, -1_001_000_000_000_000_000_000_000_000_000_000_000,
    u128, 0,
}
