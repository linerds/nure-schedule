use proc::PartialBorrow;

#[derive(Clone, Ord, PartialOrd, Debug, PartialBorrow)]
pub struct Teacher {
    #[borrow_id]
    pub id: i64,
    pub abbr: String,
    pub name: String,
    pub department_id: Option<i32>,
}
