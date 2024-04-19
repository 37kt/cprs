pub fn aliens_dp_convex(x: usize, p_lb: i64, p_ub: i64, g: impl Fn(i64) -> i64) -> i64 {
    let x = x as i64;
    assert!(p_lb < p_ub);
    let mut l = p_lb - 1;
    let mut r = p_ub + 1;
    while l + 1 < r {
        let p = l + (r - l) / 2;
        let c = g(p + 1) - g(p);
        if c <= x {
            r = p;
        } else {
            l = p;
        }
    }
    g(r) - r * x
}

pub fn aliens_dp_concave(x: usize, p_lb: i64, p_ub: i64, g: impl Fn(i64) -> i64) -> i64 {
    let x = x as i64;
    assert!(p_lb < p_ub);
    let mut l = p_lb - 1;
    let mut r = p_ub + 1;
    while l + 1 < r {
        let p = l + (r - l) / 2;
        let c = g(p) - g(p + 1);
        if c <= x {
            r = p;
        } else {
            l = p;
        }
    }
    g(r) + r * x
}
