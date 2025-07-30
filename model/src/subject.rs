use crate::impl_borrow;

use super::Id;

#[derive(Clone, Ord, PartialOrd, Debug)]
pub struct Subject {
    pub id: Id,
    pub abbr: String,
    pub name: String,
}

impl_borrow!(Subject);
