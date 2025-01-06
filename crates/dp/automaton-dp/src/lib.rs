use std::{cmp::Ordering, collections::HashMap};

pub trait DeterministicFiniteAutoMaton {
    type Symbol;
    type State;

    fn initial_state(&self) -> Self::State;
    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State>;
    fn is_accepting(&self, state: &Self::State) -> bool;
}

pub struct Intersection<A0, A1>(A0, A1)
where
    A0: DeterministicFiniteAutoMaton,
    A1: DeterministicFiniteAutoMaton<Symbol = A0::Symbol>;

impl<A0, A1> DeterministicFiniteAutoMaton for Intersection<A0, A1>
where
    A0: DeterministicFiniteAutoMaton,
    A1: DeterministicFiniteAutoMaton<Symbol = A0::Symbol>,
{
    type Symbol = A0::Symbol;
    type State = (A0::State, A1::State);

    fn initial_state(&self) -> Self::State {
        (self.0.initial_state(), self.1.initial_state())
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        let (st0, st1) = state;
        let st0 = self.0.transition(st0, c)?;
        let st1 = self.1.transition(st1, c)?;
        Some((st0, st1))
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        let (st0, st1) = state;
        self.0.is_accepting(st0) && self.1.is_accepting(st1)
    }
}

pub struct LessThanEqualDfa<'a, T>(&'a [T])
where
    T: Ord;

#[derive(PartialEq, Eq, Hash)]
pub struct LessThanEqualDfaState {
    i: usize,
    tight: bool,
}

impl<T> DeterministicFiniteAutoMaton for LessThanEqualDfa<'_, T>
where
    T: Ord,
{
    type Symbol = T;
    type State = LessThanEqualDfaState;

    fn initial_state(&self) -> Self::State {
        LessThanEqualDfaState { i: 0, tight: true }
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        assert!(state.i < self.0.len(), "state index out of bounds");
        match (state.tight, c.cmp(&self.0[state.i])) {
            (false, _) => Some(false),
            (true, Ordering::Less) => Some(false),
            (true, Ordering::Equal) => Some(true),
            (true, Ordering::Greater) => None,
        }
        .map(|tight| Self::State {
            i: state.i + 1,
            tight,
        })
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        state.i == self.0.len()
    }
}

pub struct LessThanDfa<'a, T>(&'a [T])
where
    T: Ord;

#[derive(PartialEq, Eq, Hash)]
pub struct LessThanDfaState {
    i: usize,
    tight: bool,
}

impl<T> DeterministicFiniteAutoMaton for LessThanDfa<'_, T>
where
    T: Ord,
{
    type Symbol = T;
    type State = LessThanDfaState;

    fn initial_state(&self) -> Self::State {
        LessThanDfaState { i: 0, tight: true }
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        assert!(state.i < self.0.len(), "state index out of bounds");
        match (state.tight, c.cmp(&self.0[state.i])) {
            (false, _) => Some(false),
            (true, Ordering::Less) => Some(false),
            (true, Ordering::Equal) => Some(true),
            (true, Ordering::Greater) => None,
        }
        .map(|tight| Self::State {
            i: state.i + 1,
            tight,
        })
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        state.i == self.0.len() && !state.tight
    }
}

pub struct GreaterThanEqualDfa<'a, T>(&'a [T])
where
    T: Ord;

#[derive(PartialEq, Eq, Hash)]
pub struct GreaterThanEqualDfaState {
    i: usize,
    tight: bool,
}

impl<T> DeterministicFiniteAutoMaton for GreaterThanEqualDfa<'_, T>
where
    T: Ord,
{
    type Symbol = T;
    type State = GreaterThanEqualDfaState;

    fn initial_state(&self) -> Self::State {
        GreaterThanEqualDfaState { i: 0, tight: true }
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        assert!(state.i < self.0.len(), "state index out of bounds");
        match (state.tight, c.cmp(&self.0[state.i])) {
            (false, _) => Some(false),
            (true, Ordering::Less) => None,
            (true, Ordering::Equal) => Some(true),
            (true, Ordering::Greater) => Some(false),
        }
        .map(|tight| Self::State {
            i: state.i + 1,
            tight,
        })
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        state.i == self.0.len()
    }
}

pub struct GreaterThanDfa<'a, T>(&'a [T])
where
    T: Ord;

#[derive(PartialEq, Eq, Hash)]
pub struct GreaterThanDfaState {
    i: usize,
    tight: bool,
}

impl<T> DeterministicFiniteAutoMaton for GreaterThanDfa<'_, T>
where
    T: Ord,
{
    type Symbol = T;
    type State = GreaterThanDfaState;

    fn initial_state(&self) -> Self::State {
        GreaterThanDfaState { i: 0, tight: true }
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        assert!(state.i < self.0.len(), "state index out of bounds");
        match (state.tight, c.cmp(&self.0[state.i])) {
            (false, _) => Some(false),
            (true, Ordering::Less) => None,
            (true, Ordering::Equal) => Some(true),
            (true, Ordering::Greater) => Some(false),
        }
        .map(|tight| Self::State {
            i: state.i + 1,
            tight,
        })
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        state.i == self.0.len() && !state.tight
    }
}

pub struct ReversedLessThanEqualDfa<'a, T>(&'a [T])
where
    T: Ord;

#[derive(PartialEq, Eq, Hash)]
pub struct ReversedLessThanEqualDfaState {
    i: usize,
    le: bool,
}

impl<T> DeterministicFiniteAutoMaton for ReversedLessThanEqualDfa<'_, T>
where
    T: Ord,
{
    type Symbol = T;
    type State = ReversedLessThanEqualDfaState;

    fn initial_state(&self) -> Self::State {
        ReversedLessThanEqualDfaState { i: 0, le: true }
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        assert!(state.i < self.0.len(), "state index out of bounds");
        Some(ReversedLessThanEqualDfaState {
            i: state.i + 1,
            le: match (state.le, c.cmp(&self.0[state.i])) {
                (_, Ordering::Less) => true,
                (_, Ordering::Greater) => false,
                (le, Ordering::Equal) => le,
            },
        })
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        state.i == self.0.len() && state.le
    }
}

pub struct ReversedLessThanDfa<'a, T>(&'a [T])
where
    T: Ord;

#[derive(PartialEq, Eq, Hash)]
pub struct ReversedLessThanDfaState {
    i: usize,
    lt: bool,
}

impl<T> DeterministicFiniteAutoMaton for ReversedLessThanDfa<'_, T>
where
    T: Ord,
{
    type Symbol = T;
    type State = ReversedLessThanDfaState;

    fn initial_state(&self) -> Self::State {
        ReversedLessThanDfaState { i: 0, lt: false }
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        assert!(state.i < self.0.len(), "state index out of bounds");
        Some(ReversedLessThanDfaState {
            i: state.i + 1,
            lt: match (state.lt, c.cmp(&self.0[state.i])) {
                (_, Ordering::Less) => true,
                (_, Ordering::Greater) => false,
                (lt, Ordering::Equal) => lt,
            },
        })
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        state.i == self.0.len() && state.lt
    }
}

pub struct ReversedGreaterThanEqualDfa<'a, T>(&'a [T])
where
    T: Ord;

#[derive(PartialEq, Eq, Hash)]
pub struct ReversedGreaterThanEqualDfaState {
    i: usize,
    ge: bool,
}

impl<T> DeterministicFiniteAutoMaton for ReversedGreaterThanEqualDfa<'_, T>
where
    T: Ord,
{
    type Symbol = T;
    type State = ReversedGreaterThanEqualDfaState;

    fn initial_state(&self) -> Self::State {
        ReversedGreaterThanEqualDfaState { i: 0, ge: true }
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        assert!(state.i < self.0.len(), "state index out of bounds");
        Some(ReversedGreaterThanEqualDfaState {
            i: state.i + 1,
            ge: match (state.ge, c.cmp(&self.0[state.i])) {
                (_, Ordering::Less) => false,
                (_, Ordering::Greater) => true,
                (ge, Ordering::Equal) => ge,
            },
        })
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        state.i == self.0.len() && state.ge
    }
}

pub struct ReversedGreaterThanDfa<'a, T>(&'a [T])
where
    T: Ord;

#[derive(PartialEq, Eq, Hash)]
pub struct ReversedGreaterThanDfaState {
    i: usize,
    gt: bool,
}

impl<T> DeterministicFiniteAutoMaton for ReversedGreaterThanDfa<'_, T>
where
    T: Ord,
{
    type Symbol = T;
    type State = ReversedGreaterThanDfaState;

    fn initial_state(&self) -> Self::State {
        ReversedGreaterThanDfaState { i: 0, gt: false }
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        assert!(state.i < self.0.len(), "state index out of bounds");
        Some(ReversedGreaterThanDfaState {
            i: state.i + 1,
            gt: match (state.gt, c.cmp(&self.0[state.i])) {
                (_, Ordering::Less) => false,
                (_, Ordering::Greater) => true,
                (gt, Ordering::Equal) => gt,
            },
        })
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        state.i == self.0.len() && state.gt
    }
}

pub fn automaton_dp<A, T>(
    dfa: A,
    alphabet: impl Iterator<Item = A::Symbol> + Clone,
    len: usize,
    mut act: impl FnMut(&A::Symbol, &T) -> T,
    mut op: impl FnMut(&T, &T) -> T,
    e: T,
    dp_initial: T,
) -> T
where
    A: DeterministicFiniteAutoMaton,
    A::State: Eq + std::hash::Hash,
    T: Clone,
{
    let mut dp = HashMap::<A::State, T>::new();
    let mut ndp = HashMap::<A::State, T>::new();
    dp.insert(dfa.initial_state(), dp_initial);

    for _ in 0..len {
        for (st, val) in dp.drain() {
            for c in alphabet.clone() {
                if let Some(nst) = dfa.transition(&st, &c) {
                    let nval = act(&c, &val);
                    ndp.entry(nst)
                        .and_modify(|sum| *sum = op(sum, &nval))
                        .or_insert(nval);
                }
            }
        }
        std::mem::swap(&mut dp, &mut ndp);
    }

    dp.into_iter()
        .filter(|(st, _)| dfa.is_accepting(st))
        .fold(e, |acc, (_, val)| op(&acc, &val))
}
