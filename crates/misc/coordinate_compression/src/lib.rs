use numeric_traits::{Cast, Integer};

/// SMALL == true: O(max-min) サイズの配列を使用  
/// SMALL == false: ソートして二分探索  
/// DISTINCT == true: x それぞれに異なる値を割り当てる  
/// DISTINCT == false: 同じ値には同じ値を割り当てる  
#[derive(Clone)]
pub struct CoordinateCompression<T, const SMALL: bool = false, const DISTINCT: bool = false>(
    Container<T>,
)
where
    T: Integer + Cast<usize>,
    u32: Cast<T>;

impl<T, const SMALL: bool, const DISTINCT: bool> CoordinateCompression<T, SMALL, DISTINCT>
where
    T: Integer + Cast<usize>,
    u32: Cast<T>,
{
    pub fn new(xs: impl IntoIterator<Item = T>) -> (Self, Vec<usize>) {
        let xs = xs.into_iter().collect::<Vec<_>>();
        let n = xs.len();
        if n == 0 {
            return (Self(Container::Empty), vec![]);
        }
        let min = *xs.iter().min().unwrap();
        let max = *xs.iter().max().unwrap();
        let mut ys = vec![0; xs.len()];
        if SMALL {
            let len = (max - min).cast() + 1;
            let mut sum = vec![0; len + 1];
            if DISTINCT {
                for &x in &xs {
                    sum[(x - min).cast() + 1] += 1;
                }
                for i in 0..len {
                    sum[i + 1] += sum[i];
                }
                for i in 0..n {
                    let j = (xs[i] - min).cast();
                    ys[i] = sum[j] as usize;
                    sum[j] += 1;
                }
                for i in (1..=len).rev() {
                    sum[i] = sum[i - 1];
                }
                sum[0] = 0;
                return (Self(Container::Small(Small { min, sum })), ys);
            } else {
                for &x in &xs {
                    sum[(x - min).cast() + 1] = 1;
                }
                for i in 0..len {
                    sum[i + 1] += sum[i];
                }
                for i in 0..n {
                    let j = (xs[i] - min).cast();
                    ys[i] = sum[j] as usize;
                }
                return (Self(Container::Small(Small { min, sum })), ys);
            }
        } else {
            if DISTINCT {
                let mut ord = (0..n).collect::<Vec<_>>();
                ord.sort_by_key(|&i| xs[i]);
                let mut new_xs = Vec::with_capacity(n);
                for i in 0..n {
                    ys[ord[i]] = i;
                    new_xs.push(xs[ord[i]]);
                }
                return (Self(Container::Large(Large { xs: new_xs })), ys);
            } else {
                let mut ord = (0..n).collect::<Vec<_>>();
                ord.sort_unstable_by_key(|&i| xs[i]);
                let mut new_xs = Vec::with_capacity(n);
                for i in 0..n {
                    if i > 0 && xs[ord[i]] == xs[ord[i - 1]] {
                        ys[ord[i]] = new_xs.len() - 1;
                    } else {
                        ys[ord[i]] = new_xs.len();
                        new_xs.push(xs[ord[i]]);
                    }
                }
                new_xs.shrink_to_fit();
                return (Self(Container::Large(Large { xs: new_xs })), ys);
            }
        }
    }

    pub fn encode(&self, x: T) -> usize {
        match &self.0 {
            Container::Empty => 0,
            Container::Small(Small { min, sum, .. }) => {
                let j = (x - *min).max(T::zero()).cast().min(sum.len() - 1);
                sum[j] as usize
            }
            Container::Large(Large { xs }) => xs.partition_point(|&xi| xi < x),
        }
    }

    pub fn decode(&self, i: usize) -> T {
        match &self.0 {
            Container::Empty => panic!("out of range"),
            Container::Small(Small { min, sum, .. }) => *min + sum[i].cast(),
            Container::Large(Large { xs }) => xs[i],
        }
    }
}

impl<T, const SMALL: bool, const DISTINCT: bool> CoordinateCompression<T, SMALL, DISTINCT>
where
    T: Integer + Cast<usize>,
    u32: Cast<T>,
{
    /// 値の範囲 \[0, n) の n を返す
    pub fn len(&self) -> usize {
        match &self.0 {
            Container::Empty => 0,
            Container::Small(Small { sum, .. }) => *sum.last().unwrap() as usize,
            Container::Large(Large { xs }) => xs.len(),
        }
    }
}

#[derive(Clone)]
struct Small<T> {
    min: T,
    sum: Vec<u32>,
}

#[derive(Clone)]
struct Large<T> {
    xs: Vec<T>,
}

#[derive(Clone)]
enum Container<T> {
    Empty,
    Small(Small<T>),
    Large(Large<T>),
}
