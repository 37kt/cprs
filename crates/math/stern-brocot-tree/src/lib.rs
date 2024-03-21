use rational::{Rational, ZTrait};

pub struct SternBrocotTreeNode<T>
where
    T: ZTrait,
{
    l: Rational<T>,
    m: Rational<T>,
    r: Rational<T>,
    seq: Vec<T>,
}

impl<T> Default for SternBrocotTreeNode<T>
where
    T: ZTrait,
{
    fn default() -> Self {
        Self {
            l: Rational::new(T::zero(), T::one()),
            m: Rational::new(T::one(), T::one()),
            r: Rational::new(T::one(), T::zero()),
            seq: vec![],
        }
    }
}

impl<T> SternBrocotTreeNode<T>
where
    T: ZTrait,
{
    pub fn new(mut x: Rational<T>) -> Self {
        assert!(x.num > T::zero() && x.den > T::zero());
        x.reduce();
        let mut n = Self::default();
        while x.num.min(x.den) > T::zero() {
            if x.num > x.den {
                let d = x.num / x.den;
                x.num -= x.den * d;
                n.go_child(
                    d - if x.num == T::zero() {
                        T::one()
                    } else {
                        T::zero()
                    },
                );
            } else {
                let d = x.den / x.num;
                x.den -= x.num * d;
                n.go_child(
                    if x.den == T::zero() {
                        T::one()
                    } else {
                        T::zero()
                    } - d,
                );
            }
        }
        n
    }

    pub fn depth(&self) -> T {
        self.seq.iter().map(|&x| x.abs()).sum::<T>()
    }

    pub fn go_child(&mut self, d: T) {
        if d == T::zero() {
            return;
        }
        if self.seq.is_empty() || (*self.seq.last().unwrap() < T::zero()) != (d < T::zero()) {
            self.seq.push(d);
        } else {
            *self.seq.last_mut().unwrap() += d;
        }
        if d < T::zero() {
            self.r.num += self.l.num * -d;
            self.r.den += self.l.den * -d;
            self.m = Rational {
                num: self.l.num + self.r.num,
                den: self.l.den + self.r.den,
            };
        } else {
            self.l.num += self.r.num * d;
            self.l.den += self.r.den * d;
            self.m = Rational {
                num: self.l.num + self.r.num,
                den: self.l.den + self.r.den,
            };
        }
    }

    pub fn go_parent(&mut self, mut d: T) -> bool {
        assert!(d >= T::zero());
        if d == T::zero() {
            return true;
        }
        while d != T::zero() {
            if self.seq.is_empty() {
                return false;
            }
            let mut x = self.seq.pop().unwrap();
            let d2 = d.min(x.abs());
            if x < T::zero() {
                self.m.num -= self.l.num * d2;
                self.m.den -= self.l.den * d2;
                self.r = Rational {
                    num: self.m.num - self.l.num,
                    den: self.m.den - self.l.den,
                };
                x += d2;
            } else {
                self.m.num -= self.r.num * d2;
                self.m.den -= self.r.den * d2;
                self.l = Rational {
                    num: self.m.num - self.r.num,
                    den: self.m.den - self.r.den,
                };
                x -= d2;
            }
            d -= d2;
            if x != T::zero() {
                self.seq.push(x);
            }
            if d2 == T::zero() {
                break;
            }
        }
        true
    }

    pub fn lca(&self, other: &SternBrocotTreeNode<T>) -> SternBrocotTreeNode<T> {
        let mut res = SternBrocotTreeNode::default();
        for (&x, &y) in self.seq.iter().zip(&other.seq) {
            if (x < T::zero()) != (y < T::zero()) {
                break;
            }
            if x < T::zero() {
                res.go_child(x.max(y));
            } else if x > T::zero() {
                res.go_child(x.min(y));
            }
            if x != y {
                break;
            }
        }
        res
    }

    pub fn lower_bound(&self) -> Rational<T> {
        self.l
    }

    pub fn upper_bound(&self) -> Rational<T> {
        self.r
    }

    pub fn val(&self) -> Rational<T> {
        self.m
    }

    pub fn path(&self) -> Vec<T> {
        self.seq.clone()
    }

    pub fn binary_search(f: impl Fn(Rational<T>) -> bool, lim: T) -> (Rational<T>, Rational<T>) {
        let mut l = Rational::new(T::zero(), T::one());
        let mut r = Rational::new(T::one(), T::zero());
        let f01 = f(l);
        let f10 = f(r);
        assert!(f01 != f10);
        loop {
            let m = Rational {
                num: l.num + r.num,
                den: l.den + r.den,
            };
            if m.num > lim || m.den > lim {
                break;
            }
            let fm = f(m);
            let sgn = (if fm != f01 { -1 } else { 1 }).into();
            let maxd = if sgn == -T::one() {
                if l.num == T::zero() {
                    (lim - r.den) / l.den
                } else {
                    ((lim - r.den) / l.den).min((lim - r.num) / l.num)
                }
            } else {
                if r.den == T::zero() {
                    (lim - l.num) / r.num
                } else {
                    ((lim - l.num) / r.num).min((lim - l.den) / r.den)
                }
            };
            let mut dl: T = 1.into();
            let mut dr: T = 2.into();
            while dr <= maxd {
                let mut n = SternBrocotTreeNode::new(m);
                n.go_child(dr * sgn);
                if f(n.val()) == fm {
                    dl = dr;
                    dr = (dr * 2.into()).min(maxd + 1.into());
                } else {
                    break;
                }
            }
            while dl + 1.into() < dr {
                let dm = dl + (dr - dl) / 2.into();
                let mut n = SternBrocotTreeNode::new(m);
                n.go_child(dm * sgn);
                if f(n.val()) == fm {
                    dl = dm;
                } else {
                    dr = dm;
                }
            }
            let mut n = SternBrocotTreeNode::new(m);
            n.go_child(dl * sgn);
            l = n.lower_bound();
            r = n.upper_bound();
        }
        (l, r)
    }
}

impl<T> PartialEq for SternBrocotTreeNode<T>
where
    T: ZTrait,
{
    fn eq(&self, other: &Self) -> bool {
        self.m == other.m
    }
}

impl<T> Eq for SternBrocotTreeNode<T> where T: ZTrait {}

impl<T> PartialOrd for SternBrocotTreeNode<T>
where
    T: ZTrait,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for SternBrocotTreeNode<T>
where
    T: ZTrait,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.m.cmp(&other.m)
    }
}
