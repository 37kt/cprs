use std::marker::PhantomData;

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
    fn pow(x: &Self::S, e: usize) -> Self::S {
        let mut res = Self::e();
        let mut x = Self::op(&Self::e(), x);
        let mut e = e;
        while e > 0 {
            if e & 1 == 1 {
                res = Self::op(&res, &x);
            }
            x = Self::op(&x, &x);
            e >>= 1;
        }
        res
    }
}

#[derive(Clone, Copy)]
pub struct ReversedMonoid<M: Monoid>(PhantomData<fn() -> M>);

impl<M: Monoid> Algebra for ReversedMonoid<M> {
    type S = M::S;
}

impl<M: Monoid> Monoid for ReversedMonoid<M> {
    fn e() -> Self::S {
        M::e()
    }

    fn op(x: &Self::S, y: &Self::S) -> Self::S {
        M::op(y, x)
    }
}

impl<M: Monoid + Act> Act for ReversedMonoid<M> {
    type X = M::X;

    fn act(f: &Self::S, x: &Self::X) -> Self::X {
        M::act(f, x)
    }
}

pub trait Group: Monoid {
    fn inv(x: &Self::S) -> Self::S;
}

#[derive(Clone, Copy)]
pub struct ReversedGroup<G: Group>(PhantomData<fn() -> G>);

impl<G: Group> Algebra for ReversedGroup<G> {
    type S = G::S;
}

impl<G: Group> Monoid for ReversedGroup<G> {
    fn e() -> Self::S {
        G::e()
    }

    fn op(x: &Self::S, y: &Self::S) -> Self::S {
        G::op(y, x)
    }
}

impl<G: Group> Group for ReversedGroup<G> {
    fn inv(x: &Self::S) -> Self::S {
        G::inv(x)
    }
}

impl<G: Group + Act> Act for ReversedGroup<G> {
    type X = G::X;

    fn act(f: &Self::S, x: &Self::X) -> Self::X {
        G::act(f, x)
    }
}

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

/// 代数型の定義  
/// algebra!(代数型名, 型)  
/// # Example
/// ```
/// use algebraic::algebra;
/// algebra!(M, i32);
/// ```
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

/// 代数型に作用を定義  
/// act!(代数型名, 作用先の型, 作用)  
/// # Example
/// ```
/// use algebraic::{act, algebra};
/// algebra!(F, i32);
/// act!(F, i32, |&f: &i32, &x: &i32| f * x);
/// ```
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

/// モノイドの定義  
/// monoid!(代数型名, 単位元, 演算)  
/// # Example
/// ```
/// use algebraic::{algebra, monoid};
/// algebra!(M, i32);
/// monoid!(M, 1, |&x: &i32, &y: &i32| x * y);
/// ```
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

/// 群の定義  
/// group!(代数型名, 単位元, 演算, 逆元)  
/// # Example
/// ```
/// use algebraic::{algebra, group};
/// algebra!(G, i32);
/// group!(G, 1, |&x: &i32, &y: &i32| x + y, |&x: &i32| -x);
/// ```
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
