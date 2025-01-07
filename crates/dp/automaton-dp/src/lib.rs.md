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
    \ a1)\n    }\n}\n\n#[derive(Clone, Copy)]\npub struct LessThanEqualDfa<'a, T>(pub\
    \ &'a [T])\nwhere\n    T: Ord;\n\nimpl<'a, T> LessThanEqualDfa<'a, T>\nwhere\n\
    \    T: Ord,\n{\n    pub fn new(a: &'a [T]) -> Self {\n        Self(a)\n    }\n\
    }\n\n#[derive(Clone, Copy, PartialEq, Eq, Hash)]\npub struct LessThanEqualDfaState\
    \ {\n    i: usize,\n    tight: bool,\n}\n\nimpl<T> DeterministicFiniteAutoMaton\
    \ for LessThanEqualDfa<'_, T>\nwhere\n    T: Ord,\n{\n    type Symbol = T;\n \
    \   type State = LessThanEqualDfaState;\n\n    fn initial_state(&self) -> Self::State\
    \ {\n        LessThanEqualDfaState { i: 0, tight: true }\n    }\n\n    fn transition(&self,\
    \ state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {\n        assert!(state.i\
    \ < self.0.len(), \"state index out of bounds\");\n        match (state.tight,\
    \ c.cmp(&self.0[state.i])) {\n            (false, _) => Some(false),\n       \
    \     (true, Ordering::Less) => Some(false),\n            (true, Ordering::Equal)\
    \ => Some(true),\n            (true, Ordering::Greater) => None,\n        }\n\
    \        .map(|tight| Self::State {\n            i: state.i + 1,\n           \
    \ tight,\n        })\n    }\n\n    fn is_accepting(&self, state: &Self::State)\
    \ -> bool {\n        state.i == self.0.len()\n    }\n}\n\n#[derive(Clone, Copy)]\n\
    pub struct LessThanDfa<'a, T>(pub &'a [T])\nwhere\n    T: Ord;\n\nimpl<'a, T>\
    \ LessThanDfa<'a, T>\nwhere\n    T: Ord,\n{\n    pub fn new(a: &'a [T]) -> Self\
    \ {\n        Self(a)\n    }\n}\n\n#[derive(Clone, Copy, PartialEq, Eq, Hash)]\n\
    pub struct LessThanDfaState {\n    i: usize,\n    tight: bool,\n}\n\nimpl<T> DeterministicFiniteAutoMaton\
    \ for LessThanDfa<'_, T>\nwhere\n    T: Ord,\n{\n    type Symbol = T;\n    type\
    \ State = LessThanDfaState;\n\n    fn initial_state(&self) -> Self::State {\n\
    \        LessThanDfaState { i: 0, tight: true }\n    }\n\n    fn transition(&self,\
    \ state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {\n        assert!(state.i\
    \ < self.0.len(), \"state index out of bounds\");\n        match (state.tight,\
    \ c.cmp(&self.0[state.i])) {\n            (false, _) => Some(false),\n       \
    \     (true, Ordering::Less) => Some(false),\n            (true, Ordering::Equal)\
    \ => Some(true),\n            (true, Ordering::Greater) => None,\n        }\n\
    \        .map(|tight| Self::State {\n            i: state.i + 1,\n           \
    \ tight,\n        })\n    }\n\n    fn is_accepting(&self, state: &Self::State)\
    \ -> bool {\n        state.i == self.0.len() && !state.tight\n    }\n}\n\n#[derive(Clone,\
    \ Copy)]\npub struct GreaterThanEqualDfa<'a, T>(pub &'a [T])\nwhere\n    T: Ord;\n\
    \nimpl<'a, T> GreaterThanEqualDfa<'a, T>\nwhere\n    T: Ord,\n{\n    pub fn new(a:\
    \ &'a [T]) -> Self {\n        Self(a)\n    }\n}\n\n#[derive(Clone, Copy, PartialEq,\
    \ Eq, Hash)]\npub struct GreaterThanEqualDfaState {\n    i: usize,\n    tight:\
    \ bool,\n}\n\nimpl<T> DeterministicFiniteAutoMaton for GreaterThanEqualDfa<'_,\
    \ T>\nwhere\n    T: Ord,\n{\n    type Symbol = T;\n    type State = GreaterThanEqualDfaState;\n\
    \n    fn initial_state(&self) -> Self::State {\n        GreaterThanEqualDfaState\
    \ { i: 0, tight: true }\n    }\n\n    fn transition(&self, state: &Self::State,\
    \ c: &Self::Symbol) -> Option<Self::State> {\n        assert!(state.i < self.0.len(),\
    \ \"state index out of bounds\");\n        match (state.tight, c.cmp(&self.0[state.i]))\
    \ {\n            (false, _) => Some(false),\n            (true, Ordering::Less)\
    \ => None,\n            (true, Ordering::Equal) => Some(true),\n            (true,\
    \ Ordering::Greater) => Some(false),\n        }\n        .map(|tight| Self::State\
    \ {\n            i: state.i + 1,\n            tight,\n        })\n    }\n\n  \
    \  fn is_accepting(&self, state: &Self::State) -> bool {\n        state.i == self.0.len()\n\
    \    }\n}\n\n#[derive(Clone, Copy)]\npub struct GreaterThanDfa<'a, T>(pub &'a\
    \ [T])\nwhere\n    T: Ord;\n\nimpl<'a, T> GreaterThanDfa<'a, T>\nwhere\n    T:\
    \ Ord,\n{\n    pub fn new(a: &'a [T]) -> Self {\n        Self(a)\n    }\n}\n\n\
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]\npub struct GreaterThanDfaState {\n\
    \    i: usize,\n    tight: bool,\n}\n\nimpl<T> DeterministicFiniteAutoMaton for\
    \ GreaterThanDfa<'_, T>\nwhere\n    T: Ord,\n{\n    type Symbol = T;\n    type\
    \ State = GreaterThanDfaState;\n\n    fn initial_state(&self) -> Self::State {\n\
    \        GreaterThanDfaState { i: 0, tight: true }\n    }\n\n    fn transition(&self,\
    \ state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {\n        assert!(state.i\
    \ < self.0.len(), \"state index out of bounds\");\n        match (state.tight,\
    \ c.cmp(&self.0[state.i])) {\n            (false, _) => Some(false),\n       \
    \     (true, Ordering::Less) => None,\n            (true, Ordering::Equal) =>\
    \ Some(true),\n            (true, Ordering::Greater) => Some(false),\n       \
    \ }\n        .map(|tight| Self::State {\n            i: state.i + 1,\n       \
    \     tight,\n        })\n    }\n\n    fn is_accepting(&self, state: &Self::State)\
    \ -> bool {\n        state.i == self.0.len() && !state.tight\n    }\n}\n\n#[derive(Clone,\
    \ Copy)]\npub struct ReversedLessThanEqualDfa<'a, T>(pub &'a [T])\nwhere\n   \
    \ T: Ord;\n\nimpl<'a, T> ReversedLessThanEqualDfa<'a, T>\nwhere\n    T: Ord,\n\
    {\n    pub fn new(a: &'a [T]) -> Self {\n        Self(a)\n    }\n}\n\n#[derive(Clone,\
    \ Copy, PartialEq, Eq, Hash)]\npub struct ReversedLessThanEqualDfaState {\n  \
    \  i: usize,\n    le: bool,\n}\n\nimpl<T> DeterministicFiniteAutoMaton for ReversedLessThanEqualDfa<'_,\
    \ T>\nwhere\n    T: Ord,\n{\n    type Symbol = T;\n    type State = ReversedLessThanEqualDfaState;\n\
    \n    fn initial_state(&self) -> Self::State {\n        ReversedLessThanEqualDfaState\
    \ { i: 0, le: true }\n    }\n\n    fn transition(&self, state: &Self::State, c:\
    \ &Self::Symbol) -> Option<Self::State> {\n        assert!(state.i < self.0.len(),\
    \ \"state index out of bounds\");\n        Some(ReversedLessThanEqualDfaState\
    \ {\n            i: state.i + 1,\n            le: match (state.le, c.cmp(&self.0[state.i]))\
    \ {\n                (_, Ordering::Less) => true,\n                (_, Ordering::Greater)\
    \ => false,\n                (le, Ordering::Equal) => le,\n            },\n  \
    \      })\n    }\n\n    fn is_accepting(&self, state: &Self::State) -> bool {\n\
    \        state.i == self.0.len() && state.le\n    }\n}\n\n#[derive(Clone, Copy)]\n\
    pub struct ReversedLessThanDfa<'a, T>(pub &'a [T])\nwhere\n    T: Ord;\n\nimpl<'a,\
    \ T> ReversedLessThanDfa<'a, T>\nwhere\n    T: Ord,\n{\n    pub fn new(a: &'a\
    \ [T]) -> Self {\n        Self(a)\n    }\n}\n\n#[derive(Clone, Copy, PartialEq,\
    \ Eq, Hash)]\npub struct ReversedLessThanDfaState {\n    i: usize,\n    lt: bool,\n\
    }\n\nimpl<T> DeterministicFiniteAutoMaton for ReversedLessThanDfa<'_, T>\nwhere\n\
    \    T: Ord,\n{\n    type Symbol = T;\n    type State = ReversedLessThanDfaState;\n\
    \n    fn initial_state(&self) -> Self::State {\n        ReversedLessThanDfaState\
    \ { i: 0, lt: false }\n    }\n\n    fn transition(&self, state: &Self::State,\
    \ c: &Self::Symbol) -> Option<Self::State> {\n        assert!(state.i < self.0.len(),\
    \ \"state index out of bounds\");\n        Some(ReversedLessThanDfaState {\n \
    \           i: state.i + 1,\n            lt: match (state.lt, c.cmp(&self.0[state.i]))\
    \ {\n                (_, Ordering::Less) => true,\n                (_, Ordering::Greater)\
    \ => false,\n                (lt, Ordering::Equal) => lt,\n            },\n  \
    \      })\n    }\n\n    fn is_accepting(&self, state: &Self::State) -> bool {\n\
    \        state.i == self.0.len() && state.lt\n    }\n}\n\n#[derive(Clone, Copy)]\n\
    pub struct ReversedGreaterThanEqualDfa<'a, T>(pub &'a [T])\nwhere\n    T: Ord;\n\
    \nimpl<'a, T> ReversedGreaterThanEqualDfa<'a, T>\nwhere\n    T: Ord,\n{\n    pub\
    \ fn new(a: &'a [T]) -> Self {\n        Self(a)\n    }\n}\n\n#[derive(Clone, Copy,\
    \ PartialEq, Eq, Hash)]\npub struct ReversedGreaterThanEqualDfaState {\n    i:\
    \ usize,\n    ge: bool,\n}\n\nimpl<T> DeterministicFiniteAutoMaton for ReversedGreaterThanEqualDfa<'_,\
    \ T>\nwhere\n    T: Ord,\n{\n    type Symbol = T;\n    type State = ReversedGreaterThanEqualDfaState;\n\
    \n    fn initial_state(&self) -> Self::State {\n        ReversedGreaterThanEqualDfaState\
    \ { i: 0, ge: true }\n    }\n\n    fn transition(&self, state: &Self::State, c:\
    \ &Self::Symbol) -> Option<Self::State> {\n        assert!(state.i < self.0.len(),\
    \ \"state index out of bounds\");\n        Some(ReversedGreaterThanEqualDfaState\
    \ {\n            i: state.i + 1,\n            ge: match (state.ge, c.cmp(&self.0[state.i]))\
    \ {\n                (_, Ordering::Less) => false,\n                (_, Ordering::Greater)\
    \ => true,\n                (ge, Ordering::Equal) => ge,\n            },\n   \
    \     })\n    }\n\n    fn is_accepting(&self, state: &Self::State) -> bool {\n\
    \        state.i == self.0.len() && state.ge\n    }\n}\n\n#[derive(Clone, Copy)]\n\
    pub struct ReversedGreaterThanDfa<'a, T>(pub &'a [T])\nwhere\n    T: Ord;\n\n\
    impl<'a, T> ReversedGreaterThanDfa<'a, T>\nwhere\n    T: Ord,\n{\n    pub fn new(a:\
    \ &'a [T]) -> Self {\n        Self(a)\n    }\n}\n\n#[derive(Clone, Copy, PartialEq,\
    \ Eq, Hash)]\npub struct ReversedGreaterThanDfaState {\n    i: usize,\n    gt:\
    \ bool,\n}\n\nimpl<T> DeterministicFiniteAutoMaton for ReversedGreaterThanDfa<'_,\
    \ T>\nwhere\n    T: Ord,\n{\n    type Symbol = T;\n    type State = ReversedGreaterThanDfaState;\n\
    \n    fn initial_state(&self) -> Self::State {\n        ReversedGreaterThanDfaState\
    \ { i: 0, gt: false }\n    }\n\n    fn transition(&self, state: &Self::State,\
    \ c: &Self::Symbol) -> Option<Self::State> {\n        assert!(state.i < self.0.len(),\
    \ \"state index out of bounds\");\n        Some(ReversedGreaterThanDfaState {\n\
    \            i: state.i + 1,\n            gt: match (state.gt, c.cmp(&self.0[state.i]))\
    \ {\n                (_, Ordering::Less) => false,\n                (_, Ordering::Greater)\
    \ => true,\n                (gt, Ordering::Equal) => gt,\n            },\n   \
    \     })\n    }\n\n    fn is_accepting(&self, state: &Self::State) -> bool {\n\
    \        state.i == self.0.len() && state.gt\n    }\n}\n\n#[derive(Clone, Copy)]\n\
    pub struct SymbolMap<A, S, F>\nwhere\n    A: DeterministicFiniteAutoMaton,\n \
    \   F: Fn(&S) -> A::Symbol,\n{\n    dfa: A,\n    map: F,\n    _marker: PhantomData<fn()\
    \ -> S>,\n}\n\nimpl<A, S, F> SymbolMap<A, S, F>\nwhere\n    A: DeterministicFiniteAutoMaton<Symbol\
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
  timestamp: '2025-01-07 06:51:23+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/dp/automaton-dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/dp/automaton-dp/src/lib.rs
- /library/crates/dp/automaton-dp/src/lib.rs.html
title: crates/dp/automaton-dp/src/lib.rs
---
