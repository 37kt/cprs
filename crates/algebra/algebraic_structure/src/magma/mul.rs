use std::{convert::Infallible, marker::PhantomData};

use algebraic_traits::{Algebraic, Associative, Commutative, Magma, Pow, Unital};

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
