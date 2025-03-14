use std::collections::BinaryHeap;

use crate::scored_item::ScoredItem;

/// A priority queue with a score function.
pub struct PriorityQueue<T, S: Ord + Copy> {
    heap: BinaryHeap<ScoredItem<T, S>>,
    score_fn: Box<dyn Fn(&T) -> S>,
}

impl<T, S: Ord + Copy> PriorityQueue<T, S> {
    /// Creates a new priority queue with the given score function.
    pub fn new(score_fn: Box<dyn Fn(&T) -> S>) -> Self {
        let heap = BinaryHeap::new();
        Self { heap, score_fn }
    }

    /// Returns the reference to the first elements in the queue.
    pub fn peek(&self) -> Option<(S, &T)> {
        self.heap
            .peek()
            .map(|scored_item| (scored_item.score, &scored_item.item))
    }

    /// Returns the first elements in the queue.
    pub fn pop(&mut self) -> Option<(S, T)> {
        self.heap
            .pop()
            .map(|scored_item| (scored_item.score, scored_item.item))
    }

    /// Pushes an item into the queue.
    /// The score of the item is calculated by the score function.
    pub fn push(&mut self, item: T) {
        let score = (self.score_fn)(&item);
        self.push_with_score(item, score);
    }

    /// Pushes an item into the queue with the given score
    /// without using the score function.
    pub fn push_with_score(&mut self, item: T, score: S) {
        self.heap.push(ScoredItem { item, score });
    }

    /// Converts the priority queue into a vector of (score, item) pairs.
    pub fn into_vec(self) -> Vec<(S, T)> {
        self.heap.into_iter().map(|x| (x.score, x.item)).collect()
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Reverse;

    use super::PriorityQueue;
    #[test]
    fn test_priority_queue() {
        let score_fn = Box::new(|s: &String| s.len());
        let mut queue = PriorityQueue::new(score_fn);

        assert!(queue.peek().is_none());

        queue.push("a".to_string()); // score = 1
        queue.push("ccc".to_string()); // score = 3
        queue.push("bb".to_string()); // score = 2
        queue.push_with_score("b".to_string(), 10); // score = 10

        assert_eq!(queue.peek(), Some((10, &"b".to_string())));
        assert_eq!(queue.pop(), Some((10, "b".to_string())));
        assert_eq!(queue.pop(), Some((3, "ccc".to_string())));
        assert_eq!(queue.pop(), Some((2, "bb".to_string())));
        assert_eq!(queue.pop(), Some((1, "a".to_string())));
        assert!(queue.peek().is_none());
    }

    #[test]
    fn test_priority_queue_reverse_order() {
        let score_fn = Box::new(|s: &String| Reverse(s.len()));
        let mut queue = PriorityQueue::new(score_fn);

        queue.push("a".to_string()); // score = -1
        queue.push("ccc".to_string()); // score = -3
        queue.push("bb".to_string()); // score = -2
        queue.push_with_score("b".to_string(), Reverse(10)); // score = -10

        assert_eq!(queue.peek(), Some((Reverse(1), &"a".to_string())));
        assert_eq!(queue.pop(), Some((Reverse(1), "a".to_string())));
        assert_eq!(queue.pop(), Some((Reverse(2), "bb".to_string())));
        assert_eq!(queue.pop(), Some((Reverse(3), "ccc".to_string())));
        assert_eq!(queue.pop(), Some((Reverse(10), "b".to_string())));
        assert!(queue.peek().is_none());
    }

    #[test]
    fn test_into_vec() {
        let score_fn = Box::new(|s: &String| s.len());
        let mut queue = PriorityQueue::new(score_fn);

        queue.push("a".to_string()); // score = 1
        queue.push("ccc".to_string()); // score = 3
        queue.push("bb".to_string()); // score = 2
        queue.push_with_score("b".to_string(), 10); // score = 10

        let mut vec = queue.into_vec();
        vec.sort_by_key(|x| x.0);
        assert_eq!(
            vec,
            vec![
                (1, "a".to_string()),
                (2, "bb".to_string()),
                (3, "ccc".to_string()),
                (10, "b".to_string()),
            ]
        );
    }
}
