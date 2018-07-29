use std::cmp::PartialOrd;
use std::collections::{BinaryHeap, HashSet};

use interval::{BeginSorted, EndSorted};
use Interval;

/// Interval tree.
#[derive(Debug)]
pub struct IntervalTree<T>
where
    T: Interval,
    BeginSorted<T>: Ord,
    EndSorted<T>: Ord,
{
    range: T,
    center: T::Item,

    left: Option<Box<IntervalTree<T>>>,
    right: Option<Box<IntervalTree<T>>>,

    overlaps_begin: BinaryHeap<BeginSorted<T>>,
    overlaps_end: BinaryHeap<EndSorted<T>>,
}

impl<T> IntervalTree<T>
where
    T: Interval,
    <T as Iterator>::Item: PartialOrd,
    BeginSorted<T>: Ord,
    EndSorted<T>: Ord,
{
    /// Creates a interval tree on `range`.
    pub fn new(range: T) -> Self {
        let center = range.center();

        Self {
            range,
            center,

            left: None,
            right: None,

            overlaps_begin: BinaryHeap::new(),
            overlaps_end: BinaryHeap::new(),
        }
    }

    /// Inserts an [`Interval`](trait.Interval.html) to this interval tree.
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate interval_tree;
    /// use interval_tree::{Interval, IntervalTree};
    ///
    /// let mut tree = IntervalTree::new(0..100);
    ///
    /// tree.insert(5..10);
    /// tree.insert(85..95);
    /// ```
    ///
    /// # Panic
    ///
    /// Panics if the interval overflows the range of this interval tree.
    pub fn insert(&mut self, interval: T) {
        assert!(!self.overflow_interval(&interval));

        if interval.end() <= self.center {
            if self.left.is_none() {
                let range = self.range.left_half();
                self.left = Some(Box::new(IntervalTree::new(range)));
            }

            self.left.as_mut().unwrap().insert(interval);
        } else if interval.begin() > self.center {
            if self.right.is_none() {
                let range = self.range.right_half();
                self.right = Some(Box::new(IntervalTree::new(range)));
            }

            self.right.as_mut().unwrap().insert(interval);
        } else {
            self.overlaps_begin.push(interval.to_begin_sorted());
            self.overlaps_end.push(interval.to_end_sorted());
        }
    }

    /// Finds [`Interval`](trait.Interval.html)s in this interval tree that contain the `point`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate interval_tree;
    ///
    /// use std::collections::HashSet;
    /// use interval_tree::{Interval, IntervalTree};
    ///
    /// let mut tree = IntervalTree::new(0..100);
    ///
    /// tree.insert(5..10);
    /// tree.insert(85..95);
    /// tree.insert(90..100);
    ///
    /// assert_eq!(tree.find_with_point(0), HashSet::new());
    ///
    /// let intervals = [&(5..10)].iter().cloned().collect();
    /// assert_eq!(tree.find_with_point(5), intervals);
    ///
    /// let intervals = [&(85..95), &(90..100)].iter().cloned().collect();
    /// assert_eq!(tree.find_with_point(90), intervals);
    /// ```
    ///
    /// # Panic
    ///
    /// Panics if the point is out-of-range of this interval tree.
    pub fn find_with_point(&self, point: T::Item) -> HashSet<&T> {
        assert!(!self.overflow_point(&point));

        let mut found = HashSet::new();
        self.find_with_point_rec(point, &mut found);
        found
    }

    fn find_with_point_rec<'a, 'b>(&'a self, point: T::Item, found: &'b mut HashSet<&'a T>) {
        if point < self.center {
            for intv in self.overlaps_begin
                .iter()
                .filter(|&intv| intv.begin() <= point)
            {
                found.insert(&intv);
            }

            if let Some(ref left) = self.left {
                left.find_with_point_rec(point, found);
            }
        } else {
            for intv in self.overlaps_end.iter().filter(|intv| intv.end() > point) {
                found.insert(&intv);
            }

            if let Some(ref right) = self.right {
                right.find_with_point_rec(point, found);
            }
        }
    }

    /// Finds [`Interval`](trait.Interval.html)s in this interval tree that overlap with
    /// `interval`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate interval_tree;
    ///
    /// use std::collections::HashSet;
    /// use interval_tree::{Interval, IntervalTree};
    ///
    /// let mut tree = IntervalTree::new(0..100);
    ///
    /// tree.insert(5..10);
    /// tree.insert(85..95);
    /// tree.insert(90..100);
    ///
    /// assert_eq!(tree.find_with_interval(0..5), HashSet::new());
    ///
    /// let intervals = [&(5..10)].iter().cloned().collect();
    /// assert_eq!(tree.find_with_interval(3..8), intervals);
    ///
    /// let intervals = [&(85..95), &(90..100)].iter().cloned().collect();
    /// assert_eq!(tree.find_with_interval(80..95), intervals);
    ///
    /// let intervals = [&(90..100)].iter().cloned().collect();
    /// assert_eq!(tree.find_with_interval(95..100), intervals);
    /// ```
    ///
    /// # Panic
    ///
    /// Panics if the interval is out-of-range of this interval tree.
    pub fn find_with_interval(&self, interval: T) -> HashSet<&T> {
        assert!(!self.overflow_interval(&interval));

        let mut found = HashSet::new();
        for p in interval {
            for intv in self.find_with_point(p) {
                found.insert(intv);
            }
        }

        found
    }

    fn overflow_interval(&self, interval: &T) -> bool {
        interval.begin() < self.range.begin() || interval.end() > self.range.end()
    }

    fn overflow_point(&self, point: &T::Item) -> bool {
        point < &self.range.begin() || point >= &self.range.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn panic_insert_begin() {
        let mut tree = IntervalTree::new(1..11);
        tree.insert(0..10);
    }

    #[test]
    #[should_panic]
    fn panic_insert_end() {
        let mut tree = IntervalTree::new(0..10);
        tree.insert(1..11);
    }

    #[test]
    #[should_panic]
    fn panic_find_with_point_begin() {
        let tree = IntervalTree::new(1..11);
        tree.find_with_point(0);
    }

    #[test]
    #[should_panic]
    fn panic_find_with_point_end() {
        let tree = IntervalTree::new(0..10);
        tree.find_with_point(10);
    }

    #[test]
    #[should_panic]
    fn panic_find_with_interval_start() {
        let tree = IntervalTree::new(1..11);
        tree.find_with_interval(0..10);
    }

    #[test]
    #[should_panic]
    fn panic_find_with_interval_end() {
        let tree = IntervalTree::new(0..10);
        tree.find_with_interval(1..11);
    }
}
