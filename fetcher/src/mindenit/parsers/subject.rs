use crate::Subject;

#[derive(serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SubjectRaw {
    id: i64,
    name: String,
    brief: String,
}

impl From<SubjectRaw> for Subject {
    fn from(SubjectRaw { id, name, brief }: SubjectRaw) -> Self {
        Self {
            id,
            name,
            abbr: brief,
        }
    }
}
