use proc::PartialBorrow;
use std::collections::HashSet;

#[test]
fn partial_borrow() {
    #[derive(Debug, PartialBorrow)]
    pub struct Person {
        #[borrow_id]
        pub id: i64,
        pub _name: String,
    }

    let pencel = Person {
        id: 1,
        _name: "pencelheimer".into(),
    };

    let noatu = Person {
        id: 2,
        _name: "noatu".into(),
    };

    let people = HashSet::from([pencel, noatu]);

    assert!(people.contains(&1));
    assert!(!people.contains(&3));
}
