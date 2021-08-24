use chrono::{NaiveDate};
use uuid::{Uuid};

/// This is the basis for the ID System.
/// The ID Struct is the base unit and an ID can (and should)
/// be used everywhere it can be. <br>
/// TODO: Storing and retrieving ID's needs to be supported.
/// Building ID's manually in code is fine for now, but really a database
/// needs to be stood up.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Id {
    pub uuid: Uuid,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub emails: Vec<String>,
    //pub address: String,
}

impl Id {

    /// Instantiates a new empty ID from a template.
    pub fn new() -> Id {
        Id {
            // This is insufficient, while UUID collision is unlikely,
            // it is possible. UUID's should be hashed on creation, test if UUID
            // exists via hash-match (if match is found, regenerate hash) otherwise create ID.
            uuid: Uuid::new_v4(),
            first_name: String::from(""),
            middle_name: String::from(""),
            last_name: String::from(""),
            date_of_birth: NaiveDate::from_ymd(1970, 01, 01),
            emails: Vec::new(),
        }
    }

    /// Instantiates an ID object when base-fields are known
    /// For example, if the information is queried from a database,
    /// this function will return an object from the field info
    pub fn new_from_data(_uuid: Uuid, _first_name: String, _middle_name: String, _last_name: String,
                _date_of_birth: NaiveDate, _emails: Vec<String>) -> Id {
        Id {
            // This is insufficient, while UUID collision is unlikely,
            // it is possible. UUID's should be hashed on creation, test if UUID
            // exists via hash-match (if match is found, regenerate hash) otherwise create ID.
            uuid: _uuid,
            first_name: _first_name,
            middle_name: _middle_name,
            last_name: _last_name,
            date_of_birth: _date_of_birth,
            emails: _emails,
        }
    }

}
