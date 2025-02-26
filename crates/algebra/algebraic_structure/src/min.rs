use std::{cmp::Ordering, marker::PhantomData};

use algebraic_traits::{Algebraic, Associative, Commutative, Magma, Pow, Unital};
use numeric_traits::{Inf, Numeric};

pub struct MinOperator<T: Numeric + Inf>(PhantomData<fn() -> T>);

impl<T: Numeric + Inf> Algebraic for MinOperator<T> {
    type Value = T;
}

impl<T: Numeric + Inf> Magma for MinOperator<T> {
    fn op(a: &T, b: &T) -> T {
        if a.partial_cmp(b)
            .expect("Error in MinOperator: Cannot compare values containing NaN.")
            == Ordering::Less
        {
            *a
        } else {
            *b
        }
    }
}

impl<T: Numeric + Inf> Unital for MinOperator<T> {
    fn unit() -> T {
        T::inf()
    }
}

impl<T: Numeric + Inf> Associative for MinOperator<T> {}

impl<T: Numeric + Inf> Commutative for MinOperator<T> {}

impl<T: Numeric + Inf> Pow for MinOperator<T> {
    fn pow(x: &T, exp: usize) -> T {
        if exp == 0 {
            Self::unit()
        } else {
            *x
        }
    }
}
