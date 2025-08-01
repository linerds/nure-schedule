use proc::PartialBorrow;

#[derive(Clone, Ord, PartialOrd, Debug, PartialBorrow)]
pub struct Subject {
    #[borrow_id]
    pub id: i64,
    pub abbr: String,
    pub name: String,
}
