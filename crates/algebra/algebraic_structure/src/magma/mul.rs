use std::{convert::Infallible, marker::PhantomData};

use algebraic_traits::{Algebraic, Associative, Commutative, Invertive, Magma, Pow, Unital};
use modint::{DynamicModInt, StaticModInt};

pub struct MulOperator<T>(Infallible, PhantomData<fn() -> T>);

macro_rules! impl_mul_operator {
    ($($t:ty),*) => {
        $(
            impl Algebraic for MulOperator<$t> {
                type Element = $t;
            }

            impl Magma for MulOperator<$t> {
                fn op(a: &$t, b: &$t) -> $t {
                    *a * *b
                }
            }

            impl Unital for MulOperator<$t> {
                fn unit() -> $t {
                    1
                }
            }

            impl Associative for MulOperator<$t> {}

            impl Commutative for MulOperator<$t> {}

            impl Pow for MulOperator<$t> {}
        )*
    }
}

impl_mul_operator!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<const P: u32> Algebraic for MulOperator<StaticModInt<P>> {
    type Element = StaticModInt<P>;
}

impl<const P: u32> Magma for MulOperator<StaticModInt<P>> {
    fn op(a: &StaticModInt<P>, b: &StaticModInt<P>) -> StaticModInt<P> {
        a * b
    }
}

impl<const P: u32> Unital for MulOperator<StaticModInt<P>> {
    fn unit() -> StaticModInt<P> {
        1.into()
    }
}

impl<const P: u32> Associative for MulOperator<StaticModInt<P>> {}

impl<const P: u32> Commutative for MulOperator<StaticModInt<P>> {}

impl<const P: u32> Invertive for MulOperator<StaticModInt<P>> {
    fn inv(x: &StaticModInt<P>) -> StaticModInt<P> {
        x.inv()
    }
}

impl<const P: u32> Pow for MulOperator<StaticModInt<P>> {
    fn pow(x: &StaticModInt<P>, exp: usize) -> StaticModInt<P> {
        x.pow(exp)
    }
}

impl Algebraic for MulOperator<DynamicModInt> {
    type Element = DynamicModInt;
}

impl Magma for MulOperator<DynamicModInt> {
    fn op(a: &DynamicModInt, b: &DynamicModInt) -> DynamicModInt {
        a * b
    }
}

impl Unital for MulOperator<DynamicModInt> {
    fn unit() -> DynamicModInt {
        1.into()
    }
}

impl Associative for MulOperator<DynamicModInt> {}

impl Commutative for MulOperator<DynamicModInt> {}

impl Pow for MulOperator<DynamicModInt> {
    fn pow(x: &DynamicModInt, exp: usize) -> DynamicModInt {
        x.pow(exp)
    }
}
