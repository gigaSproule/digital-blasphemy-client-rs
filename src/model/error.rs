use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: u64,
    pub description: String,
    pub errors: Option<Vec<String>>,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let errors: String = self
            .errors
            .clone()
            .unwrap_or(vec![])
            .iter()
            .map(|error| format!("'{error}'"))
            .collect::<Vec<String>>()
            .join(", ");
        write!(
            f,
            "Code: {}, Description: {}, Errors: {}",
            self.code, self.description, errors
        )
    }
}

impl Error for ErrorResponse {}
