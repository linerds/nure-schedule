use crate::{EventKind, join};

use std::{collections::BTreeSet, fmt::Write};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FilterBuilder(Filter);

impl Default for FilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl From<FilterBuilder> for Option<Filter> {
    fn from(value: FilterBuilder) -> Self {
        value.build()
    }
}

/// [`FilterBuilder::build()`] will return [None] when it is empty:
/// ```rust
/// use schedule_data::FilterBuilder;
///
/// let builder = FilterBuilder::new();
/// assert_eq!(builder.build(), None);
/// ```
///
/// It will build successfully when not empty:
/// ```rust
/// use schedule_data::{FilterBuilder, EventKind};
///
/// let mut builder = FilterBuilder::new().groups([1]);
///
/// let filter = builder.build().expect("filter is not empty");
/// ```
impl FilterBuilder {
    pub const fn new() -> Self {
        Self(Filter {
            kinds: BTreeSet::new(),
            subjects: BTreeSet::new(),
            auditoriums: BTreeSet::new(),
            groups: BTreeSet::new(),
            teachers: BTreeSet::new(),
        })
    }
    pub fn build(self) -> Option<Filter> {
        if self.0.kinds.is_empty()
            && self.0.groups.is_empty()
            && self.0.subjects.is_empty()
            && self.0.auditoriums.is_empty()
            && self.0.groups.is_empty()
            && self.0.teachers.is_empty()
        {
            None
        } else {
            Some(self.0)
        }
    }

    #[must_use]
    pub fn kinds(mut self, iter: impl IntoIterator<Item = EventKind>) -> Self {
        self.0.kinds = BTreeSet::from_iter(iter);
        self
    }
    #[must_use]
    pub fn subjects(mut self, iter: impl IntoIterator<Item = i64>) -> Self {
        self.0.subjects = BTreeSet::from_iter(iter);
        self
    }
    #[must_use]
    pub fn auditoriums(mut self, iter: impl IntoIterator<Item = i64>) -> Self {
        self.0.auditoriums = BTreeSet::from_iter(iter);
        self
    }
    #[must_use]
    pub fn groups(mut self, iter: impl IntoIterator<Item = i64>) -> Self {
        self.0.groups = BTreeSet::from_iter(iter);
        self
    }
    #[must_use]
    pub fn teachers(mut self, iter: impl IntoIterator<Item = i64>) -> Self {
        self.0.teachers = BTreeSet::from_iter(iter);
        self
    }
}

/// Has no public methods nor fields. See [`FilterBuilder`].
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Filter {
    kinds: BTreeSet<EventKind>,
    subjects: BTreeSet<i64>,
    auditoriums: BTreeSet<i64>,
    groups: BTreeSet<i64>,
    teachers: BTreeSet<i64>,
}

impl Filter {
    /// Should construct something similar to this (no formatting though):
    /// ```sql
    /// SELECT e.id FROM Events e
    ///     JOIN EventGroups eg ON eg.event_id = e.id
    ///     JOIN EventTeachers et ON et.event_id = e.id
    /// WHERE
    ///         e.auditorium_id = 1
    ///     AND e.subject_id IN (1, 2)
    ///     AND e.kind IN (1, 2, 3, 4)
    ///     AND eg.group_id IN (1, 2)
    ///     AND et.teacher_id IN (1, 2)
    /// GROUP BY e.id HAVING
    ///         COUNT(DISTINCT eg.group_id) = 2
    ///     AND COUNT(DISTINCT et.teacher_id) = 2
    /// ```
    pub(crate) fn build_query(
        Self {
            groups,
            teachers,
            subjects,
            auditoriums,
            kinds,
        }: &Self,
    ) -> String {
        let mut query = String::from(" SELECT e.id FROM Events e ");

        let mut counts = Vec::new();

        if !groups.is_empty() {
            query.push_str(" JOIN EventGroups eg ON eg.event_id = e.id ");
            counts.push(format!(" COUNT(DISTINCT eg.group_id) = {} ", groups.len()));
        }
        if !teachers.is_empty() {
            query.push_str(" JOIN EventTeachers et ON et.event_id = e.id ");
            counts.push(format!(
                " COUNT(DISTINCT et.teacher_id) = {} ",
                teachers.len()
            ));
        }

        query.push_str(" WHERE ");
        query.push_str(&join(
            [
                where_part("e.kind", kinds),
                where_part("e.subject_id", subjects),
                where_part("e.auditorium_id", auditoriums),
                where_part("eg.group_id", groups),
                where_part("et.teacher_id", teachers),
            ]
            .into_iter()
            .flatten(),
            " AND ",
        ));

        query.push_str(" GROUP BY e.id HAVING ");
        query.push_str(&join(&counts, " AND "));

        query
    }
}

fn where_part<I, T>(field: &str, iter: I) -> Option<String>
where
    I: IntoIterator<Item = T>,
    T: std::fmt::Display,
{
    let mut iter = iter.into_iter();

    let first = iter.next()?;

    let Some(second) = iter.next() else {
        return Some(format!(" {field} = {first} "));
    };

    let mut res = format!(" {field} IN ({first}, {second}");
    for val in iter {
        write!(res, ", {val}").expect("write! to string should not fail");
    }
    res.push_str(") ");

    Some(res)
}
