use super::{impl_borrow, Id};

#[derive(Clone, Ord, PartialOrd, Debug)]
pub struct Auditorium {
    pub id: Id,
    pub name: String,
    pub floor: i8,
    pub power: bool,
    pub building: String,
}

impl_borrow!(Auditorium);
