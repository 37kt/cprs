use std::ops::{Bound, RangeBounds};

use numeric_traits::Integer;

pub trait IntoHalfOpenRange<T>: RangeBounds<T>
where
    T: Integer,
{
    fn into_half_open_range(&self, l: T, r: T) -> (T, T) {
        let start = match self.start_bound() {
            Bound::Unbounded => l,
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + T::one(),
        };
        let end = match self.end_bound() {
            Bound::Unbounded => r,
            Bound::Included(&end) => end + T::one(),
            Bound::Excluded(&end) => end,
        };
        assert!(l <= start && start <= end && end <= r);
        (start, end)
    }
}

impl<R, T> IntoHalfOpenRange<T> for R
where
    R: RangeBounds<T>,
    T: Integer,
{
}
