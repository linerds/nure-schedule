use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum EventKind {
    #[serde(rename = "Лк")]
    Lecture,
    #[serde(rename = "Пз")]
    PracticalWork,
    #[serde(rename = "Лб")]
    LaboratoryWork,
    #[serde(rename = "Конс")]
    Consultation,
    #[serde(rename = "Зал")]
    FinalTest,
    #[serde(rename = "Екз")]
    Exam,
    #[serde(rename = "КП/КР")]
    CourseWork,
}
