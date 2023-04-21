use std::collections::HashSet;

/// f^n s = t となる最初の n (n < lim) が存在するなら返す
pub fn discrete_logarithm<S, F, Act, Composition>(
    mut s: S,
    t: S,
    f: F,
    act: Act,
    composition: Composition,
    lim: usize,
) -> Option<usize>
where
    S: Clone + std::hash::Hash + Eq,
    F: Clone,
    Act: Fn(&F, &S) -> S,
    Composition: Fn(&F, &F) -> F,
{
    let fpow = |mut n: usize| {
        assert!(n > 0);
        n -= 1;
        let mut p = f.clone();
        let mut res = f.clone();
        while n > 0 {
            if n & 1 == 1 {
                res = composition(&res, &p);
            }
            p = composition(&p, &p);
            n >>= 1;
        }
        res
    };

    let m = (lim as f64).sqrt().ceil() as usize;
    let mut st = HashSet::new();
    let mut tt = t.clone();
    for _ in 0..m {
        tt = act(&f, &tt);
        st.insert(tt.clone());
    }
    let g = fpow(m);
    let mut failed = false;
    for i in 0..=m {
        let s1 = act(&g, &s);
        if st.contains(&s1) {
            for j in 0..m {
                if s == t {
                    let res = i * m + j;
                    return if res >= lim { None } else { Some(res) };
                }
                s = act(&f, &s);
            }
            if failed {
                return None;
            }
            failed = true;
        }
        s = s1;
    }

    None
}
