use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: u64,
    pub description: String,
    pub errors: Vec<String>,
}
