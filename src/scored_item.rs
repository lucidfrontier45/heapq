use std::cmp::Ordering;

pub(crate) struct ScoredItem<T, S: Ord> {
    pub(crate) score: S,
    pub(crate) item: T,
}

impl<T, S: Ord> PartialEq for ScoredItem<T, S> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<T, S: Ord> Eq for ScoredItem<T, S> {}

impl<T, S: Ord> Ord for ScoredItem<T, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl<T, S: Ord> PartialOrd for ScoredItem<T, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, S: Ord> From<(S, T)> for ScoredItem<T, S> {
    fn from(value: (S, T)) -> Self {
        Self {
            score: value.0,
            item: value.1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::ScoredItem;

    #[test]
    fn test_scored_item() {
        let x1 = ScoredItem {
            score: 1,
            item: "z",
        };
        let x2 = ScoredItem {
            score: 2,
            item: "a",
        };

        assert!(x1 < x2);
    }
}
