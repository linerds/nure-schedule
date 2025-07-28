use crate::Group;

#[derive(serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GroupRaw {
    id: i64,
    name: String,
    direction_id: Option<i32>,
    speciality_id: Option<i32>,
}

impl From<GroupRaw> for (i64, Group) {
    fn from(
        GroupRaw {
            id,
            name,
            direction_id,
            speciality_id,
        }: GroupRaw,
    ) -> Self {
        (
            id,
            Group {
                name,
                direction_id,
                speciality_id,
            },
        )
    }
}
