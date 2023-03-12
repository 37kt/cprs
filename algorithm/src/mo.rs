pub trait Mo {
    type Output: Default + Clone;

    #[allow(unused_variables)]
    fn add(&mut self, i: usize) {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn remove(&mut self, i: usize) {
        unimplemented!()
    }

    fn add_left(&mut self, i: usize) {
        self.add(i);
    }

    fn add_right(&mut self, i: usize) {
        self.add(i);
    }

    fn remove_left(&mut self, i: usize) {
        self.remove(i);
    }

    fn remove_right(&mut self, i: usize) {
        self.remove(i);
    }

    fn query(&self) -> Self::Output;

    fn solve(&mut self, qs: &[(usize, usize)]) -> Vec<Self::Output> {
        let n = qs.iter().map(|&(_, r)| r).max().unwrap();
        let q = qs.len();
        let w = n / n.min((q as f64).sqrt().round() as usize);
        let mut ord = (0..q).collect::<Vec<_>>();
        ord.sort_by_key(|&i| {
            let (l, r) = qs[i];
            (l / w, r)
        });
        let mut l = 0;
        let mut r = 0;

        let mut res = vec![Default::default(); q];
        for i in ord {
            let (ll, rr) = qs[i];
            while l > ll {
                l -= 1;
                self.add_left(l);
            }
            while r < rr {
                self.add_right(r);
                r += 1;
            }
            while l < ll {
                self.remove_left(l);
                l += 1;
            }
            while r > rr {
                r -= 1;
                self.remove_right(r);
            }
            res[i] = self.query();
        }
        res
    }
}
