use serde::{Serialize, Deserialize};


/// List of certified extensions
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub enum CertifiedExtension {
    US_SSN(US_SSN),
    US_DLN(US_DLN),
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
    ssn: String,
}

impl US_DLN {
    /// Creates a new SSN
    pub fn new(_ssn: String) -> US_SSN {
        // Because DL # do not follow a standard format, no validation is necessary
        US_SSN {
            ssn: _ssn,
        }
    }
}
