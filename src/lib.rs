extern crate num_traits;

use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;

use num_traits::int::PrimInt;

mod interval;
pub use interval::Interval;
use interval::{IntervalBeginSorted, IntervalEndSorted};

#[derive(Debug)]
pub struct IntervalTree<T: Hash + Ord> {
    begin: T,
    end: T,
    center: T,

    left: Option<Box<IntervalTree<T>>>,
    right: Option<Box<IntervalTree<T>>>,

    overlaps_begin: BinaryHeap<IntervalBeginSorted<T>>,
    overlaps_end: BinaryHeap<IntervalEndSorted<T>>,
}

impl<T: Hash + PrimInt> IntervalTree<T> {
    pub fn new(begin: T, end: T) -> Self {
        Self {
            begin,
            end,
            center: (begin + end) / T::from(2).unwrap(),

            left: None,
            right: None,

            overlaps_begin: BinaryHeap::new(),
            overlaps_end: BinaryHeap::new(),
        }
    }

    /// # Examples
    ///
    /// ```rust
    /// extern crate interval_tree;
    /// use interval_tree::{Interval, IntervalTree};
    ///
    /// let mut tree = IntervalTree::new(0, 100);
    ///
    /// tree.insert(Interval::new(5, 10));
    /// tree.insert(Interval::new(85, 95));
    /// ```
    ///
    /// # Panic
    ///
    /// Panics if the interval overflows the range of this interval tree.
    pub fn insert(&mut self, interval: Interval<T>) {
        assert!(interval.begin >= self.begin && interval.end <= self.end);

        if interval.end <= self.center {
            if self.left.is_none() {
                self.left = Some(Box::new(IntervalTree::new(self.begin, self.center)));
            }

            self.left.as_mut().unwrap().insert(interval);
        } else if interval.begin > self.center {
            if self.right.is_none() {
                self.right = Some(Box::new(IntervalTree::new(self.center, self.end)));
            }

            self.right.as_mut().unwrap().insert(interval);
        } else {
            self.overlaps_begin
                .push(IntervalBeginSorted::new(interval.clone()));
            self.overlaps_end
                .push(IntervalEndSorted::new(interval.clone()));
        }
    }

    /// # Examples
    ///
    /// ```rust
    /// extern crate interval_tree;
    ///
    /// use std::collections::HashSet;
    /// use interval_tree::{Interval, IntervalTree};
    ///
    /// let mut tree = IntervalTree::new(0, 100);
    ///
    /// tree.insert(Interval::new(5, 10));
    /// tree.insert(Interval::new(85, 95));
    /// tree.insert(Interval::new(90, 100));
    ///
    /// assert_eq!(tree.find_with_point(0), HashSet::new());
    ///
    /// let intervals = [Interval::new(5, 10)].iter().cloned().collect();
    /// assert_eq!(tree.find_with_point(5), intervals);
    ///
    /// let intervals = [Interval::new(85, 95), Interval::new(90, 100)].iter().cloned().collect();
    /// assert_eq!(tree.find_with_point(90), intervals);
    /// ```
    ///
    /// # Panic
    ///
    /// Panics if the point is out-of-range of this interval tree;
    pub fn find_with_point(&self, point: T) -> HashSet<Interval<T>> {
        assert!(point >= self.begin && point < self.end);

        let mut found = HashSet::new();
        self.find_with_point_rec(point, &mut found);
        found
    }

    fn find_with_point_rec(&self, point: T, found: &mut HashSet<Interval<T>>) {
        if point < self.center {
            for intv in self.overlaps_begin
                .iter()
                .filter(|intv| intv.begin <= point)
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
    //     assert!(interval.begin >= self.begin && interval.end <= self.end);
    //
    //     let mut found = HashSet::new();
    //     for p in interval.begin..interval.end {
    //         found.union(&self.find_with_point(p));
    //     }
    //
    //     found
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_with_point() {
        let mut tree = IntervalTree::new(0, 10);
        for i in 0..=5 {
            tree.insert(Interval::new(i, i + 5));
        }

        assert_eq!(
            tree.find_with_point(0),
            [Interval::new(0, 5)].iter().cloned().collect()
        );

        assert_eq!(
            tree.find_with_point(2),
            [
                Interval::new(0, 5),
                Interval::new(1, 6),
                Interval::new(2, 7)
            ].iter()
                .cloned()
                .collect()
        );

        assert_eq!(
            tree.find_with_point(5),
            [
                Interval::new(1, 6),
                Interval::new(2, 7),
                Interval::new(3, 8),
                Interval::new(4, 9),
                Interval::new(5, 10)
            ].iter()
                .cloned()
                .collect()
        );

        assert_eq!(
            tree.find_with_point(7),
            [
                Interval::new(3, 8),
                Interval::new(4, 9),
                Interval::new(5, 10)
            ].iter()
                .cloned()
                .collect()
        );

        assert_eq!(
            tree.find_with_point(9),
            [Interval::new(5, 10)].iter().cloned().collect()
        );
    }

    #[test]
    #[should_panic]
    fn panic_insert_begin() {
        let mut tree = IntervalTree::new(1, 11);
        tree.insert(Interval::new(0, 10));
    }

    #[test]
    #[should_panic]
    fn panic_insert_end() {
        let mut tree = IntervalTree::new(0, 10);
        tree.insert(Interval::new(1, 11));
    }

    #[test]
    #[should_panic]
    fn panic_find_with_point_begin() {
        let tree = IntervalTree::new(1, 11);
        tree.find_with_point(0);
    }

    #[test]
    #[should_panic]
    fn panic_find_with_point_end() {
        let tree = IntervalTree::new(0, 10);
        tree.find_with_point(10);
    }
}
