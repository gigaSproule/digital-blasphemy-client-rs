use crate::{model::Operator, model::common_response::Endpoints, model::wallpaper::Wallpaper};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetWallpaperResponse {
    pub db_core: GetWallpaperDBCore,
    pub wallpaper: Option<Wallpaper>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetWallpaperDBCore {
    pub timestamp: i64,
    pub endpoints: Endpoints,
    pub request: GetWallpaperDBCoreRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetWallpaperDBCoreRequest {
    pub params: GetWallpaperResponseParams,
    pub query: GetWallpaperResponseQuery,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetWallpaperResponseParams {
    pub wallpaper_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetWallpaperResponseQuery {
    pub filter_res_height: i64,
    pub filter_res_operator: Operator,
    pub filter_res_operator_height: Option<Operator>,
    pub filter_res_operator_width: Option<Operator>,
    pub filter_res_width: i64,
    pub show_comments: bool,
    pub show_pickle_jar: bool,
    pub show_resolutions: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetWallpaperRequest {
    pub wallpaper_id: u16,
    pub filter_res_height: i64,
    pub filter_res_operator: Operator,
    pub filter_res_operator_height: Operator,
    pub filter_res_operator_width: Operator,
    pub filter_res_width: i64,
    pub show_comments: bool,
    pub show_pickle_jar: bool,
    pub show_resolutions: bool,
}

impl GetWallpaperRequest {
    pub fn builder() -> GetWallpaperRequestBuilder {
        GetWallpaperRequestBuilder::new()
    }
}

#[derive(Clone)]
pub struct GetWallpaperRequestBuilder {
    get_wallpaper_request: GetWallpaperRequest,
}

impl GetWallpaperRequestBuilder {
    pub(crate) fn new() -> Self {
        GetWallpaperRequestBuilder {
            get_wallpaper_request: GetWallpaperRequest {
                wallpaper_id: 0,
                filter_res_height: 0,
                filter_res_operator: Operator::GreaterThanOrEqual,
                filter_res_operator_height: Operator::GreaterThanOrEqual,
                filter_res_operator_width: Operator::GreaterThanOrEqual,
                filter_res_width: 0,
                show_comments: false,
                show_pickle_jar: false,
                show_resolutions: true,
            },
        }
    }

    pub fn build(self) -> GetWallpaperRequest {
        if self.get_wallpaper_request.wallpaper_id == 0 {
            panic!("Wallpaper ID must be provided.");
        }
        self.get_wallpaper_request
    }

    pub fn wallpaper_id(mut self, wallpaper_id: u16) -> GetWallpaperRequestBuilder {
        self.get_wallpaper_request.wallpaper_id = wallpaper_id;
        self
    }

    pub fn filter_res_height(mut self, filter_res_height: i64) -> GetWallpaperRequestBuilder {
        self.get_wallpaper_request.filter_res_height = filter_res_height;
        self
    }

    pub fn filter_res_operator(
        mut self,
        filter_res_operator: Operator,
    ) -> GetWallpaperRequestBuilder {
        self.get_wallpaper_request.filter_res_operator = filter_res_operator;
        self
    }

    pub fn filter_res_operator_height(
        mut self,
        filter_res_operator_height: Operator,
    ) -> GetWallpaperRequestBuilder {
        self.get_wallpaper_request.filter_res_operator_height = filter_res_operator_height;
        self
    }

    pub fn filter_res_operator_width(
        mut self,
        filter_res_operator_width: Operator,
    ) -> GetWallpaperRequestBuilder {
        self.get_wallpaper_request.filter_res_operator_width = filter_res_operator_width;
        self
    }

    pub fn filter_res_width(mut self, filter_res_width: i64) -> GetWallpaperRequestBuilder {
        self.get_wallpaper_request.filter_res_width = filter_res_width;
        self
    }

    pub fn show_comments(mut self, show_comments: bool) -> GetWallpaperRequestBuilder {
        self.get_wallpaper_request.show_comments = show_comments;
        self
    }

    pub fn show_pickle_jar(mut self, show_pickle_jar: bool) -> GetWallpaperRequestBuilder {
        self.get_wallpaper_request.show_pickle_jar = show_pickle_jar;
        self
    }

    pub fn show_resolutions(mut self, show_resolutions: bool) -> GetWallpaperRequestBuilder {
        self.get_wallpaper_request.show_resolutions = show_resolutions;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_wallpaper_request_builder_test {
        use super::*;

        #[test]
        #[should_panic(expected = "Wallpaper ID must be provided.")]
        fn test_get_wallpaper_request_builder_rejects_build_when_wallpaper_id_not_provided() {
            GetWallpaperRequest::builder().build();
        }

        #[test]
        fn test_get_wallpaper_request_builder_sets_defaults() {
            let get_wallpaper_request = GetWallpaperRequest::builder().wallpaper_id(1).build();

            assert_eq!(get_wallpaper_request.wallpaper_id, 1);
            assert_eq!(get_wallpaper_request.filter_res_height, 0);
            assert_eq!(
                get_wallpaper_request.filter_res_operator,
                Operator::GreaterThanOrEqual
            );
            assert_eq!(
                get_wallpaper_request.filter_res_operator_height,
                Operator::GreaterThanOrEqual
            );
            assert_eq!(
                get_wallpaper_request.filter_res_operator_width,
                Operator::GreaterThanOrEqual
            );
            assert_eq!(get_wallpaper_request.filter_res_width, 0);
            assert_eq!(get_wallpaper_request.show_comments, false);
            assert_eq!(get_wallpaper_request.show_pickle_jar, false);
            assert_eq!(get_wallpaper_request.show_resolutions, true);
        }

        #[test]
        fn test_get_wallpaper_request_builder_overrides_all_defaults() {
            let get_wallpaper_request = GetWallpaperRequest::builder()
                .wallpaper_id(1)
                .filter_res_height(100)
                .filter_res_operator(Operator::LessThan)
                .filter_res_operator_height(Operator::LessThan)
                .filter_res_operator_width(Operator::LessThan)
                .filter_res_width(200)
                .show_comments(true)
                .show_pickle_jar(true)
                .show_resolutions(false)
                .build();

            assert_eq!(get_wallpaper_request.wallpaper_id, 1);
            assert_eq!(get_wallpaper_request.filter_res_height, 100);
            assert_eq!(
                get_wallpaper_request.filter_res_operator,
                Operator::LessThan
            );
            assert_eq!(
                get_wallpaper_request.filter_res_operator_height,
                Operator::LessThan
            );
            assert_eq!(
                get_wallpaper_request.filter_res_operator_width,
                Operator::LessThan
            );
            assert_eq!(get_wallpaper_request.filter_res_width, 200);
            assert_eq!(get_wallpaper_request.show_comments, true);
            assert_eq!(get_wallpaper_request.show_pickle_jar, true);
            assert_eq!(get_wallpaper_request.show_resolutions, false);
        }
    }
}
