use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;
use std::ops::{Add, Div, Range};

use interval::{IntervalBeginSorted, IntervalEndSorted};
use Interval;

/// Interval tree.
#[derive(Debug)]
pub struct IntervalTree<T: Ord> {
    range: Range<T>,
    center: T,

    left: Option<Box<IntervalTree<T>>>,
    right: Option<Box<IntervalTree<T>>>,

    overlaps_begin: BinaryHeap<IntervalBeginSorted<T>>,
    overlaps_end: BinaryHeap<IntervalEndSorted<T>>,
}

impl<T> IntervalTree<T>
where
    T: Clone + Ord + Hash + Add<T, Output = T> + Div<i32, Output = T>,
{
    /// Creates a interval tree on `range`.
    pub fn new(range: Range<T>) -> Self {
        Self {
            range: range.clone(),
            center: (range.start + range.end) / 2,

            left: None,
            right: None,

            overlaps_begin: BinaryHeap::new(),
            overlaps_end: BinaryHeap::new(),
        }
    }

    /// Inserts an [`Interval`](struct.Interval.html) to this interval tree.
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate interval_tree;
    /// use interval_tree::{Interval, IntervalTree};
    ///
    /// let mut tree = IntervalTree::new(0..100);
    ///
    /// tree.insert(Interval::new(5..10));
    /// tree.insert(Interval::new(85..95));
    /// ```
    ///
    /// # Panic
    ///
    /// Panics if the interval overflows the range of this interval tree.
    pub fn insert(&mut self, interval: Interval<T>) {
        assert!(!self.overflow_interval(&interval));

        if interval.end <= self.center {
            if self.left.is_none() {
                let range = self.range.start.clone()..self.center.clone();
                self.left = Some(Box::new(IntervalTree::new(range)));
            }

            self.left.as_mut().unwrap().insert(interval);
        } else if interval.start > self.center {
            if self.right.is_none() {
                let range = self.center.clone()..self.range.end.clone();
                self.right = Some(Box::new(IntervalTree::new(range)));
            }

            self.right.as_mut().unwrap().insert(interval);
        } else {
            self.overlaps_begin
                .push(IntervalBeginSorted::new(interval.clone()));
            self.overlaps_end
                .push(IntervalEndSorted::new(interval.clone()));
        }
    }

    /// Finds [`Interval`](struct.Interval.html)s in this interval tree that contains the `point`.
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
    /// tree.insert(Interval::new(5..10));
    /// tree.insert(Interval::new(85..95));
    /// tree.insert(Interval::new(90..100));
    ///
    /// assert_eq!(tree.find_with_point(0), HashSet::new());
    ///
    /// let intervals = [Interval::new(5..10)].iter().cloned().collect();
    /// assert_eq!(tree.find_with_point(5), intervals);
    ///
    /// let intervals = [Interval::new(85..95), Interval::new(90..100)].iter().cloned().collect();
    /// assert_eq!(tree.find_with_point(90), intervals);
    /// ```
    ///
    /// # Panic
    ///
    /// Panics if the point is out-of-range of this interval tree;
    pub fn find_with_point(&self, point: T) -> HashSet<Interval<T>> {
        assert!(!self.overflow_point(&point));

        let mut found = HashSet::new();
        self.find_with_point_rec(point, &mut found);
        found
    }

    fn find_with_point_rec(&self, point: T, found: &mut HashSet<Interval<T>>) {
        if point < self.center {
            for intv in self.overlaps_begin
                .iter()
                .filter(|intv| intv.start <= point)
            {
                found.insert(intv.0.clone());
            }

            if let Some(ref left) = self.left {
                left.find_with_point_rec(point, found);
            }
        } else {
            for intv in self.overlaps_end.iter().filter(|intv| intv.end > point) {
                found.insert(intv.0.clone());
            }

            if let Some(ref right) = self.right {
                right.find_with_point_rec(point, found);
            }
        }
    }

    // fn find_with_interval(&self, interval: Interval<T>) -> HashSet<Interval<T>> {
    //     assert!(!self.overflow_interval(&interval));
    //
    //     let mut found = HashSet::new();
    //     for p in interval.0 {
    //         found.union(&self.find_with_point(p));
    //     }
    //
    //     found
    // }

    fn overflow_interval(&self, interval: &Interval<T>) -> bool {
        interval.start < self.range.start || interval.end > self.range.end
    }

    fn overflow_point(&self, point: &T) -> bool {
        point < &self.range.start || point >= &self.range.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_with_point() {
        let mut tree = IntervalTree::new(0..10);
        for i in 0..=5 {
            tree.insert(Interval::new(i..(i + 5)));
        }

        assert_eq!(
            tree.find_with_point(0),
            [Interval::new(0..5)].iter().cloned().collect()
        );

        assert_eq!(
            tree.find_with_point(2),
            [
                Interval::new(0..5),
                Interval::new(1..6),
                Interval::new(2..7)
            ].iter()
                .cloned()
                .collect()
        );

        assert_eq!(
            tree.find_with_point(5),
            [
                Interval::new(1..6),
                Interval::new(2..7),
                Interval::new(3..8),
                Interval::new(4..9),
                Interval::new(5..10)
            ].iter()
                .cloned()
                .collect()
        );

        assert_eq!(
            tree.find_with_point(7),
            [
                Interval::new(3..8),
                Interval::new(4..9),
                Interval::new(5..10)
            ].iter()
                .cloned()
                .collect()
        );

        assert_eq!(
            tree.find_with_point(9),
            [Interval::new(5..10)].iter().cloned().collect()
        );
    }

    #[test]
    #[should_panic]
    fn panic_insert_begin() {
        let mut tree = IntervalTree::new(1..11);
        tree.insert(Interval::new(0..10));
    }

    #[test]
    #[should_panic]
    fn panic_insert_end() {
        let mut tree = IntervalTree::new(0..10);
        tree.insert(Interval::new(1..11));
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
}
