use std::{convert::Infallible, marker::PhantomData};

use algebraic_traits::{Algebraic, Associative, Commutative, Invertive, Magma, Pow, Unital};
use modint::{DynamicModInt, StaticModInt};

pub struct AddOperator<T>(Infallible, PhantomData<fn() -> T>);

macro_rules! impl_add_operator {
    ($($t:ty),*) => {
        $(
            impl Algebraic for AddOperator<$t> {
                type Element = $t;
            }

            impl Magma for AddOperator<$t> {
                fn op(a: &$t, b: &$t) -> $t {
                    *a + *b
                }
            }

            impl Unital for AddOperator<$t> {
                fn unit() -> $t {
                    0
                }
            }

            impl Associative for AddOperator<$t> {}

            impl Commutative for AddOperator<$t> {}

            impl Invertive for AddOperator<$t> {
                fn inv(x: &$t) -> $t {
                    0 - *x
                }
            }

            impl Pow for AddOperator<$t> {
                fn pow(x: &$t, exp: usize) -> $t {
                    *x * exp as $t
                }
            }
        )*
    }
}

impl_add_operator!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<const P: u32> Algebraic for AddOperator<StaticModInt<P>> {
    type Element = StaticModInt<P>;
}

impl<const P: u32> Magma for AddOperator<StaticModInt<P>> {
    fn op(a: &StaticModInt<P>, b: &StaticModInt<P>) -> StaticModInt<P> {
        a + b
    }
}

impl<const P: u32> Unital for AddOperator<StaticModInt<P>> {
    fn unit() -> StaticModInt<P> {
        0.into()
    }
}

impl<const P: u32> Associative for AddOperator<StaticModInt<P>> {}

impl<const P: u32> Commutative for AddOperator<StaticModInt<P>> {}

impl<const P: u32> Invertive for AddOperator<StaticModInt<P>> {
    fn inv(x: &StaticModInt<P>) -> StaticModInt<P> {
        -x
    }
}

impl<const P: u32> Pow for AddOperator<StaticModInt<P>> {
    fn pow(x: &StaticModInt<P>, exp: usize) -> StaticModInt<P> {
        x * exp
    }
}

impl Algebraic for AddOperator<DynamicModInt> {
    type Element = DynamicModInt;
}

impl Magma for AddOperator<DynamicModInt> {
    fn op(a: &DynamicModInt, b: &DynamicModInt) -> DynamicModInt {
        a + b
    }
}

impl Unital for AddOperator<DynamicModInt> {
    fn unit() -> DynamicModInt {
        0.into()
    }
}

impl Associative for AddOperator<DynamicModInt> {}

impl Commutative for AddOperator<DynamicModInt> {}

impl Invertive for AddOperator<DynamicModInt> {
    fn inv(x: &DynamicModInt) -> DynamicModInt {
        -x
    }
}

impl Pow for AddOperator<DynamicModInt> {
    fn pow(x: &DynamicModInt, exp: usize) -> DynamicModInt {
        x * exp
    }
}
