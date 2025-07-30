use crate::impl_borrow;

use super::Id;

#[derive(Clone, Ord, PartialOrd, Debug)]
pub struct Group {
    pub id: Id,
    pub name: String,
    pub direction_id: Option<i32>,
    pub speciality_id: Option<i32>,
}

impl_borrow!(Group);
