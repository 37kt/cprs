// reference: https://tubo28.me/compprog/algorithm/flow_with_lu_bound/
// verify: https://atcoder.jp/contests/abc285/tasks/abc285_g

use max_flow::{FlowType, MaxFlow};

pub struct MaxFlowLowerBound {
    n: usize,
    mf: MaxFlow,
    sum_lower: FlowType,
}

impl MaxFlowLowerBound {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            mf: MaxFlow::new(n + 2),
            sum_lower: 0,
        }
    }

    // from->to に流量制約 [lower, upper] の辺を張る
    pub fn add_edge(&mut self, from: usize, to: usize, lower: FlowType, upper: FlowType) {
        assert!(from != to);
        assert!(0 <= lower && lower <= upper);
        let ss = self.n;
        let tt = self.n + 1;
        self.mf.add_edge(from, to, upper - lower);
        self.mf.add_edge(ss, to, lower);
        self.mf.add_edge(from, tt, lower);
        self.sum_lower += lower;
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> Option<FlowType> {
        let ss = self.n;
        let tt = self.n + 1;
        let a = self.mf.max_flow(ss, tt);
        let b = self.mf.max_flow(s, tt);
        let c = self.mf.max_flow(ss, t);
        let d = self.mf.max_flow(s, t);
        if a + c == self.sum_lower && a + b == self.sum_lower {
            Some(b + d)
        } else {
            None
        }
    }
}
