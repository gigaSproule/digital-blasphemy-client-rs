use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum WallpaperType {
    Single,
    Dual,
    Triple,
    Mobile,
}

impl WallpaperType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WallpaperType::Single => "single",
            WallpaperType::Dual => "dual",
            WallpaperType::Triple => "triple",
            WallpaperType::Mobile => "mobile",
        }
    }

    pub fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let str = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };
        match str.as_str() {
            "single" => Ok(WallpaperType::Single),
            "dual" => Ok(WallpaperType::Dual),
            "triple" => Ok(WallpaperType::Triple),
            "mobile" => Ok(WallpaperType::Mobile),
            _ => Err(format!("Unknown wallpaper type {input}")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Operator {
    #[serde(rename = "=")]
    Equal,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = ">=")]
    GreaterThanOrEqual,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = "<=")]
    LessThanOrEqual,
}

impl Operator {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Operator::Equal => "=",
            Operator::GreaterThan => ">",
            Operator::GreaterThanOrEqual => ">=",
            Operator::LessThan => "<",
            Operator::LessThanOrEqual => "<=",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Order {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

impl Order {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Order::Ascending => "asc",
            Order::Descending => "desc",
        }
    }
}
