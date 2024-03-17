pub trait Algebra {
    type S;
}

pub trait Act: Algebra {
    type X;
    fn act(f: &Self::S, x: &Self::X) -> Self::X;
}

pub trait Monoid: Algebra {
    fn e() -> Self::S;
    fn op(x: &Self::S, y: &Self::S) -> Self::S;
}

pub trait Group: Monoid {
    fn inv(x: &Self::S) -> Self::S;
}

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

#[macro_export]
macro_rules! algebra {
    ($ident:ident, $ty:ty) => {
        #[derive(Clone)]
        enum $ident {}
        impl $crate::Algebra for $ident {
            type S = $ty;
        }
    };
}

#[macro_export]
macro_rules! act {
    ($ident:ident, $tar:ty, $act:expr) => {
        impl $crate::Act for $ident {
            type X = $tar;
            #[inline]
            fn act(f: &Self::S, x: &Self::X) -> Self::X {
                $act(f, x)
            }
        }
    };
}

#[macro_export]
macro_rules! monoid {
    ($ident:ident, $e:expr, $op:expr) => {
        impl $crate::Monoid for $ident {
            #[inline]
            fn e() -> Self::S {
                $e
            }
            #[inline]
            fn op(x: &Self::S, y: &Self::S) -> Self::S {
                $op(x, y)
            }
        }
    };
}

#[macro_export]
macro_rules! group {
    ($ident:ident, $e:expr, $op:expr, $inv:expr) => {
        impl $crate::Monoid for $ident {
            #[inline]
            fn e() -> Self::S {
                $e
            }
            #[inline]
            fn op(x: &Self::S, y: &Self::S) -> Self::S {
                $op(x, y)
            }
        }
        impl $crate::Group for $ident {
            #[inline]
            fn inv(x: &Self::S) -> Self::S {
                $inv(x)
            }
        }
    };
}

macro_rules! impl_zero_one {
    ($($t:ty)*) => {
        $(
            impl $crate::Zero for $t {
                fn zero() -> Self {
                    0
                }
                fn is_zero(&self) -> bool {
                    *self == 0
                }
            }
            impl $crate::One for $t {
                fn one() -> Self {
                    1
                }
                fn is_one(&self) -> bool {
                    *self == 1
                }
            }
        )*
    };
}

impl_zero_one!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);
