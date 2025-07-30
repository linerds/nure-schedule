use crate::impl_borrow;

use super::Id;

#[derive(Clone, Ord, PartialOrd, Debug)]
pub struct Teacher {
    pub id: Id,
    pub abbr: String,
    pub name: String,
    pub department_id: Option<i32>,
}

impl_borrow!(Teacher);
