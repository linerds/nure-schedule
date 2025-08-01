use proc::PartialBorrow;

#[derive(Clone, Ord, PartialOrd, Debug, PartialBorrow)]
pub struct Auditorium {
    #[borrow_id]
    pub id: i64,
    pub name: String,
    pub floor: i8,
    pub power: bool,
    pub building: String,
}
