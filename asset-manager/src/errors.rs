use std::fmt;

#[derive(Debug)]
pub enum AppErrors {
    InvalidPassword
}

impl fmt::Display for AppErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AppErrors::InvalidPassword => write!(f, "Invalid password."),
        }
    }
}