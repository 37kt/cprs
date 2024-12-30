/// a に含まれない最小の非負整数を求める。
pub fn mex(a: &[usize]) -> usize {
    let mut a = a.to_vec();
    for i in 0..a.len() {
        while i < a.len() && a[i] != i {
            if a[i] >= a.len() || a[i] == a[a[i]] {
                a.swap_remove(i);
            } else {
                let j = a[i];
                a.swap(i, j);
            }
        }
    }
    a.iter()
        .enumerate()
        .position(|(i, &x)| x != i)
        .unwrap_or(a.len())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use rand::Rng;

    use super::*;

    #[test]
    fn test_mex() {
        for _ in 0..1000 {
            let mut rng = rand::thread_rng();
            let n = rng.gen_range(0..100);
            let m = n + 10;
            let a = (0..n).map(|_| rng.gen_range(0..m)).collect::<Vec<_>>();
            let res = mex(&a);

            let mut st = BTreeSet::new();
            for i in 0..=m {
                st.insert(i);
            }
            for &x in &a {
                st.remove(&x);
            }
            assert_eq!(res, *st.iter().next().unwrap());
        }
    }
}
