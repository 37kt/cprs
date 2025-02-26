use std::marker::PhantomData;

use algebraic_traits::{Algebraic, Associative, Commutative, Magma, Unital};
use numeric_traits::Numeric;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CountSum<T: Numeric> {
    pub count: T,
    pub sum: T,
}

pub struct CountSumOperator<T: Numeric>(PhantomData<fn() -> T>);

impl<T: Numeric> Algebraic for CountSumOperator<T> {
    type Value = CountSum<T>;
}

impl<T: Numeric> Magma for CountSumOperator<T> {
    fn op(
        &CountSum { count: c1, sum: s1 }: &CountSum<T>,
        &CountSum { count: c2, sum: s2 }: &CountSum<T>,
    ) -> CountSum<T> {
        CountSum {
            count: c1 + c2,
            sum: s1 + s2,
        }
    }
}

impl<T: Numeric> Unital for CountSumOperator<T> {
    fn unit() -> CountSum<T> {
        CountSum {
            count: T::zero(),
            sum: T::zero(),
        }
    }
}

impl<T: Numeric> Associative for CountSumOperator<T> {}

impl<T: Numeric> Commutative for CountSumOperator<T> {}
