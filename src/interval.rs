use std::cmp::Ordering;
use std::ops::{Deref, Range};

/// Interval.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Interval<T>(pub(crate) Range<T>);

impl<T> Interval<T> {
    pub fn new(range: Range<T>) -> Self {
        Interval(range)
    }
}

impl<T> Deref for Interval<T> {
    type Target = Range<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct IntervalBeginSorted<T>(pub(crate) Interval<T>);

impl<T> IntervalBeginSorted<T> {
    pub(crate) fn new(interval: Interval<T>) -> Self {
        IntervalBeginSorted(interval)
    }
}

impl<T> Deref for IntervalBeginSorted<T> {
    type Target = Interval<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Ord> Ord for IntervalBeginSorted<T> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.start.cmp(&rhs.start)
    }
}

impl<T: Ord> PartialOrd for IntervalBeginSorted<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(&rhs))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct IntervalEndSorted<T>(pub(crate) Interval<T>);

impl<T> IntervalEndSorted<T> {
    pub(crate) fn new(interval: Interval<T>) -> Self {
        IntervalEndSorted(interval)
    }
}

impl<T> Deref for IntervalEndSorted<T> {
    type Target = Interval<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Ord> Ord for IntervalEndSorted<T> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        rhs.end.cmp(&self.end)
    }
}

impl<T: Ord> PartialOrd for IntervalEndSorted<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(&rhs))
    }
}
