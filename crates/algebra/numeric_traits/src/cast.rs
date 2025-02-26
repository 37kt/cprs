pub trait Cast<T> {
    fn cast(self) -> T;
}

macro_rules! impl_cast_2 {
    ($t1:ty, $($t2:ty),*) => {
        $(
            impl Cast<$t2> for $t1 {
                fn cast(self) -> $t2 {
                    self as $t2
                }
            }
        )*
    }
}

macro_rules! impl_cast_1 {
    ($($t:ty),*) => {
        $(
            impl_cast_2! {
                $t, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize, f32, f64
            }
        )*
    }
}

impl_cast_1! {
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize, f32, f64
}
