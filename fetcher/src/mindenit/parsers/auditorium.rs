use crate::Auditorium;

#[derive(serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuditoriumRaw {
    id: i64,
    name: String,
    floor: i8,
    has_power: bool,
    building_id: String,
}

impl From<AuditoriumRaw> for (i64, Auditorium) {
    fn from(
        AuditoriumRaw {
            id,
            name,
            floor,
            has_power,
            building_id,
        }: AuditoriumRaw,
    ) -> Self {
        (
            id,
            Auditorium {
                name,
                floor,
                power: has_power,
                building: building_id,
            },
        )
    }
}
