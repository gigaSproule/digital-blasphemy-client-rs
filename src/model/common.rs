use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    mod wallpaper_type_test {
        use super::*;

        #[test]
        fn wallpaper_type_as_str_single() {
            assert_eq!(WallpaperType::Single.as_str(), "single");
        }

        #[test]
        fn wallpaper_type_as_str_dual() {
            assert_eq!(WallpaperType::Dual.as_str(), "dual");
        }

        #[test]
        fn wallpaper_type_as_str_triple() {
            assert_eq!(WallpaperType::Triple.as_str(), "triple");
        }

        #[test]
        fn wallpaper_type_as_str_mobile() {
            assert_eq!(WallpaperType::Mobile.as_str(), "mobile");
        }

        #[test]
        fn wallpaper_type_from_str_case_sensitive_single_lowercase() {
            assert_eq!(
                WallpaperType::from_str("single", false),
                Ok(WallpaperType::Single)
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_sensitive_single_uppercase() {
            assert_eq!(
                WallpaperType::from_str("SINGLE", false),
                Err("Unknown wallpaper type SINGLE".to_string())
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_insensitive_single_lowercase() {
            assert_eq!(
                WallpaperType::from_str("single", true),
                Ok(WallpaperType::Single)
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_insensitive_single_uppercase() {
            assert_eq!(
                WallpaperType::from_str("SINGLE", true),
                Ok(WallpaperType::Single)
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_sensitive_dual_lowercase() {
            assert_eq!(
                WallpaperType::from_str("dual", false),
                Ok(WallpaperType::Dual)
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_sensitive_dual_uppercase() {
            assert_eq!(
                WallpaperType::from_str("DUAL", false),
                Err("Unknown wallpaper type DUAL".to_string())
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_insensitive_dual_lowercase() {
            assert_eq!(
                WallpaperType::from_str("dual", true),
                Ok(WallpaperType::Dual)
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_insensitive_dual_uppercase() {
            assert_eq!(
                WallpaperType::from_str("DUAL", true),
                Ok(WallpaperType::Dual)
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_sensitive_triple_lowercase() {
            assert_eq!(
                WallpaperType::from_str("triple", false),
                Ok(WallpaperType::Triple)
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_sensitive_triple_uppercase() {
            assert_eq!(
                WallpaperType::from_str("TRIPLE", false),
                Err("Unknown wallpaper type TRIPLE".to_string())
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_insensitive_triple_lowercase() {
            assert_eq!(
                WallpaperType::from_str("triple", true),
                Ok(WallpaperType::Triple)
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_insensitive_triple_uppercase() {
            assert_eq!(
                WallpaperType::from_str("TRIPLE", true),
                Ok(WallpaperType::Triple)
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_sensitive_mobile_lowercase() {
            assert_eq!(
                WallpaperType::from_str("mobile", false),
                Ok(WallpaperType::Mobile)
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_sensitive_mobile_uppercase() {
            assert_eq!(
                WallpaperType::from_str("MOBILE", false),
                Err("Unknown wallpaper type MOBILE".to_string())
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_insensitive_mobile_lowercase() {
            assert_eq!(
                WallpaperType::from_str("mobile", true),
                Ok(WallpaperType::Mobile)
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_insensitive_mobile_uppercase() {
            assert_eq!(
                WallpaperType::from_str("MOBILE", true),
                Ok(WallpaperType::Mobile)
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_sensitive_unknown() {
            assert_eq!(
                WallpaperType::from_str("unknown", false),
                Err("Unknown wallpaper type unknown".to_string())
            );
        }

        #[test]
        fn wallpaper_type_from_str_case_insensitive_unknown() {
            assert_eq!(
                WallpaperType::from_str("unknown", true),
                Err("Unknown wallpaper type unknown".to_string())
            );
        }
    }

    mod operator_test {
        use super::*;

        #[test]
        fn operator_as_str_equal() {
            assert_eq!(Operator::Equal.as_str(), "=");
        }

        #[test]
        fn operator_as_str_greater_than() {
            assert_eq!(Operator::GreaterThan.as_str(), ">");
        }

        #[test]
        fn operator_as_str_greater_than_or_equal() {
            assert_eq!(Operator::GreaterThanOrEqual.as_str(), ">=");
        }

        #[test]
        fn operator_as_str_less_than() {
            assert_eq!(Operator::LessThan.as_str(), "<");
        }

        #[test]
        fn operator_as_str_less_than_or_equal() {
            assert_eq!(Operator::LessThanOrEqual.as_str(), "<=");
        }
    }

    mod order_test {
        use super::*;

        #[test]
        fn order_as_str_ascending() {
            assert_eq!(Order::Ascending.as_str(), "asc");
        }

        #[test]
        fn order_as_str_descending() {
            assert_eq!(Order::Descending.as_str(), "desc");
        }
    }
}
