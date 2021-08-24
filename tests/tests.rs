use nosracid::id::Id;
use uuid::{Uuid};

// External package imports
use chrono::{NaiveDate};

#[test]
fn test_blank_id_construction() {

    // test manual construction of rule
    let _test_id = Id {
        uuid: Uuid::new_v4(),
        first_name: String::from("First"),
        middle_name: String::from("Middle"),
        last_name: String::from("Last"),
        emails: vec![String::from("example@example.com")],
        date_of_birth: NaiveDate::from_ymd(1970, 01, 01),
    };

}

#[test]
fn test_id_from_data() {
    // test constructor from data
    let _test_id = Id::new_from_data(
        Uuid::new_v4(),
        String::from("First"),
        String::from("Middle"),
        String::from("Last"),
        NaiveDate::from_ymd(1970, 01, 01),
        vec![String::from("example@example.com")],
    );
}
