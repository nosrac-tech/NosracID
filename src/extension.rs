use serde::{Serialize, Deserialize};
use chrono::{NaiveDate};


/// List of certified extensions
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub enum CertifiedExtension {
    Passport(Passport),
    US_SSN(US_SSN),
    US_DLN(US_DLN),
    UK_NIN(UK_NIN),
    UK_DLN(UK_DLN),
}


/// US Passport Number
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub struct Passport {
    passport_num: String,
    country: String,
    country_code: String,
    place_of_birth: String,
    issuing_authority: String,
    issue_date: NaiveDate,
    expiration_date: NaiveDate,
}

impl Passport {
    /// Creates a new Passport Number
    pub fn new(_passport_num: String,
               _country: String,
               _country_code: String,
               _place_of_birth: String,
               _issuing_authority: String,
               _issue_date: NaiveDate,
               _expiration_date: NaiveDate
        ) -> Option<Passport> {

        Some(Passport {
            passport_num: _passport_num,
            country: _country,
            country_code: _country_code,
            place_of_birth: _place_of_birth,
            issuing_authority: _issuing_authority,
            issue_date: _issue_date,
            expiration_date: _expiration_date,
        })
    }
}

/// Social Security numbers are stored
/// as a String object so that it can validated more easily.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub struct US_SSN {
    ssn: String,
}

impl US_SSN {
    /// Creates a new SSN
    pub fn new(_ssn: String) -> Option<US_SSN> {
        // Ensures that social security number is numeric only and meets requirements
        let social: String = _ssn.chars().filter(|c| c.is_digit(10)).collect();
        if social.chars().count() == 9 {
            Some(US_SSN {
                ssn: _ssn,
            })
        } else {
            None
        }

    }
}

/// US Driver License number
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub struct US_DLN {
    dln: String,
}

impl US_DLN {
    /// Creates a new SSN
    pub fn new(_dln: String) -> US_DLN {
        // Because DL # do not follow a standard format, no validation is necessary
        US_DLN {
            dln: _dln,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub struct UK_NIN {
    nin: String,
}

impl UK_NIN {
    /// Creates a new Passport Number
    pub fn new(_nin: String) -> Option<UK_NIN> {

        // These are UTF-8 encoded, so each character
        // is a single byte. If these characters were
        // differently encoded, a different subsetting strategy should be used.
        let mut first_seq = _nin[0..3].chars();
        let mut second_seq = _nin[3..9].chars();
        let mut final_seq = _nin[8..].chars();

        // ensures that passport # is numeric only
        if _nin.chars().count() == 9 {
            if first_seq.all(char::is_numeric) {
                if second_seq.all(char::is_alphabetic) {
                    if final_seq.all(char::is_numeric) {
                       return Some(UK_NIN {
                           nin: _nin,
                       })
                    }
                }
            }
        }
        None
    }
}

/// UK Driver License number
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub struct UK_DLN {
    dln: String,
}

impl UK_DLN {
    /// Creates a new SSN
    pub fn new(_dln: String) -> UK_DLN {
        // Because DL # do not follow a standard format, no validation is necessary
        UK_DLN {
            dln: _dln,
        }
    }
}
