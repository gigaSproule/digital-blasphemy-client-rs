use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Endpoints {
    pub api: String,
    pub image: String,
    pub thumb: String,
    pub web: String,
}
