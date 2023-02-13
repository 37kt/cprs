fn mid(l: i64, r: i64) -> i64 {
    l + (r - l) / 2
}

fn min(a: Option<i64>, b: Option<i64>) -> Option<i64> {
    match (a, b) {
        (None, _) => b,
        (_, None) => a,
        (Some(a), Some(b)) => Some(a.min(b)),
    }
}

#[derive(Clone, Copy)]
struct Line(i64, i64);

impl Line {
    fn eval(&self, x: i64) -> i64 {
        self.0 * x + self.1
    }
}

struct Node {
    f: Option<Line>,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

impl Node {
    fn find(&self, l: i64, r: i64, x: i64) -> Option<i64> {
        assert!(l <= x && x < r);
        let mut res = None;
        if let Some(f) = self.f {
            let y = f.eval(x);
            res = min(res, Some(y));
        }
        let m = mid(l, r);
        if x < m {
            if let Some(c) = &self.l {
                let y = c.find(l, m, x);
                res = min(res, y);
            }
        } else {
            if let Some(c) = &self.r {
                let y = c.find(m, r, x);
                res = min(res, y);
            }
        }
        res
    }

    fn insert(&mut self, l: i64, r: i64, a: i64, b: i64, mut f: Line) {
        if r <= a || b <= l {
            return;
        }
        if a <= l && r <= b {
            if self.f.is_none() {
                self.f = Some(f);
                return;
            }
            if self.f.unwrap().eval(l) > f.eval(l) {
                std::mem::swap(self.f.as_mut().unwrap(), &mut f);
            }
            let m = mid(l, r);
            if self.f.unwrap().eval(r) <= f.eval(r) {
                return;
            } else if self.f.unwrap().eval(m) < f.eval(m) {
                if self.r.is_none() {
                    self.r = Some(Box::new(Node {
                        f: None,
                        l: None,
                        r: None,
                    }));
                }
                self.r.as_mut().unwrap().insert(m, r, a, b, f);
            } else {
                if self.l.is_none() {
                    self.l = Some(Box::new(Node {
                        f: None,
                        l: None,
                        r: None,
                    }));
                }
                std::mem::swap(self.f.as_mut().unwrap(), &mut f);
                self.l.as_mut().unwrap().insert(l, m, a, b, f);
            }
        } else {
            let m = mid(l, r);
            if self.l.is_none() {
                self.l = Some(Box::new(Node {
                    f: None,
                    l: None,
                    r: None,
                }));
            }
            self.l.as_mut().unwrap().insert(l, m, a, b, f);
            if self.r.is_none() {
                self.r = Some(Box::new(Node {
                    f: None,
                    l: None,
                    r: None,
                }));
            }
            self.r.as_mut().unwrap().insert(m, r, a, b, f);
        }
    }
}

pub struct LiChaoTree {
    root: Option<Node>,
    min_x: i64,
    max_x: i64,
    max_query: bool,
}

impl LiChaoTree {
    // max_queryがfalseなら最小値クエリ、trueなら最大値クエリ
    pub fn new(min_x: i64, max_x: i64, max_query: bool) -> Self {
        Self {
            root: None,
            min_x,
            max_x,
            max_query,
        }
    }

    // 直線ax+bを追加
    pub fn add_line(&mut self, a: i64, b: i64) {
        self.add_segment(a, b, self.min_x, self.max_x);
    }

    // 線分ax+b(l<=x<r)を追加
    pub fn add_segment(&mut self, mut a: i64, mut b: i64, l: i64, r: i64) {
        if self.max_query {
            a *= -1;
            b *= -1;
        }
        if self.root.is_none() {
            self.root = Some(Node {
                f: None,
                l: None,
                r: None,
            });
        }
        self.root
            .as_mut()
            .unwrap()
            .insert(self.min_x, self.max_x, l, r, Line(a, b));
    }

    // ax+bのxを定めたときの最小値を求める
    pub fn find(&self, x: i64) -> Option<i64> {
        self.root
            .as_ref()
            .and_then(|v| v.find(self.min_x, self.max_x, x))
            .map(|res| res * if self.max_query { -1 } else { 1 })
    }
}
