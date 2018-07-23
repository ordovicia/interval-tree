use std::cmp::Ordering;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Interval<T> {
    pub(crate) begin: T,
    pub(crate) end: T,
}

impl<T: Ord> Interval<T> {
    pub fn new(begin: T, end: T) -> Self {
        Self { begin, end }
    }
}

#[derive(Debug, Eq)]
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
        self.begin.cmp(&rhs.begin)
    }
}

impl<T: Ord> PartialOrd for IntervalBeginSorted<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(&rhs))
    }
}

impl<T: PartialEq> PartialEq for IntervalBeginSorted<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.begin == rhs.begin
    }
}

#[derive(Debug, Eq)]
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

impl<T: PartialEq> PartialEq for IntervalEndSorted<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.end == rhs.end
    }
}
