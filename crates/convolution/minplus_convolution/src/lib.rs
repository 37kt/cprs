mod convex_and_convex;
pub use convex_and_convex::*;

mod convex_and_arbitrary;
pub use convex_and_arbitrary::*;

use numeric_traits::Integer;

pub fn is_convex<T>(a: &[T]) -> bool
where
    T: Integer,
{
    a.windows(3).all(|a| a[1] + a[1] <= a[2] + a[0])
}

pub fn is_concave<T>(a: &[T]) -> bool
where
    T: Integer,
{
    a.windows(3).all(|a| a[1] + a[1] >= a[2] + a[0])
}
