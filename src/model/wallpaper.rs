use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallpaper {
    pub id: u16,
    pub all_free: Option<bool>,
    pub content: Option<String>,
    pub free: Option<bool>,
    pub name: String,
    pub paths: Paths,
    pub rating: Option<String>, // f64
    pub resolutions: Option<Resolutions>,
    pub sku: Option<String>,
    pub tags: Option<HashMap<String, Tag>>,
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paths {
    pub api: String,
    pub thumb: String,
    pub web: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resolutions {
    pub single: Vec<Resolution>,
    pub dual: Option<Vec<Resolution>>,
    pub triple: Option<Vec<Resolution>>,
    pub mobile: Option<Vec<Resolution>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resolution {
    pub label: String,
    pub width: String,
    pub height: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}
