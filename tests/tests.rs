use nosracid::id::Id;
use nosracid::extension::{CertifiedExtension, US_SSN};

// External package imports
use chrono::{NaiveDate};
use uuid::{Uuid};

#[test]
fn test_blank_id_construction() {

    let test_ssn = US_SSN::new(String::from("000000000")).unwrap();

    // test manual construction of rule
    let _test_id = Id {
        uuid: Uuid::new_v4(),
        first_name: String::from("First"),
        middle_name: String::from("Middle"),
        last_name: String::from("Last"),
        date_of_birth: NaiveDate::from_ymd(1970, 01, 01),
        emails: vec![String::from("example@example.com")],
        certified_extensions: vec![CertifiedExtension::US_SSN(test_ssn)],
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
        Vec::new(),
    );
}


#[test]
fn test_id_to_json() {
    // Test conversion of ID to json object
    let test_ssn = US_SSN::new(String::from("000000000")).unwrap();

    // test manual construction of rule
    let _test_id = Id {
        uuid: Uuid::new_v4(),
        first_name: String::from("First"),
        middle_name: String::from("Middle"),
        last_name: String::from("Last"),
        date_of_birth: NaiveDate::from_ymd(1970, 01, 01),
        emails: vec![String::from("example@example.com")],
        certified_extensions: vec![CertifiedExtension::US_SSN(test_ssn)],
    };

    // Convert the ID to a JSON string.
    let serialized = _test_id.to_json();

    // Must be run with `cargo test -- --nocapture`
    println!("ID JSON = {}", serialized);
}
