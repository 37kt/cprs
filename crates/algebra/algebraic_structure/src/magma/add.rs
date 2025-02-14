use std::{convert::Infallible, marker::PhantomData};

use algebraic_traits::{Algebraic, Associative, Commutative, Invertive, Magma, Pow, Unital};

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

impl_add_operator!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_add_operator_signed!(i8, i16, i32, i64, i128, isize);
