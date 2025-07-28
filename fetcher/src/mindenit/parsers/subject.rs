use crate::Subject;

#[derive(serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SubjectRaw {
    id: i64,
    name: String,
    brief: String,
}

impl From<SubjectRaw> for (i64, Subject) {
    fn from(SubjectRaw { id, name, brief }: SubjectRaw) -> Self {
        (id, Subject { name, abbr: brief })
    }
}
