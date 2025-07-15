use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Wallpaper {
    pub id: u16,
    pub all_free: Option<bool>,
    pub comments: Option<Comments>,
    pub content: Option<String>,
    pub free: Option<bool>,
    pub name: String,
    pub paths: Paths,
    pub pickle_jar: Option<PickleJar>,
    pub rating: Option<String>, // f64
    pub resolutions: Option<Resolutions>,
    pub sku: Option<String>,
    pub tags: Option<HashMap<String, Tag>>,
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Comments {
    pub comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Comment {
    pub id: String,
    pub author_id: String,
    pub author_display: String,
    pub content: String,
    pub rating: String,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Paths {
    pub api: String,
    pub thumb: String,
    pub web: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PickleJar {
    pub parent: String,
    pub siblings: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Resolutions {
    pub single: Vec<Resolution>,
    pub dual: Option<Vec<Resolution>>,
    pub triple: Option<Vec<Resolution>>,
    pub mobile: Option<Vec<Resolution>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Resolution {
    pub label: String,
    pub width: String,
    pub height: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}
