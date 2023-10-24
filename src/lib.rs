#![forbid(missing_docs)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

mod queue;
mod scored_item;

pub use queue::PriorityQueue;
