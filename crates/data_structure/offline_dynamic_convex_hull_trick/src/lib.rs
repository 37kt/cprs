use numeric_traits::{Inf, Integer};

pub struct OfflineDynamicConvexHullTrick {
    lines: Vec<(i64, i64)>,
    add_time: Vec<usize>,
    remove_time: Vec<usize>,
    points: Vec<i64>,
    get_time: Vec<usize>,
    time: usize,
    last_is_get: bool,

    st: Vec<usize>,
    res: Vec<i64>,
}

impl OfflineDynamicConvexHullTrick {
    pub fn new() -> Self {
        Self {
            lines: vec![],
            add_time: vec![],
            remove_time: vec![],
            points: vec![],
            get_time: vec![],
            time: 0,
            last_is_get: false,

            st: vec![],
            res: vec![],
        }
    }

    /// 直線を追加して、その `id` を返す  
    pub fn add_line(&mut self, a: i64, b: i64) -> usize {
        if std::mem::replace(&mut self.last_is_get, false) {
            self.time += 1;
        }
        let id = self.lines.len();
        self.lines.push((a, b));
        self.add_time.push(self.time);
        self.remove_time.push(usize::inf());
        id
    }

    /// 直線 `id` を削除する
    pub fn remove_line(&mut self, id: usize) {
        assert!(id < self.lines.len(), "line {} is not added", id);
        if std::mem::replace(&mut self.last_is_get, false) {
            self.time += 1;
        }
        assert_eq!(
            std::mem::replace(&mut self.remove_time[id], self.time),
            usize::inf(),
            "line {} is already removed",
            id
        );
    }

    /// min(ax + b) を求めるクエリを発行して、クエリの `id` を返す  
    /// 答えは `solve` で返ってくる配列の `id` 番目に入る
    pub fn min(&mut self, x: i64) -> usize {
        self.last_is_get = true;
        let id = self.points.len();
        self.points.push(x);
        self.get_time.push(self.time);
        id
    }

    /// すべての `min` クエリの答えを返す  
    pub fn solve(mut self) -> Vec<i64> {
        if self.points.is_empty() {
            return vec![];
        }
        if self.last_is_get {
            self.time += 1;
        }
        let lg = self.time.ceil_log2();
        let n = 1 << lg;
        for t in &mut self.remove_time {
            if *t >= self.time {
                *t = n;
            }
        }

        let mut lines = (0..self.lines.len()).collect::<Vec<_>>();
        lines.sort_by_key(|&i| {
            let (a, b) = self.lines[i];
            (-a, b)
        });
        let mut points = (0..self.points.len()).collect::<Vec<_>>();
        points.sort_by_key(|&i| -self.points[i]);

        self.res.resize(self.points.len(), i64::inf());
        self.dfs(0, n, lines, points);
        self.res
    }

    fn eval(&self, i: usize, x: i64) -> i64 {
        let (a, b) = self.lines[i];
        a * x + b
    }

    fn f(&self, i: usize, j: usize) -> i64 {
        let (a, b) = self.lines[i];
        let (c, d) = self.lines[j];
        (d - b).floor_div(a - c)
    }

    fn check(&self, i: usize, j: usize, k: usize) -> bool {
        self.f(i, j) < self.f(j, k)
    }

    fn dfs(&mut self, l: usize, r: usize, lines: Vec<usize>, points: Vec<usize>) {
        if lines.is_empty() || points.is_empty() {
            return;
        }
        let m = (l + r) / 2;
        self.st.clear();

        let mut lines_l = vec![];
        let mut points_l = vec![];
        let mut lines_r = vec![];
        let mut points_r = vec![];
        for li in lines {
            let tl = self.add_time[li];
            let tr = self.remove_time[li];
            if tl <= l && r <= tr {
                if let Some(&lj) = self.st.last() {
                    if self.lines[lj].0 == self.lines[li].0 {
                        continue;
                    }
                }
                while let &[.., lk, lj] = &self.st[..] {
                    if self.check(lk, lj, li) {
                        break;
                    } else {
                        self.st.pop();
                    }
                }
                self.st.push(li);
                continue;
            }
            if l + 1 < r {
                if tl < m {
                    lines_l.push(li);
                }
                if m < tr {
                    lines_r.push(li);
                }
            }
        }

        for pi in points {
            let x = self.points[pi];
            while let &[.., li, lj] = &self.st[..] {
                if self.eval(li, x) <= self.eval(lj, x) {
                    self.st.pop();
                } else {
                    break;
                }
            }
            if let Some(&li) = self.st.last() {
                self.res[pi] = self.res[pi].min(self.eval(li, x));
            }
            if l + 1 < r {
                if self.get_time[pi] < m {
                    points_l.push(pi);
                } else {
                    points_r.push(pi);
                }
            }
        }
        if l + 1 < r {
            self.dfs(l, m, lines_l, points_l);
            self.dfs(m, r, lines_r, points_r);
        }
    }
}
