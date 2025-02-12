use std::marker::PhantomData;

use crate::{Affine, AffineOperator};
use algebraic_traits::{Act, Algebraic, Associative, Commutative, Magma, Semiring, Unital};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CountSumOperator<T: Semiring> {
    _marker: PhantomData<fn() -> T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CountSum<T: Semiring> {
    pub count: T::Element,
    pub sum: T::Element,
}

impl<T: Semiring> Algebraic for CountSumOperator<T> {
    type Element = CountSum<T>;
}

impl<T: Semiring> Magma for CountSumOperator<T> {
    fn op(
        CountSum { count: c1, sum: s1 }: &CountSum<T>,
        CountSum { count: c2, sum: s2 }: &CountSum<T>,
    ) -> CountSum<T> {
        CountSum {
            count: T::add(c1, c2),
            sum: T::add(s1, s2),
        }
    }
}

impl<T: Semiring> Unital for CountSumOperator<T> {
    fn unit() -> CountSum<T> {
        CountSum {
            count: T::zero(),
            sum: T::zero(),
        }
    }
}

impl<T: Semiring> Associative for CountSumOperator<T> {}

impl<T: Semiring> Commutative for CountSumOperator<T> {}

impl<T: Semiring> Act<CountSumOperator<T>> for AffineOperator<T>
where
    T::Element: Clone,
{
    fn act(Affine(a, b): &Affine<T>, CountSum { count, sum }: &CountSum<T>) -> CountSum<T> {
        CountSum {
            count: count.clone(),
            sum: T::add(&T::mul(a, sum), &T::mul(b, count)),
        }
    }
}
