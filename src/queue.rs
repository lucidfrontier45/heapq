use std::collections::BinaryHeap;

use crate::scored_item::ScoredItem;

pub struct PriorityQueue<T, S: Ord, F: Fn(&T) -> S> {
    heap: BinaryHeap<ScoredItem<T, S>>,
    score_fn: F,
}

impl<T, S: Ord, F: Fn(&T) -> S> PriorityQueue<T, S, F> {
    pub fn new(score_fn: F) -> Self {
        let heap = BinaryHeap::new();
        Self { heap, score_fn }
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|item| &item.item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|item| item.item)
    }

    pub fn push_with_score(&mut self, item: T, score: S) {
        self.heap.push(ScoredItem { item, score });
    }

    pub fn push(&mut self, item: T) {
        let score = (self.score_fn)(&item);
        self.push_with_score(item, score);
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Reverse;

    use super::PriorityQueue;
    #[test]
    fn test_priority_queue() {
        let score_fn = |s: &String| s.len();
        let mut queue = PriorityQueue::new(score_fn);

        assert!(queue.peek().is_none());

        queue.push("a".to_string()); // score = 1
        queue.push("ccc".to_string()); // score = 3
        queue.push("bb".to_string()); // score = 2
        queue.push_with_score("b".to_string(), 10); // score = 10

        assert_eq!(queue.peek(), Some(&"b".to_string()));
        assert_eq!(queue.pop(), Some("b".to_string()));
        assert_eq!(queue.pop(), Some("ccc".to_string()));
        assert_eq!(queue.pop(), Some("bb".to_string()));
        assert_eq!(queue.pop(), Some("a".to_string()));
        assert!(queue.peek().is_none());
    }

    #[test]
    fn test_priority_queue_reverse_order() {
        let score_fn = |s: &String| Reverse(s.len());
        let mut queue = PriorityQueue::new(score_fn);

        queue.push("a".to_string()); // score = -1
        queue.push("ccc".to_string()); // score = -3
        queue.push("bb".to_string()); // score = -2
        queue.push_with_score("b".to_string(), Reverse(10)); // score = -10

        assert_eq!(queue.peek(), Some(&"a".to_string()));
        assert_eq!(queue.pop(), Some("a".to_string()));
        assert_eq!(queue.pop(), Some("bb".to_string()));
        assert_eq!(queue.pop(), Some("ccc".to_string()));
        assert_eq!(queue.pop(), Some("b".to_string()));
        assert!(queue.peek().is_none());
    }
}
