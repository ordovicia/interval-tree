# Interval Tree

A simple data structure representing an interval tree.

Currently this crate supports

- inserting intervals, and
- findind intervals with a point.

## Examples

```rust
extern crate interval_tree;
use interval_tree::{Interval, IntervalTree};

let mut tree = IntervalTree::new(0..10);
for i in 0..=5 {
    tree.insert(Interval::new(i..(i + 5)));
}

assert_eq!(
    tree.find_with_point(1),
    [Interval::new(0..5), Interval::new(1..6)].iter().cloned().collect()
);

assert_eq!(
    tree.find_with_point(5),
    (1..=5).map(|i| Interval::new(i..(i + 5))).collect());

assert_eq!(
    tree.find_with_point(9),
    [Interval::new(5..10)].iter().cloned().collect()
);
```
