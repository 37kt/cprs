---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{cmp::Ordering, collections::HashMap, marker::PhantomData};\n\n\
    pub trait DeterministicFiniteAutoMaton {\n    type Symbol;\n    type State;\n\n\
    \    fn initial_state(&self) -> Self::State;\n    fn transition(&self, state:\
    \ &Self::State, c: &Self::Symbol) -> Option<Self::State>;\n    fn is_accepting(&self,\
    \ state: &Self::State) -> bool;\n}\n\n#[derive(Clone, Copy)]\npub struct Intersection<A0,\
    \ A1>(pub A0, pub A1)\nwhere\n    A0: DeterministicFiniteAutoMaton,\n    A1: DeterministicFiniteAutoMaton<Symbol\
    \ = A0::Symbol>;\n\nimpl<A0, A1> DeterministicFiniteAutoMaton for Intersection<A0,\
    \ A1>\nwhere\n    A0: DeterministicFiniteAutoMaton,\n    A1: DeterministicFiniteAutoMaton<Symbol\
    \ = A0::Symbol>,\n{\n    type Symbol = A0::Symbol;\n    type State = (A0::State,\
    \ A1::State);\n\n    fn initial_state(&self) -> Self::State {\n        (self.0.initial_state(),\
    \ self.1.initial_state())\n    }\n\n    fn transition(&self, state: &Self::State,\
    \ c: &Self::Symbol) -> Option<Self::State> {\n        let (st0, st1) = state;\n\
    \        let st0 = self.0.transition(st0, c)?;\n        let st1 = self.1.transition(st1,\
    \ c)?;\n        Some((st0, st1))\n    }\n\n    fn is_accepting(&self, state: &Self::State)\
    \ -> bool {\n        let (st0, st1) = state;\n        self.0.is_accepting(st0)\
    \ && self.1.is_accepting(st1)\n    }\n}\n\nimpl<A0, A1> Intersection<A0, A1>\n\
    where\n    A0: DeterministicFiniteAutoMaton,\n    A1: DeterministicFiniteAutoMaton<Symbol\
    \ = A0::Symbol>,\n{\n    pub fn new(a0: A0, a1: A1) -> Self {\n        Self(a0,\
    \ a1)\n    }\n}\n\n#[derive(Clone, Copy)]\npub struct LexicographicalDfa<'a, T>\
    \ {\n    seq: &'a [T],\n    ord: Ordering,\n    eq: bool,\n}\n\nimpl<'a, T> DeterministicFiniteAutoMaton\
    \ for LexicographicalDfa<'a, T>\nwhere\n    T: Ord,\n{\n    type Symbol = T;\n\
    \    type State = (usize, bool);\n\n    fn initial_state(&self) -> Self::State\
    \ {\n        (0, true)\n    }\n\n    fn transition(&self, state: &Self::State,\
    \ c: &Self::Symbol) -> Option<Self::State> {\n        let &(i, tight) = state;\n\
    \        let x = self.seq.get(i)?;\n        match (tight, c.cmp(x)) {\n      \
    \      (true, Ordering::Equal) => Some((i + 1, true)),\n            (true, ord)\
    \ if ord == self.ord.reverse() => None,\n            _ => Some((i + 1, false)),\n\
    \        }\n    }\n\n    fn is_accepting(&self, state: &Self::State) -> bool {\n\
    \        self.eq || !state.1\n    }\n}\n\nimpl<'a, T> LexicographicalDfa<'a, T>\n\
    where\n    T: Ord,\n{\n    pub fn less_than(seq: &'a [T]) -> Self {\n        Self\
    \ {\n            seq,\n            ord: Ordering::Less,\n            eq: false,\n\
    \        }\n    }\n\n    pub fn less_than_equal(seq: &'a [T]) -> Self {\n    \
    \    Self {\n            seq,\n            ord: Ordering::Less,\n            eq:\
    \ true,\n        }\n    }\n\n    pub fn greater_than(seq: &'a [T]) -> Self {\n\
    \        Self {\n            seq,\n            ord: Ordering::Greater,\n     \
    \       eq: false,\n        }\n    }\n\n    pub fn greater_than_equal(seq: &'a\
    \ [T]) -> Self {\n        Self {\n            seq,\n            ord: Ordering::Greater,\n\
    \            eq: true,\n        }\n    }\n}\n\n#[derive(Clone, Copy)]\npub struct\
    \ ReversedLexicographicalDfa<'a, T> {\n    seq: &'a [T],\n    ord: Ordering,\n\
    \    eq: bool,\n}\n\nimpl<'a, T> DeterministicFiniteAutoMaton for ReversedLexicographicalDfa<'a,\
    \ T>\nwhere\n    T: Ord,\n{\n    type Symbol = T;\n    type State = (usize, bool);\n\
    \n    fn initial_state(&self) -> Self::State {\n        (self.seq.len(), self.eq)\n\
    \    }\n\n    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State>\
    \ {\n        let &(i, accepting) = state;\n        let i = i.checked_sub(1)?;\n\
    \        let x = self.seq.get(i)?;\n        match c.cmp(x) {\n            Ordering::Equal\
    \ => Some((i, accepting)),\n            ord => Some((i, ord == self.ord)),\n \
    \       }\n    }\n\n    fn is_accepting(&self, state: &Self::State) -> bool {\n\
    \        state.1\n    }\n}\n\nimpl<'a, T> ReversedLexicographicalDfa<'a, T>\n\
    where\n    T: Ord,\n{\n    pub fn less_than(seq: &'a [T]) -> Self {\n        Self\
    \ {\n            seq,\n            ord: Ordering::Less,\n            eq: false,\n\
    \        }\n    }\n\n    pub fn less_than_equal(seq: &'a [T]) -> Self {\n    \
    \    Self {\n            seq,\n            ord: Ordering::Less,\n            eq:\
    \ true,\n        }\n    }\n\n    pub fn greater_than(seq: &'a [T]) -> Self {\n\
    \        Self {\n            seq,\n            ord: Ordering::Greater,\n     \
    \       eq: false,\n        }\n    }\n\n    pub fn greater_than_equal(seq: &'a\
    \ [T]) -> Self {\n        Self {\n            seq,\n            ord: Ordering::Greater,\n\
    \            eq: true,\n        }\n    }\n}\n\n#[derive(Clone, Copy)]\npub struct\
    \ SymbolMap<A, S, F>\nwhere\n    A: DeterministicFiniteAutoMaton,\n    F: Fn(&S)\
    \ -> A::Symbol,\n{\n    dfa: A,\n    map: F,\n    _marker: PhantomData<fn() ->\
    \ S>,\n}\n\nimpl<A, S, F> SymbolMap<A, S, F>\nwhere\n    A: DeterministicFiniteAutoMaton<Symbol\
    \ = S>,\n    F: Fn(&S) -> A::Symbol,\n{\n    pub fn new(dfa: A, map: F) -> Self\
    \ {\n        Self {\n            dfa,\n            map,\n            _marker:\
    \ PhantomData,\n        }\n    }\n}\n\nimpl<A, S, F> DeterministicFiniteAutoMaton\
    \ for SymbolMap<A, S, F>\nwhere\n    A: DeterministicFiniteAutoMaton,\n    F:\
    \ Fn(&S) -> A::Symbol,\n{\n    type Symbol = S;\n    type State = A::State;\n\n\
    \    fn initial_state(&self) -> Self::State {\n        self.dfa.initial_state()\n\
    \    }\n\n    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State>\
    \ {\n        self.dfa.transition(state, &(self.map)(c))\n    }\n\n    fn is_accepting(&self,\
    \ state: &Self::State) -> bool {\n        self.dfa.is_accepting(state)\n    }\n\
    }\n\npub fn automaton_dp<A, T>(\n    dfa: A,\n    alphabet: impl Iterator<Item\
    \ = A::Symbol> + Clone,\n    len: usize,\n    mut act: impl FnMut(&A::Symbol,\
    \ &T) -> T,\n    mut op: impl FnMut(&T, &T) -> T,\n    e: T,\n    dp_initial:\
    \ T,\n) -> T\nwhere\n    A: DeterministicFiniteAutoMaton,\n    A::State: Eq +\
    \ std::hash::Hash,\n    T: Clone,\n{\n    let mut dp = HashMap::<A::State, T>::new();\n\
    \    let mut ndp = HashMap::<A::State, T>::new();\n    dp.insert(dfa.initial_state(),\
    \ dp_initial);\n\n    for _ in 0..len {\n        for (st, val) in dp.drain() {\n\
    \            for c in alphabet.clone() {\n                if let Some(nst) = dfa.transition(&st,\
    \ &c) {\n                    let nval = act(&c, &val);\n                    ndp.entry(nst)\n\
    \                        .and_modify(|sum| *sum = op(sum, &nval))\n          \
    \              .or_insert(nval);\n                }\n            }\n        }\n\
    \        std::mem::swap(&mut dp, &mut ndp);\n    }\n\n    dp.into_iter()\n   \
    \     .filter(|(st, _)| dfa.is_accepting(st))\n        .fold(e, |acc, (_, val)|\
    \ op(&acc, &val))\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/dp/automaton-dp/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-09 08:31:18+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/dp/automaton-dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/dp/automaton-dp/src/lib.rs
- /library/crates/dp/automaton-dp/src/lib.rs.html
title: crates/dp/automaton-dp/src/lib.rs
---
