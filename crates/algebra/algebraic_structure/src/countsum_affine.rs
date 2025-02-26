use std::{convert::Infallible, marker::PhantomData};

use algebraic_traits::Act;
use numeric_traits::Numeric;

use crate::magma::{Affine, AffineOperator, CountSum, CountSumOperator};

pub struct CountsumAffineOperator<T>(Infallible, PhantomData<fn() -> T>);

impl<T: Numeric> Act for CountsumAffineOperator<T> {
    type Operand = CountSumOperator<T>;
    type Operator = AffineOperator<T>;

    fn act(&CountSum { count, sum }: &CountSum<T>, &Affine(a, b): &Affine<T>) -> CountSum<T> {
        CountSum {
            count,
            sum: a * sum + b * count,
        }
    }
}
