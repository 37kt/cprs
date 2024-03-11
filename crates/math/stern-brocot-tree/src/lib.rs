use rational::Rational;

pub struct SternBrocotTreeNode {
    l: Rational,
    m: Rational,
    r: Rational,
    seq: Vec<i128>,
}

impl Default for SternBrocotTreeNode {
    fn default() -> Self {
        Self {
            l: Rational::new(0, 1),
            m: Rational::new(1, 1),
            r: Rational::new(1, 0),
            seq: vec![],
        }
    }
}

impl SternBrocotTreeNode {
    pub fn new(mut x: Rational) -> Self {
        assert!(x.num > 0 && x.den > 0);
        x.reduce();
        let mut n = Self::default();
        while x.num.min(x.den) > 0 {
            if x.num > x.den {
                let d = x.num / x.den;
                x.num -= x.den * d;
                n.go_child(d - if x.num == 0 { 1 } else { 0 });
            } else {
                let d = x.den / x.num;
                x.den -= x.num * d;
                n.go_child(if x.den == 0 { 1 } else { 0 } - d);
            }
        }
        n
    }

    pub fn depth(&self) -> i128 {
        self.seq.iter().map(|&x| x.abs()).sum::<i128>()
    }

    pub fn go_child(&mut self, d: i128) {
        if d == 0 {
            return;
        }
        if self.seq.is_empty() || (*self.seq.last().unwrap() < 0) != (d < 0) {
            self.seq.push(d);
        } else {
            *self.seq.last_mut().unwrap() += d;
        }
        if d < 0 {
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

    pub fn go_parent(&mut self, mut d: i128) -> bool {
        assert!(d >= 0);
        if d == 0 {
            return true;
        }
        while d != 0 {
            if self.seq.is_empty() {
                return false;
            }
            let mut x = self.seq.pop().unwrap();
            let d2 = d.min(x.abs());
            if x < 0 {
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
            if x != 0 {
                self.seq.push(x);
            }
            if d2 == 0 {
                break;
            }
        }
        true
    }

    pub fn lca(&self, other: &SternBrocotTreeNode) -> SternBrocotTreeNode {
        let mut res = SternBrocotTreeNode::default();
        for (&x, &y) in self.seq.iter().zip(&other.seq) {
            if (x < 0) != (y < 0) {
                break;
            }
            if x < 0 {
                res.go_child(x.max(y));
            } else if x > 0 {
                res.go_child(x.min(y));
            }
            if x != y {
                break;
            }
        }
        res
    }

    pub fn lower_bound(&self) -> Rational {
        self.l
    }

    pub fn upper_bound(&self) -> Rational {
        self.r
    }

    pub fn val(&self) -> Rational {
        self.m
    }

    pub fn path(&self) -> Vec<i128> {
        self.seq.clone()
    }
}

impl PartialEq for SternBrocotTreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.m == other.m
    }
}

impl Eq for SternBrocotTreeNode {}

impl PartialOrd for SternBrocotTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SternBrocotTreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.m.cmp(&other.m)
    }
}
