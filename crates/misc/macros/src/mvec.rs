/// mvec![]
#[macro_export]
macro_rules! mvec {
    ($x:expr; $n:expr) => {
        vec![$x; $n]
    };
    ($x:expr; $n:expr $(; $m:expr)+) => {
        vec![mvec![$x $(; $m)*]; $n]
    };
}
