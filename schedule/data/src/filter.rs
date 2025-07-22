use crate::EventKind;

use std::{collections::BTreeSet, fmt::Write};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Filter {
    kinds: BTreeSet<EventKind>,
    subjects: BTreeSet<i64>,
    auditoriums: BTreeSet<i64>,
    groups: BTreeSet<i64>,
    teachers: BTreeSet<i64>,
}

impl Filter {
    pub fn builder() -> FilterBuilder {
        FilterBuilder::default()
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FilterBuilder(Filter);

impl Default for FilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl From<FilterBuilder> for Filter {
    fn from(value: FilterBuilder) -> Self {
        value.build()
    }
}

/// ```rust
/// use schedule_data::{Filter, FilterBuilder};
///
/// FilterBuilder::new().groups([4, 2]).build();
/// // OR
/// let filter: Filter = FilterBuilder::new().teachers([1]).into();
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
    pub fn build(self) -> Filter {
        self.0
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

impl Filter {
    /// Should write something similar to this:
    /// ```sql
    /// SELECT e.id FROM Events e
    ///   JOIN EventGroups eg ON eg.event_id = e.id
    ///   JOIN EventTeachers et ON et.event_id = e.id
    /// WHERE e.auditorium_id = 1
    ///   AND e.subject_id IN (1, 2)
    ///   AND e.kind IN (1, 2, 3, 4)
    ///   AND eg.group_id IN (1, 2)
    ///   AND et.teacher_id IN (1, 2)
    /// GROUP BY e.id HAVING
    ///       COUNT(DISTINCT eg.group_id) = 2
    ///   AND COUNT(DISTINCT et.teacher_id) = 2
    /// ```
    pub(crate) fn write_query(&self, query: &mut String, written: &mut bool, write_before: &str) {
        if self.kinds.is_empty()
            && self.subjects.is_empty()
            && self.auditoriums.is_empty()
            && self.groups.is_empty()
            && self.teachers.is_empty()
        {
            return;
        }

        if *written {
            query.push_str(write_before);
        }
        *written = true;

        query.push_str("SELECT e.id FROM Events e\n");

        if !self.groups.is_empty() {
            query.push_str("  JOIN EventGroups eg ON eg.event_id = e.id\n");
        }
        if !self.teachers.is_empty() {
            query.push_str("  JOIN EventTeachers et ON et.event_id = e.id\n");
        }

        query.push_str("WHERE ");

        let mut prepend_and = false;
        where_condition(query, &mut prepend_and, "e.kind", &self.kinds);
        where_condition(query, &mut prepend_and, "e.subject_id", &self.subjects);
        where_condition(
            query,
            &mut prepend_and,
            "e.auditorium_id",
            &self.auditoriums,
        );
        where_condition(query, &mut prepend_and, "eg.group_id", &self.groups);
        where_condition(query, &mut prepend_and, "et.teacher_id", &self.teachers);

        if self.groups.is_empty() && self.teachers.is_empty() {
            return;
        }
        query.push_str("GROUP BY e.id HAVING\n  ");

        if !self.groups.is_empty() {
            #[cfg(test)] // pretty
            if !self.teachers.is_empty() {
                query.push_str("    ");
            }

            write!(query, "COUNT(DISTINCT eg.group_id) = {}", self.groups.len()).unwrap();

            if !self.teachers.is_empty() {
                query.push_str("\n  AND ");
            }
        }
        if !self.teachers.is_empty() {
            write!(
                query,
                "COUNT(DISTINCT et.teacher_id) = {}",
                self.teachers.len()
            )
            .unwrap();
        }
    }
}

fn where_condition<I, T>(query: &mut String, prepend_and: &mut bool, field: &str, iter: I)
where
    I: IntoIterator<Item = T>,
    T: std::fmt::Display,
{
    let mut iter = iter.into_iter();

    let Some(first) = iter.next() else {
        return;
    };

    if *prepend_and {
        query.push_str("  AND ");
    }
    *prepend_and = true; // for the future calls

    let Some(second) = iter.next() else {
        writeln!(query, "{field} = {first}").unwrap();
        return;
    };

    write!(query, "{field} IN ({first}, {second}").unwrap();
    for val in iter {
        write!(query, ", {val}").unwrap();
    }
    query.push_str(")\n");
}
