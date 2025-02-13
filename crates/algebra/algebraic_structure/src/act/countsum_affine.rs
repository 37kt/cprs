use std::{convert::Infallible, marker::PhantomData};

use algebraic_traits::{Act, Semiring};

use crate::magma::{Affine, AffineOperator, CountSum, CountSumOperator};

pub struct CountsumAffineOperator<T: Semiring>(Infallible, PhantomData<fn() -> T>);

impl<T: Semiring> Act for CountsumAffineOperator<T>
where
    T::Element: Clone,
{
    type Operator = AffineOperator<T>;
    type Operand = CountSumOperator<T>;

    fn act(CountSum { count, sum }: &CountSum<T>, Affine(a, b): &Affine<T>) -> CountSum<T> {
        CountSum {
            count: count.clone(),
            sum: T::add(&T::mul(a, sum), &T::mul(b, count)),
        }
    }
}
