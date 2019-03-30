# Interval Tree

A simple data structure for efficiently inserting and finding intervals.

Currently this crate supports

- inserting intervals
- findind intervals with a point
- findind intervals with an interval

## Examples

```rust
use interval_tree::{Interval, IntervalTree};

let mut tree = IntervalTree::new(0..10);
for i in 0..=5 {
    tree.insert(i..(i + 5));
}

// 0  1  2  3  4  5  6  7  8  9  10
// |--|--|--|--|--|--|--|--|--|--|
// *--------------o
//    *--------------o
//       *--------------o
//          *--------------o
//             *--------------o
//                *--------------o

assert_eq!(
    tree.find_with_point(1),
    [&(0..5), &(1..6)].iter().cloned().collect()
);

assert_eq!(
    tree.find_with_point(5),
    [&(1..6), &(2..7), &(3..8), &(4..9), &(5..10)].iter().cloned().collect()
);

assert_eq!(
    tree.find_with_point(9),
    [&(5..10)].iter().cloned().collect()
);

assert_eq!(
    tree.find_with_interval(0..3),
    [&(0..5), &(1..6), &(2..7)].iter().cloned().collect()
);

assert_eq!(
    tree.find_with_interval(6..9),
    [&(2..7), &(3..8), &(4..9), &(5..10)].iter().cloned().collect()
);
```
