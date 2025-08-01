use std::{collections::HashSet, hash::Hash, marker::PhantomData};

use serde::{
    de::{Deserializer, SeqAccess, Visitor},
    Deserialize,
};

/// [`Deserialize`] an array of `Raw` values ( `[ Raw, .. ]` ) into a [`HashSet<T>`]. \
/// `Raw` must implement [`Deserialize`] and `Into<T>`.
pub struct ArrayToSet<Raw, T> {
    map: HashSet<T>,
    _marker: PhantomData<fn(Raw) -> T>,
}
impl<R, T> From<ArrayToSet<R, T>> for HashSet<T> {
    fn from(value: ArrayToSet<R, T>) -> Self {
        value.map
    }
}

struct ArrayVisitor<R, T> {
    _marker: PhantomData<fn() -> ArrayToSet<R, T>>,
}

impl<'de, R, T> Visitor<'de> for ArrayVisitor<R, T>
where
    R: Deserialize<'de> + Into<T>,
    T: Hash + Eq,
{
    type Value = ArrayToSet<R, T>;

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut map = HashSet::new();
        while let Some(group) = seq.next_element::<R>()?.map(Into::into) {
            map.insert(group); // TODO? log duplicate ids
        }
        Ok(ArrayToSet {
            map,
            _marker: PhantomData,
        })
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an array of values")
    }
}

impl<'de, R, T> Deserialize<'de> for ArrayToSet<R, T>
where
    R: Deserialize<'de> + Into<T>,
    T: Hash + Eq,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ArrayVisitor {
            _marker: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Hash, Eq, PartialEq)]
    pub struct Person {
        pub id: i64,
        pub name: String,
    }

    #[derive(Deserialize)]
    pub struct PersonRaw {
        id: i64,
        name: String,
    }
    impl From<PersonRaw> for Person {
        fn from(PersonRaw { id, name }: PersonRaw) -> Self {
            Self { id, name }
        }
    }

    #[test]
    fn parse() -> Result<(), serde_json::Error> {
        let data = r#"[ { "id": 1, "name": "John" },
                        { "id": 3, "name": "Jane" },
                        { "id": 2, "name": "Alice" } ]"#;
        let parsed: ArrayToSet<PersonRaw, Person> = serde_json::from_str(data)?;
        let map: HashSet<Person> = parsed.into();

        println!("{map:#?}");

        Ok(())
    }
}
