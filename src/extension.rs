/// List of certified (Nosrac created) extensions
#[derive(PartialEq, Eq, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum CertifiedExtensionType {
    US_SSN,
    US_DLN,
}

/// Despite SSNs being a number, it is stored
/// as a String object so that it can validated more easily.
#[derive(PartialEq, Eq, Clone, Debug)]
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
