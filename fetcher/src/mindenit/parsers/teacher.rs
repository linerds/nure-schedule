use crate::Teacher;

#[derive(serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeacherRaw {
    id: i64,
    full_name: String,
    short_name: String,
    department_id: Option<i32>,
}

impl From<TeacherRaw> for (i64, Teacher) {
    fn from(
        TeacherRaw {
            id,
            full_name,
            short_name,
            department_id,
        }: TeacherRaw,
    ) -> Self {
        (
            id,
            Teacher {
                abbr: short_name,
                name: full_name,
                department_id,
            },
        )
    }
}
