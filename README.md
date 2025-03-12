# Heapq
Priority Queue with scoring function

# Usage

- Rust Edition: 2024
- MSRV: 1.85

```toml
[dependencies]
heapq = "0.2.0"
```

In the code, you first need to create an instance with `heapq::PriorityQueue::new`. It takes a closure that converts your item type `&T` into score type `S: Ord`. Then you can use `push/pop/peek` methods in the same way as `std::collections::BinaryHeap`.

# Example

```rust
use std::cmp::Reverse;

use heapq::PriorityQueue;

fn main() {
    // a score function that returns the length of a string
    let score_fn = Box::new(|s: &String| s.len());
    // create a new priority queue with the score function
    let mut queue = PriorityQueue::new(score_fn);

    // the queue is empty at the beginning
    assert!(queue.peek().is_none());

    // push some items into the queue
    // the score function is used to calculate the score of each item
    queue.push("a".to_string()); // score = 1
    queue.push("ccc".to_string()); // score = 3
    queue.push("bb".to_string()); // score = 2

    // you can also push an item with an explicit score
    queue.push_with_score("b".to_string(), 10); // score = 10

    // peek the item with the highest priority
    assert_eq!(queue.peek(), Some(&"b".to_string()));

    // pop the item with the highest priority
    assert_eq!(queue.pop(), Some("b".to_string()));
    assert_eq!(queue.pop(), Some("ccc".to_string()));
    assert_eq!(queue.pop(), Some("bb".to_string()));
    assert_eq!(queue.pop(), Some("a".to_string()));

    // the queue is empty again
    assert!(queue.peek().is_none());

    // you can also use a reverse order
    let score_fn = Box::new(|s: &String| Reverse(s.len()));
    let mut queue = PriorityQueue::new(score_fn);

    queue.push("a".to_string()); // score = -1
    queue.push("ccc".to_string()); // score = -3
    queue.push("bb".to_string()); // score = -2

    // remember to use Reverse when pushing an item with an explicit score
    queue.push_with_score("b".to_string(), Reverse(10)); // score = -10

    assert_eq!(queue.peek(), Some(&"a".to_string()));
    assert_eq!(queue.pop(), Some("a".to_string()));
    assert_eq!(queue.pop(), Some("bb".to_string()));
    assert_eq!(queue.pop(), Some("ccc".to_string()));
    assert_eq!(queue.pop(), Some("b".to_string()));
    assert!(queue.peek().is_none());
}
```