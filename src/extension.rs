use serde::{Serialize, Deserialize};


/// List of certified extensions
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub enum CertifiedExtension {
    US_SSN(US_SSN),
    US_DLN(US_DLN),
    US_PN(),
    UK_NIN(),
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

/// US Passport Number
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub struct US_PN {
    passport: String,
}

impl US_PN {
    /// Creates a new Passport Number
    pub fn new(_passport: String) -> Option<US_PN> {

        // ensures that passport # is numeric only
        let _passport: String = _passport.chars().filter(|c| c.is_digit(10)).collect();

        // makes sure passport number is longer than 8 digits
        if _passport.chars().count() <= 8 {
            Some(US_PN {
                passport: _passport,
            })
        } else {
            None
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
