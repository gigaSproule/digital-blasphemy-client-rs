use crate::{
    model::common_response::Endpoints,
    model::wallpaper::Wallpaper,
    model::{Operator, Order},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetWallpapersResponse {
    pub db_core: GetWallpapersDBCore,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetWallpapersDBCore {
    pub timestamp: i64,
    pub endpoints: Endpoints,
    pub request: GetWallpapersDBCoreRequest,
    pub total_pages: u16,
    pub wallpapers: HashMap<String, Wallpaper>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetWallpapersDBCoreRequest {
    pub query: GetWallpapersResponseQuery,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetWallpapersResponseQuery {
    pub filter_date_day: Option<u8>,
    pub filter_date_month: Option<u8>,
    pub filter_date_year: Option<u16>,
    pub filter_date_operator: Operator,
    pub filter_gallery: Option<Vec<u16>>,
    pub filter_rating: Option<f32>,
    pub filter_rating_operator: Option<Operator>,
    pub filter_res_operator_height: Option<Operator>,
    pub filter_res_operator_width: Option<Operator>,
    pub filter_res_height: i64,
    pub filter_res_operator: Operator,
    pub filter_res_width: i64,
    pub filter_tag: Option<Vec<u16>>,
    pub limit: u16,
    pub order: Order,
    pub order_by: GetWallpapersOrderBy,
    pub page: u16,
    pub s: Option<String>,
    pub show_comments: bool,
    pub show_pickle_jar: bool,
    pub show_resolutions: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GetWallpapersOrderBy {
    Date,
    Name,
}

impl GetWallpapersOrderBy {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            GetWallpapersOrderBy::Date => "date",
            GetWallpapersOrderBy::Name => "name",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetWallpapersRequest {
    pub filter_date_day: u8,
    pub filter_date_month: u8,
    pub filter_date_year: u16,
    pub filter_date_operator: Operator,
    pub filter_gallery: Vec<u16>,
    pub filter_rating: f32,
    pub filter_rating_operator: Operator,
    pub filter_res_height: i64,
    pub filter_res_operator: Operator,
    pub filter_res_operator_height: Operator,
    pub filter_res_operator_width: Operator,
    pub filter_res_width: i64,
    pub filter_tag: Vec<u16>,
    pub limit: u16,
    pub order: Order,
    pub order_by: GetWallpapersOrderBy,
    pub page: u16,
    pub s: String,
    pub show_comments: bool,
    pub show_pickle_jar: bool,
    pub show_resolutions: bool,
}

impl GetWallpapersRequest {
    pub fn builder() -> GetWallpapersRequestQueryBuilder {
        GetWallpapersRequestQueryBuilder::new()
    }
}

#[derive(Clone)]
pub struct GetWallpapersRequestQueryBuilder {
    get_wallpapers_request: GetWallpapersRequest,
}

#[allow(dead_code)]
impl GetWallpapersRequestQueryBuilder {
    pub(crate) fn new() -> Self {
        GetWallpapersRequestQueryBuilder {
            get_wallpapers_request: GetWallpapersRequest {
                filter_date_day: 0,
                filter_date_month: 0,
                filter_date_year: 0,
                filter_date_operator: Operator::GreaterThanOrEqual,
                filter_gallery: vec![],
                filter_rating: 0_f32,
                filter_rating_operator: Operator::GreaterThanOrEqual,
                filter_res_height: 0,
                filter_res_operator: Operator::GreaterThanOrEqual,
                filter_res_operator_height: Operator::GreaterThanOrEqual,
                filter_res_operator_width: Operator::GreaterThanOrEqual,
                filter_res_width: 0,
                filter_tag: vec![],
                limit: 10,
                order: Order::Ascending,
                order_by: GetWallpapersOrderBy::Date,
                page: 1,
                s: "".to_string(),
                show_comments: false,
                show_pickle_jar: false,
                show_resolutions: true,
            },
        }
    }

    pub fn build(self) -> GetWallpapersRequest {
        self.get_wallpapers_request
    }

    pub fn filter_date_day(mut self, filter_date_day: u8) -> GetWallpapersRequestQueryBuilder {
        if !(1..=31).contains(&filter_date_day) {
            panic!("Filter date day must be between 1 and 31.");
        }
        self.get_wallpapers_request.filter_date_day = filter_date_day;
        self
    }

    pub fn filter_date_month(mut self, filter_date_month: u8) -> GetWallpapersRequestQueryBuilder {
        if !(1..=12).contains(&filter_date_month) {
            panic!("Filter date month must be between 1 and 12.");
        }
        self.get_wallpapers_request.filter_date_month = filter_date_month;
        self
    }

    pub fn filter_date_year(mut self, filter_date_year: u16) -> GetWallpapersRequestQueryBuilder {
        if filter_date_year < 1997 {
            panic!("Filter date year must be from 1997 inclusive.");
        }
        self.get_wallpapers_request.filter_date_year = filter_date_year;
        self
    }

    pub fn filter_date_operator(
        mut self,
        filter_date_operator: Operator,
    ) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.filter_date_operator = filter_date_operator;
        self
    }

    pub fn filter_gallery(mut self, filter_gallery: Vec<u16>) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.filter_gallery = filter_gallery;
        self
    }

    pub fn filter_rating(mut self, filter_rating: f32) -> GetWallpapersRequestQueryBuilder {
        if !(1_f32..=5_f32).contains(&filter_rating) {
            panic!("Filter rating must be between 1 and 5.");
        }
        self.get_wallpapers_request.filter_rating = filter_rating;
        self
    }

    pub fn filter_rating_operator(
        mut self,
        filter_rating_operator: Operator,
    ) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.filter_rating_operator = filter_rating_operator;
        self
    }

    pub fn filter_res_height(mut self, filter_res_height: i64) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.filter_res_height = filter_res_height;
        self
    }

    pub fn filter_res_operator(
        mut self,
        filter_res_operator: Operator,
    ) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.filter_res_operator = filter_res_operator;
        self
    }

    pub fn filter_res_operator_height(
        mut self,
        filter_res_operator_height: Operator,
    ) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.filter_res_operator_height = filter_res_operator_height;
        self
    }

    pub fn filter_res_operator_width(
        mut self,
        filter_res_operator_width: Operator,
    ) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.filter_res_operator_width = filter_res_operator_width;
        self
    }

    pub fn filter_res_width(mut self, filter_res_width: i64) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.filter_res_width = filter_res_width;
        self
    }

    pub fn filter_tag(mut self, filter_tag: Vec<u16>) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.filter_tag = filter_tag;
        self
    }

    pub fn limit(mut self, limit: u16) -> GetWallpapersRequestQueryBuilder {
        if !(1..=50).contains(&limit) {
            panic!("Limit must be between 1 and 50.");
        }
        self.get_wallpapers_request.limit = limit;
        self
    }

    pub fn order(mut self, order: Order) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.order = order;
        self
    }

    pub fn order_by(mut self, order_by: GetWallpapersOrderBy) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.order_by = order_by;
        self
    }

    pub fn page(mut self, page: u16) -> GetWallpapersRequestQueryBuilder {
        if page < 1 {
            panic!("Page must be greater than 0.");
        }
        self.get_wallpapers_request.page = page;
        self
    }

    pub fn s(mut self, s: String) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.s = s;
        self
    }

    pub fn show_comments(mut self, show_comments: bool) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.show_comments = show_comments;
        self
    }

    pub fn show_pickle_jar(mut self, show_pickle_jar: bool) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.show_pickle_jar = show_pickle_jar;
        self
    }

    pub fn show_resolutions(mut self, show_resolutions: bool) -> GetWallpapersRequestQueryBuilder {
        self.get_wallpapers_request.show_resolutions = show_resolutions;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_wallpapers_request_query_builder {
        use super::*;

        #[test]
        #[should_panic(expected = "Filter date day must be between 1 and 31.")]
        fn test_get_wallpapers_request_query_builder_filter_date_day_rejects_less_than_1() {
            GetWallpapersRequestQueryBuilder::new().filter_date_day(0);
        }

        #[test]
        #[should_panic(expected = "Filter date day must be between 1 and 31.")]
        fn test_get_wallpapers_request_query_builder_filter_date_day_rejects_greater_than_31() {
            GetWallpapersRequestQueryBuilder::new().filter_date_day(32);
        }

        #[test]
        fn test_get_wallpapers_request_query_builder_filter_date_day_accepts_1() {
            GetWallpapersRequestQueryBuilder::new().filter_date_day(1);
        }

        #[test]
        fn test_get_wallpapers_request_query_builder_filter_date_day_accepts_31() {
            GetWallpapersRequestQueryBuilder::new().filter_date_day(1);
        }

        #[test]
        #[should_panic(expected = "Filter date month must be between 1 and 12.")]
        fn test_get_wallpapers_request_query_builder_filter_date_month_rejects_less_than_1() {
            GetWallpapersRequestQueryBuilder::new().filter_date_month(0);
        }

        #[test]
        #[should_panic(expected = "Filter date month must be between 1 and 12.")]
        fn test_get_wallpapers_request_query_builder_filter_date_month_rejects_greater_than_31() {
            GetWallpapersRequestQueryBuilder::new().filter_date_month(32);
        }

        #[test]
        fn test_get_wallpapers_request_query_builder_filter_date_month_accepts_1() {
            GetWallpapersRequestQueryBuilder::new().filter_date_month(1);
        }

        #[test]
        fn test_get_wallpapers_request_query_builder_filter_date_month_accepts_31() {
            GetWallpapersRequestQueryBuilder::new().filter_date_month(1);
        }

        #[test]
        #[should_panic(expected = "Filter date year must be from 1997 inclusive.")]
        fn test_get_wallpapers_request_query_builder_filter_date_year_rejects_less_than_1997() {
            GetWallpapersRequestQueryBuilder::new().filter_date_year(1996);
        }

        #[test]
        fn test_get_wallpapers_request_query_builder_filter_date_month_accepts_1997() {
            GetWallpapersRequestQueryBuilder::new().filter_date_year(1997);
        }

        #[test]
        #[should_panic(expected = "Filter rating must be between 1 and 5.")]
        fn test_get_wallpapers_request_query_builder_filter_rating_rejects_less_than_1() {
            GetWallpapersRequestQueryBuilder::new().filter_rating(0.99);
        }

        #[test]
        #[should_panic(expected = "Filter rating must be between 1 and 5.")]
        fn test_get_wallpapers_request_query_builder_filter_rating_rejects_greater_than_5() {
            GetWallpapersRequestQueryBuilder::new().filter_rating(5.01);
        }

        #[test]
        fn test_get_wallpapers_request_query_builder_filter_rating_accepts_1() {
            GetWallpapersRequestQueryBuilder::new().filter_rating(1_f32);
        }

        #[test]
        fn test_get_wallpapers_request_query_builder_filter_rating_accepts_5() {
            GetWallpapersRequestQueryBuilder::new().filter_rating(5_f32);
        }

        #[test]
        #[should_panic(expected = "Limit must be between 1 and 50.")]
        fn test_get_wallpapers_request_query_builder_limit_rejects_less_than_1() {
            GetWallpapersRequestQueryBuilder::new().limit(0);
        }

        #[test]
        #[should_panic(expected = "Limit must be between 1 and 50.")]
        fn test_get_wallpapers_request_query_builder_limit_rejects_greater_than_50() {
            GetWallpapersRequestQueryBuilder::new().limit(51);
        }

        #[test]
        fn test_get_wallpapers_request_query_builder_limit_accepts_1() {
            GetWallpapersRequestQueryBuilder::new().limit(1);
        }

        #[test]
        fn test_get_wallpapers_request_query_builder_limit_accepts_50() {
            GetWallpapersRequestQueryBuilder::new().limit(50);
        }

        #[test]
        #[should_panic(expected = "Page must be greater than 0.")]
        fn test_get_wallpapers_request_query_builder_page_rejects_less_than_1() {
            GetWallpapersRequestQueryBuilder::new().page(0);
        }

        #[test]
        fn test_get_wallpapers_request_query_builder_page_accepts_1() {
            GetWallpapersRequestQueryBuilder::new().page(1);
        }
    }
}
