use std::marker::PhantomData;

use algebraic_traits::{Algebraic, Associative, Commutative, Invertive, Magma, Pow, Unital};
use numeric_traits::Integer;

pub struct XorOperator<T: Integer>(PhantomData<fn() -> T>);

impl<T: Integer> Algebraic for XorOperator<T> {
    type Value = T;
}

impl<T: Integer> Magma for XorOperator<T> {
    fn op(a: &T, b: &T) -> T {
        *a ^ *b
    }
}

impl<T: Integer> Unital for XorOperator<T> {
    fn unit() -> T {
        T::zero()
    }
}

impl<T: Integer> Associative for XorOperator<T> {}

impl<T: Integer> Commutative for XorOperator<T> {}

impl<T: Integer> Invertive for XorOperator<T> {
    fn inv(x: &T) -> T {
        *x
    }
}

impl<T: Integer> Pow for XorOperator<T> {
    fn pow(x: &T, exp: usize) -> T {
        if exp & 1 == 0 {
            T::zero()
        } else {
            *x
        }
    }
}
