use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::{Deref, Range};

/// Interval.
pub trait Interval: Clone + Eq + Hash + Iterator {
    fn begin(&self) -> Self::Item;
    fn end(&self) -> Self::Item;

    fn center(&self) -> Self::Item;

    fn left_half(&self) -> Self;
    fn right_half(&self) -> Self;

    fn to_begin_sorted(&self) -> BeginSorted<Self>;
    fn to_end_sorted(&self) -> EndSorted<Self>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct BeginSorted<T: Interval>(T);

#[derive(Debug, PartialEq, Eq)]
pub struct EndSorted<T: Interval>(T);

impl<T: Interval> BeginSorted<T> {
    pub(crate) fn to_interval(&self) -> T {
        self.0.clone()
    }
}

impl<T: Interval> EndSorted<T> {
    pub(crate) fn to_interval(&self) -> T {
        self.0.clone()
    }
}

impl<T: Interval> Deref for BeginSorted<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Interval> Deref for EndSorted<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! impl_interval_for_range {
    ($int:ty) => {
        impl Interval for Range<$int> {
            fn begin(&self) -> Self::Item {
                self.start
            }

            fn end(&self) -> Self::Item {
                self.end
            }

            fn center(&self) -> Self::Item {
                (self.start + self.end) / 2
            }

            fn left_half(&self) -> Self {
                self.begin()..self.center()
            }

            fn right_half(&self) -> Self {
                self.center()..self.end()
            }

            fn to_begin_sorted(&self) -> BeginSorted<Self> {
                BeginSorted(self.clone())
            }

            fn to_end_sorted(&self) -> EndSorted<Self> {
                EndSorted(self.clone())
            }
        }

        impl Ord for BeginSorted<Range<$int>> {
            fn cmp(&self, rhs: &Self) -> Ordering {
                self.start.cmp(&rhs.start)
            }
        }

        impl PartialOrd for BeginSorted<Range<$int>> {
            fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
                Some(self.cmp(&rhs))
            }
        }

        impl Ord for EndSorted<Range<$int>> {
            fn cmp(&self, rhs: &Self) -> Ordering {
                rhs.end.cmp(&self.end)
            }
        }

        impl PartialOrd for EndSorted<Range<$int>> {
            fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
                Some(self.cmp(&rhs))
            }
        }
    };
}

impl_interval_for_range!(u8);
impl_interval_for_range!(i8);
impl_interval_for_range!(u16);
impl_interval_for_range!(i16);
impl_interval_for_range!(u32);
impl_interval_for_range!(i32);
impl_interval_for_range!(u64);
impl_interval_for_range!(i64);
impl_interval_for_range!(u128);
impl_interval_for_range!(i128);
impl_interval_for_range!(usize);
impl_interval_for_range!(isize);
