use proc::PartialBorrow;

#[derive(Clone, Ord, PartialOrd, Debug, PartialBorrow)]
pub struct Group {
    #[borrow_id]
    pub id: i64,
    pub name: String,
    pub direction_id: Option<i32>,
    pub speciality_id: Option<i32>,
}
