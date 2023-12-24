pub fn euler_phi(n: usize) -> Vec<usize> {
    let mut p = vec![0; n + 1];
    for i in 0..=n {
        p[i] = i;
    }
    for i in 2..=n {
        if p[i] != i {
            continue;
        }
        for j in (i..=n).step_by(i) {
            p[j] /= i;
            p[j] *= i - 1;
        }
    }
    p
}
