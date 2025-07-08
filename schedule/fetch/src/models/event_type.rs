use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum EventType {
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
