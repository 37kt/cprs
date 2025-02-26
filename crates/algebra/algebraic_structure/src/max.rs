use std::{cmp::Ordering, marker::PhantomData};

use algebraic_traits::{Algebraic, Associative, Commutative, Magma, Pow, Unital};
use numeric_traits::{NegInf, Numeric};

pub struct MaxOperator<T: Numeric + NegInf>(PhantomData<fn() -> T>);

impl<T: Numeric + NegInf> Algebraic for MaxOperator<T> {
    type Value = T;
}

impl<T: Numeric + NegInf> Magma for MaxOperator<T> {
    fn op(a: &T, b: &T) -> T {
        if a.partial_cmp(b)
            .expect("Error in MaxOperator: Cannot compare values containing NaN.")
            == Ordering::Greater
        {
            *a
        } else {
            *b
        }
    }
}

impl<T: Numeric + NegInf> Unital for MaxOperator<T> {
    fn unit() -> T {
        T::neg_inf()
    }
}

impl<T: Numeric + NegInf> Associative for MaxOperator<T> {}

impl<T: Numeric + NegInf> Commutative for MaxOperator<T> {}

impl<T: Numeric + NegInf> Pow for MaxOperator<T> {
    fn pow(x: &T, exp: usize) -> T {
        if exp == 0 {
            Self::unit()
        } else {
            *x
        }
    }
}
