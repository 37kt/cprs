use std::{convert::Infallible, marker::PhantomData};

use algebraic_traits::{Algebraic, Associative, Commutative, Invertive, Magma, Pow, Unital};
use dynamic_modint::DynamicModInt;
use static_modint::StaticModInt;

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
                    0 as $t
                }
            }

            impl Associative for AddOperator<$t> {}

            impl Commutative for AddOperator<$t> {}

            impl Pow for AddOperator<$t> {
                fn pow(x: &$t, exp: usize) -> $t {
                    *x * exp as $t
                }
            }
        )*
    }
}

macro_rules! impl_add_operator_signed {
    ($($t:ty),*) => {
        $(
            impl Invertive for AddOperator<$t> {
                fn inv(x: &$t) -> $t {
                    -*x
                }
            }
        )*
    }
}

impl_add_operator!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
impl_add_operator_signed!(i8, i16, i32, i64, i128, isize, f32, f64);

impl<const MOD: u32> Algebraic for AddOperator<StaticModInt<MOD>> {
    type Element = StaticModInt<MOD>;
}

impl<const MOD: u32> Magma for AddOperator<StaticModInt<MOD>> {
    fn op(a: &StaticModInt<MOD>, b: &StaticModInt<MOD>) -> StaticModInt<MOD> {
        *a + *b
    }
}

impl<const MOD: u32> Unital for AddOperator<StaticModInt<MOD>> {
    fn unit() -> StaticModInt<MOD> {
        StaticModInt::from_raw(0)
    }
}

impl<const MOD: u32> Associative for AddOperator<StaticModInt<MOD>> {}

impl<const MOD: u32> Commutative for AddOperator<StaticModInt<MOD>> {}

impl<const MOD: u32> Pow for AddOperator<StaticModInt<MOD>> {
    fn pow(x: &StaticModInt<MOD>, exp: usize) -> StaticModInt<MOD> {
        *x * exp
    }
}

impl<const MOD: u32> Invertive for AddOperator<StaticModInt<MOD>> {
    fn inv(x: &StaticModInt<MOD>) -> StaticModInt<MOD> {
        -*x
    }
}

impl<ID> Algebraic for AddOperator<DynamicModInt<ID>> {
    type Element = DynamicModInt<ID>;
}

impl<ID> Magma for AddOperator<DynamicModInt<ID>> {
    fn op(a: &DynamicModInt<ID>, b: &DynamicModInt<ID>) -> DynamicModInt<ID> {
        *a + *b
    }
}

impl<ID> Unital for AddOperator<DynamicModInt<ID>> {
    fn unit() -> DynamicModInt<ID> {
        DynamicModInt::from_raw(0)
    }
}

impl<ID> Associative for AddOperator<DynamicModInt<ID>> {}

impl<ID> Commutative for AddOperator<DynamicModInt<ID>> {}

impl<ID> Pow for AddOperator<DynamicModInt<ID>> {
    fn pow(x: &DynamicModInt<ID>, exp: usize) -> DynamicModInt<ID> {
        *x * exp
    }
}

impl<ID> Invertive for AddOperator<DynamicModInt<ID>> {
    fn inv(x: &DynamicModInt<ID>) -> DynamicModInt<ID> {
        -*x
    }
}
