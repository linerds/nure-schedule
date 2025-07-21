mod database;
mod entity;
mod error;
mod filter;

pub use database::Database;
pub use entity::*;
pub use filter::{Filter, FilterBuilder};

// TODO: move to utils crate
fn join<I, T>(iter: I, sep: &str) -> String
where
    I: IntoIterator<Item = T>,
    T: std::fmt::Display,
{
    use std::fmt::Write;

    let mut iter = iter.into_iter();
    let first = iter.next().map(|x| x.to_string()).unwrap_or_default();

    iter.fold(first, |mut acc, s| {
        write!(acc, "{sep}{s}").expect("failed to join strings with write! macro");
        acc
    })
}
