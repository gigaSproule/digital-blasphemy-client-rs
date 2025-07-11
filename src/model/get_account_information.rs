use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccountInformationResponse {
    pub db_core: GetAccountInformationDBCore,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccountInformationDBCore {
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub active: bool,
    pub display_name: String,
    pub id: u64,
    pub lifetime: bool,
    pub plus: bool,
}
