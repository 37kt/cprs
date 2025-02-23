use std::{convert::Infallible, marker::PhantomData};

use algebraic_traits::{Algebraic, Associative, Commutative, Invertive, Magma, Pow, Unital};
use dynamic_modint::DynamicModInt;
use static_modint::StaticModInt;

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
                    1 as $t
                }
            }

            impl Associative for MulOperator<$t> {}

            impl Commutative for MulOperator<$t> {}

            impl Pow for MulOperator<$t> {}
        )*
    }
}

macro_rules! impl_mul_operator_float {
    ($($t:ty),*) => {
        $(
            impl Invertive for MulOperator<$t> {
                fn inv(x: &$t) -> $t {
                    1.0 / *x
                }
            }
        )*
    }
}

impl_mul_operator!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
impl_mul_operator_float!(f32, f64);

impl<const MOD: u32> Algebraic for MulOperator<StaticModInt<MOD>> {
    type Element = StaticModInt<MOD>;
}

impl<const MOD: u32> Magma for MulOperator<StaticModInt<MOD>> {
    fn op(a: &StaticModInt<MOD>, b: &StaticModInt<MOD>) -> StaticModInt<MOD> {
        *a * *b
    }
}

impl<const MOD: u32> Unital for MulOperator<StaticModInt<MOD>> {
    fn unit() -> StaticModInt<MOD> {
        StaticModInt::from(1)
    }
}

impl<const MOD: u32> Associative for MulOperator<StaticModInt<MOD>> {}

impl<const MOD: u32> Commutative for MulOperator<StaticModInt<MOD>> {}

impl<const MOD: u32> Pow for MulOperator<StaticModInt<MOD>> {
    fn pow(x: &StaticModInt<MOD>, exp: usize) -> StaticModInt<MOD> {
        x.pow(exp)
    }
}

impl<const MOD: u32> Invertive for MulOperator<StaticModInt<MOD>> {
    fn inv(x: &StaticModInt<MOD>) -> StaticModInt<MOD> {
        x.recip()
    }
}

impl<ID> Algebraic for MulOperator<DynamicModInt<ID>> {
    type Element = DynamicModInt<ID>;
}

impl<ID> Magma for MulOperator<DynamicModInt<ID>> {
    fn op(a: &DynamicModInt<ID>, b: &DynamicModInt<ID>) -> DynamicModInt<ID> {
        *a * *b
    }
}

impl<ID> Unital for MulOperator<DynamicModInt<ID>> {
    fn unit() -> DynamicModInt<ID> {
        DynamicModInt::from(1)
    }
}

impl<ID> Associative for MulOperator<DynamicModInt<ID>> {}

impl<ID> Commutative for MulOperator<DynamicModInt<ID>> {}

impl<ID> Pow for MulOperator<DynamicModInt<ID>> {
    fn pow(x: &DynamicModInt<ID>, exp: usize) -> DynamicModInt<ID> {
        x.pow(exp)
    }
}

impl<ID> Invertive for MulOperator<DynamicModInt<ID>> {
    fn inv(x: &DynamicModInt<ID>) -> DynamicModInt<ID> {
        x.recip()
    }
}
