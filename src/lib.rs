mod model;

pub use model::*;

pub use crate::model::{
    DownloadWallpaperRequest, DownloadWallpaperResponse, ErrorResponse, GetWallpaperRequest,
    GetWallpaperResponse, GetWallpapersOrderBy, GetWallpapersRequest, GetWallpapersResponse,
    Wallpaper,
};
use log::{debug, log_enabled, Level};
use reqwest::{RequestBuilder, Response, StatusCode};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub struct DigitalBlasphemyClient {
    authorization: String,
    client: reqwest::Client,
    base_url: String,
}

impl DigitalBlasphemyClient {
    pub fn new(api_key: String) -> Result<DigitalBlasphemyClient, Box<dyn Error>> {
        Ok(DigitalBlasphemyClient {
            authorization: format!("Bearer {api_key}"),
            client: reqwest::Client::builder().build()?,
            base_url: "https://api.digitalblasphemy.com".to_string(),
        })
    }

    #[cfg(test)]
    pub(crate) fn new_test(
        api_key: String,
        base_url: String,
    ) -> Result<DigitalBlasphemyClient, Box<dyn Error>> {
        Ok(DigitalBlasphemyClient {
            authorization: format!("Bearer {api_key}"),
            client: reqwest::Client::builder().build()?,
            base_url,
        })
    }

    pub async fn get_user_information(
        &self,
    ) -> Result<GetAccountInformationResponse, ErrorResponse> {
        let get_account_information_response = self
            .get_request_json::<GetAccountInformationResponse>(
                &vec![],
                format!("{}/v2/core/account", self.base_url),
            )
            .await?;
        Ok(get_account_information_response)
    }

    pub async fn get_wallpapers(
        &self,
        request: GetWallpapersRequest,
    ) -> Result<GetWallpapersResponse, ErrorResponse> {
        let get_wallpaper_response = self
            .get_request_json::<GetWallpapersResponse>(
                &Self::get_wallpapers_query(&request),
                format!("{}/v2/core/wallpapers", self.base_url),
            )
            .await?;
        Ok(get_wallpaper_response)
    }

    fn get_wallpapers_query(request: &GetWallpapersRequest) -> Vec<(&str, String)> {
        let mut query: Vec<(&str, String)> = vec![];
        if request.filter_date_day != 0 {
            query.push(("filter_date_day", request.filter_date_day.to_string()));
        }
        if request.filter_date_month != 0 {
            query.push(("filter_date_month", request.filter_date_month.to_string()));
        }
        if request.filter_date_year != 0 {
            query.push(("filter_date_year", request.filter_date_year.to_string()));
        }
        query.push((
            "filter_date_operator",
            request.filter_date_operator.as_str().to_string(),
        ));
        request
            .filter_gallery
            .iter()
            .for_each(|gallery| query.push(("filter_gallery", gallery.to_string())));
        if request.filter_rating != 0_f32 {
            query.push(("filter_rating", request.filter_rating.to_string()));
        }
        query.push((
            "filter_rating_operator",
            request.filter_rating_operator.as_str().to_string(),
        ));
        if request.filter_res_height != 0 {
            query.push(("filter_res_height", request.filter_res_height.to_string()));
        }
        query.push((
            "filter_res_operator",
            request.filter_res_operator.as_str().to_string(),
        ));
        query.push((
            "filter_res_operator_height",
            request.filter_res_operator_height.as_str().to_string(),
        ));
        query.push((
            "filter_res_operator_width",
            request.filter_res_operator_width.as_str().to_string(),
        ));
        if request.filter_res_width != 0 {
            query.push(("filter_res_width", request.filter_res_width.to_string()));
        }
        request
            .filter_tag
            .iter()
            .for_each(|tag| query.push(("filter_tag", tag.to_string())));
        query.push(("limit", request.limit.to_string()));
        query.push(("order", request.order.as_str().to_string()));
        if request.order_by != GetWallpapersOrderBy::Date {
            query.push(("order_by", request.order_by.as_str().to_string()));
        }
        query.push(("page", request.page.to_string()));
        if !request.s.is_empty() {
            query.push(("s", request.s.to_string()));
        }
        query.push(("show_comments", request.show_comments.to_string()));
        query.push(("show_pickle_jar", request.show_pickle_jar.to_string()));
        query.push(("show_resolutions", request.show_resolutions.to_string()));
        query
    }

    pub async fn get_wallpaper(
        &self,
        request: &GetWallpaperRequest,
    ) -> Result<Option<Wallpaper>, ErrorResponse> {
        let get_wallpaper_response = self
            .get_request_json::<GetWallpaperResponse>(
                &Self::get_wallpaper_query(request),
                format!(
                    "{}/v2/core/wallpaper/{}",
                    self.base_url, request.wallpaper_id
                ),
            )
            .await?;
        Ok(get_wallpaper_response.wallpaper)
    }

    fn get_wallpaper_query(request: &GetWallpaperRequest) -> Vec<(&str, String)> {
        let mut query: Vec<(&str, String)> = vec![];
        if request.filter_res_height != 0 {
            let filter_res_height = request.filter_res_height;
            query.push(("filter_res_height", filter_res_height.to_string()))
        }
        if request.filter_res_width != 0 {
            let filter_res_width = request.filter_res_width;
            query.push(("filter_res_width", filter_res_width.to_string()));
        }
        query.push((
            "filter_res_operator",
            request.filter_res_operator.as_str().to_string(),
        ));
        query.push((
            "filter_res_operator_height",
            request.filter_res_operator_height.as_str().to_string(),
        ));
        query.push((
            "filter_res_operator_width",
            request.filter_res_operator_width.as_str().to_string(),
        ));
        query.push(("show_comments", request.show_comments.to_string()));
        query.push(("show_pickle_jar", request.show_pickle_jar.to_string()));
        query.push(("show_resolutions", request.show_resolutions.to_string()));
        query
    }

    pub async fn download_wallpaper(
        &self,
        filename: impl AsRef<Path>,
        request: DownloadWallpaperRequest,
    ) -> Result<(), ErrorResponse> {
        let download_wallpaper_response = self
            .get_request_json::<DownloadWallpaperResponse>(
                &Self::download_query(&request),
                format!(
                    "{}/v2/core/download/wallpaper/{}/{}/{}/{}",
                    self.base_url,
                    request.wallpaper_type.as_str(),
                    request.width,
                    request.height,
                    request.wallpaper_id
                ),
            )
            .await?;

        let file_response = self
            .get_request(&vec![], download_wallpaper_response.download.url)
            .await?;

        let mut open_options = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(filename)
            .expect("Unable to create file");

        open_options
            .write_all(
                &file_response
                    .bytes()
                    .await
                    .expect("Unable to read response as bytes"),
            )
            .unwrap();

        Ok(())
    }

    fn download_query(request: &DownloadWallpaperRequest) -> Vec<(&str, String)> {
        vec![("show_watermark", request.show_watermark.to_string())]
    }

    async fn get_request_json<T: serde::de::DeserializeOwned>(
        &self,
        query: &Vec<(&str, String)>,
        url: String,
    ) -> Result<T, ErrorResponse> {
        let response = self
            .get_request(query, url)
            .await?
            .json::<T>()
            .await
            .expect("Unable to parse the body as JSON");
        Ok(response)
    }

    async fn get_request(
        &self,
        query: &Vec<(&str, String)>,
        url: String,
    ) -> Result<Response, ErrorResponse> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());
        headers.insert("Authorization", self.authorization.parse().unwrap());

        let request = self
            .client
            .request(reqwest::Method::GET, url)
            .query(&query)
            .headers(headers.clone());

        Self::debug_log(&request);

        let response = request.send().await;

        if response.is_err() {
            let error = response.unwrap_err();
            let status = error.status().unwrap();
            return Err(ErrorResponse {
                code: status.as_u16() as u64,
                description: status.canonical_reason().unwrap().to_string(),
                errors: Some(vec![error.source().unwrap().to_string()]),
            });
        }

        let unwrapped_response = response.unwrap();
        if !unwrapped_response.status().is_success() {
            if unwrapped_response.status() == StatusCode::NOT_FOUND {
                return Err(ErrorResponse {
                    code: unwrapped_response.status().as_u16() as u64,
                    description: unwrapped_response
                        .status()
                        .canonical_reason()
                        .unwrap()
                        .to_string(),
                    errors: Some(vec![
                        unwrapped_response
                            .text()
                            .await
                            .expect("Unable to parse the body as text"),
                    ]),
                });
            }
            return Err(unwrapped_response
                .json::<ErrorResponse>()
                .await
                .expect("Unable to parse the body as JSON Error Response"));
        }

        Ok(unwrapped_response)
    }

    fn debug_log(request: &RequestBuilder) {
        if log_enabled!(Level::Debug) {
            debug!("{:?}", request.try_clone().unwrap().build().unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;

    #[tokio::test]
    async fn get_user_information_can_map_successful_response() -> Result<(), Box<dyn Error>> {
        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("GET", "/v2/core/account")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(fs::read_to_string(
                "resources/get_user_information_success.json",
            )?)
            .create_async()
            .await;

        let client =
            DigitalBlasphemyClient::new_test("api_key".to_string(), server.url().to_string())?;

        let user_information = client.get_user_information().await.unwrap();

        assert_eq!(user_information.db_core.timestamp, 1);
        assert!(user_information.user.active);
        assert_eq!(user_information.user.display_name, "username".to_string());
        assert_eq!(user_information.user.id, 2);
        assert!(user_information.user.lifetime);
        assert!(user_information.user.plus);

        mock.assert_async().await;

        Ok(())
    }

    #[tokio::test]
    async fn get_user_information_can_map_error_response() -> Result<(), Box<dyn Error>> {
        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("GET", "/v2/core/account")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(fs::read_to_string("resources/unauthorised_response.json")?)
            .create_async()
            .await;

        let client =
            DigitalBlasphemyClient::new_test("api_key".to_string(), server.url().to_string())?;

        let error = client.get_user_information().await.unwrap_err();

        assert_eq!(error.code, 401);
        assert_eq!(error.description, "Unauthorized".to_string());

        mock.assert_async().await;

        Ok(())
    }

    #[tokio::test]
    async fn get_wallpapers_can_map_successful_response_fully_populated()
    -> Result<(), Box<dyn Error>> {
        let mut server = mockito::Server::new_async().await;

        let get_wallpapers_request = GetWallpapersRequest::builder().build();

        let mock = server
            .mock(
                "GET",
                mockito::Matcher::Regex(r"^/v2/core/wallpapers.*$".to_string()),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(fs::read_to_string(
                "resources/get_wallpapers_success_fully_populated.json",
            )?)
            .create_async()
            .await;

        let client =
            DigitalBlasphemyClient::new_test("api_key".to_string(), server.url().to_string())?;

        let get_wallpapers_response = client.get_wallpapers(get_wallpapers_request).await.unwrap();

        let expected_wallpaper_13 = Wallpaper {
            id: 13,
            all_free: Some(true),
            comments: Some(Comments {
                comments: vec![
                    Comment {
                        id: "14".to_string(),
                        author_id: "author ID 1".to_string(),
                        author_display: "author display 1".to_string(),
                        content: "Content 1".to_string(),
                        rating: "15".to_string(),
                        timestamp: 16,
                    },
                    Comment {
                        id: "17".to_string(),
                        author_id: "author ID 2".to_string(),
                        author_display: "author display 2".to_string(),
                        content: "Content 2".to_string(),
                        rating: "18".to_string(),
                        timestamp: 19,
                    },
                ],
            }),
            content: Some("Content 3".to_string()),
            free: Some(true),
            name: "Vulcan".to_string(),
            paths: Paths {
                api: "/wallpaper/13".to_string(),
                thumb: "/thumbnail/21x22/vulcan_thumbnail_21x22.jpg".to_string(),
                web: "/sec/vulcan/".to_string(),
            },
            pickle_jar: Some(PickleJar {
                parent: "parent 1".to_string(),
                siblings: vec!["sibling 1".to_string(), "sibling 2".to_string()],
            }),
            rating: Some("20".to_string()),
            resolutions: Some(Resolutions {
                single: vec![
                    Resolution {
                        label: "21x22".to_string(),
                        width: "21".to_string(),
                        height: "22".to_string(),
                        image: "/single/21x22/vulcan_single_21x22.jpg".to_string(),
                    },
                    Resolution {
                        label: "23x24".to_string(),
                        width: "23".to_string(),
                        height: "24".to_string(),
                        image: "/single/23x24/vulcan_single_23x24.jpg".to_string(),
                    },
                ],
                dual: Some(vec![
                    Resolution {
                        label: "25x26".to_string(),
                        width: "25".to_string(),
                        height: "26".to_string(),
                        image: "/dual/25x26/vulcan_dual_25x26.jpg".to_string(),
                    },
                    Resolution {
                        label: "27x28".to_string(),
                        width: "27".to_string(),
                        height: "28".to_string(),
                        image: "/dual/27x28/vulcan_dual_27x28.jpg".to_string(),
                    },
                ]),
                triple: Some(vec![
                    Resolution {
                        label: "29x30".to_string(),
                        width: "29".to_string(),
                        height: "30".to_string(),
                        image: "/triple/29x30/vulcan_triple_29x30.jpg".to_string(),
                    },
                    Resolution {
                        label: "31x32".to_string(),
                        width: "31".to_string(),
                        height: "32".to_string(),
                        image: "/triple/31x32/vulcan_triple_31x32.jpg".to_string(),
                    },
                ]),
                mobile: Some(vec![
                    Resolution {
                        label: "33x34".to_string(),
                        width: "33".to_string(),
                        height: "34".to_string(),
                        image: "/mobile/33x34/vulcan_mobile_33x34.jpg".to_string(),
                    },
                    Resolution {
                        label: "33x34".to_string(),
                        width: "33".to_string(),
                        height: "34".to_string(),
                        image: "/mobile/33x34/vulcan_mobile_33x34.jpg".to_string(),
                    },
                ]),
            }),
            sku: Some("vulcan".to_string()),
            tags: Some(HashMap::from([
                (
                    "35".to_string(),
                    Tag {
                        id: 35,
                        name: "Tag 1".to_string(),
                    },
                ),
                (
                    "36".to_string(),
                    Tag {
                        id: 36,
                        name: "Tag 2".to_string(),
                    },
                ),
            ])),
            timestamp: Some(37),
        };
        let expected_wallpaper_38 = Wallpaper {
            id: 38,
            all_free: Some(false),
            comments: Some(Comments {
                comments: vec![
                    Comment {
                        id: "39".to_string(),
                        author_id: "author ID 3".to_string(),
                        author_display: "author display 3".to_string(),
                        content: "Content 4".to_string(),
                        rating: "40".to_string(),
                        timestamp: 41,
                    },
                    Comment {
                        id: "42".to_string(),
                        author_id: "author ID 4".to_string(),
                        author_display: "author display 4".to_string(),
                        content: "Content 5".to_string(),
                        rating: "43".to_string(),
                        timestamp: 44,
                    },
                ],
            }),
            content: Some("Content 6".to_string()),
            free: Some(false),
            name: "Valley I".to_string(),
            paths: Paths {
                api: "/wallpaper/38".to_string(),
                thumb: "/thumbnail/46x47/valley_thumbnail_46x47.jpg".to_string(),
                web: "/sec/valley/".to_string(),
            },
            pickle_jar: Some(PickleJar {
                parent: "parent 2".to_string(),
                siblings: vec!["sibling 3".to_string(), "sibling 4".to_string()],
            }),
            rating: Some("45".to_string()),
            resolutions: Some(Resolutions {
                single: vec![
                    Resolution {
                        label: "46x47".to_string(),
                        width: "46".to_string(),
                        height: "47".to_string(),
                        image: "/single/46x47/valley_single_46x47.jpg".to_string(),
                    },
                    Resolution {
                        label: "48x49".to_string(),
                        width: "48".to_string(),
                        height: "49".to_string(),
                        image: "/single/48x49/valley_single_48x49.jpg".to_string(),
                    },
                ],
                dual: Some(vec![
                    Resolution {
                        label: "50x51".to_string(),
                        width: "50".to_string(),
                        height: "51".to_string(),
                        image: "/dual/50x51/valley_dual_50x51.jpg".to_string(),
                    },
                    Resolution {
                        label: "52x53".to_string(),
                        width: "52".to_string(),
                        height: "53".to_string(),
                        image: "/dual/52x53/valley_dual_52x53.jpg".to_string(),
                    },
                ]),
                triple: Some(vec![
                    Resolution {
                        label: "54x55".to_string(),
                        width: "54".to_string(),
                        height: "55".to_string(),
                        image: "/triple/54x55/valley_triple_54x55.jpg".to_string(),
                    },
                    Resolution {
                        label: "56x57".to_string(),
                        width: "56".to_string(),
                        height: "57".to_string(),
                        image: "/triple/56x57/valley_triple_56x57.jpg".to_string(),
                    },
                ]),
                mobile: Some(vec![
                    Resolution {
                        label: "58x59".to_string(),
                        width: "58".to_string(),
                        height: "59".to_string(),
                        image: "/mobile/58x59/valley_mobile_58x59.jpg".to_string(),
                    },
                    Resolution {
                        label: "60x61".to_string(),
                        width: "60".to_string(),
                        height: "61".to_string(),
                        image: "/mobile/60x61/valley_mobile_60x61.jpg".to_string(),
                    },
                ]),
            }),
            sku: Some("valley".to_string()),
            tags: Some(HashMap::from([
                (
                    "63".to_string(),
                    Tag {
                        id: 63,
                        name: "Tag 4".to_string(),
                    },
                ),
                (
                    "62".to_string(),
                    Tag {
                        id: 62,
                        name: "Tag 3".to_string(),
                    },
                ),
            ])),
            timestamp: Some(64),
        };
        let expected_get_wallpapers_response = GetWallpapersResponse {
            db_core: GetWallpapersDBCore {
                timestamp: 1,
                endpoints: Endpoints {
                    api: "https://api.digitalblasphemy.com/v2/core".to_string(),
                    image: "https://arcadia.digitalblasphemy.com".to_string(),
                    thumb: "https://cdn.digitalblasphemy.com".to_string(),
                    web: "https://digitalblasphemy.com".to_string(),
                },
                request: GetWallpapersDBCoreRequest {
                    query: GetWallpapersResponseQuery {
                        filter_date_day: Some(2),
                        filter_date_month: Some(3),
                        filter_date_year: Some(4),
                        filter_date_operator: Operator::Equal,
                        filter_gallery: Some(vec![5]),
                        filter_rating: Some(6_f32),
                        filter_rating_operator: Some(Operator::GreaterThanOrEqual),
                        filter_res_operator_height: Some(Operator::GreaterThanOrEqual),
                        filter_res_operator_width: Some(Operator::GreaterThanOrEqual),
                        filter_res_height: 7,
                        filter_res_operator: Operator::GreaterThanOrEqual,
                        filter_res_width: 8,
                        filter_tag: Some(vec![9]),
                        limit: 10,
                        order: Order::Ascending,
                        order_by: GetWallpapersOrderBy::Name,
                        page: 11,
                        s: Some("search".to_string()),
                        show_comments: true,
                        show_pickle_jar: true,
                        show_resolutions: true,
                    },
                },
                total_pages: 12,
                wallpapers: HashMap::from([
                    ("13".to_string(), expected_wallpaper_13),
                    ("38".to_string(), expected_wallpaper_38),
                ]),
            },
        };
        assert_eq!(get_wallpapers_response, expected_get_wallpapers_response);

        mock.assert_async().await;

        Ok(())
    }

    #[tokio::test]
    async fn get_wallpapers_can_map_successful_response_minimal_populated()
    -> Result<(), Box<dyn Error>> {
        let mut server = mockito::Server::new_async().await;

        let get_wallpapers_request = GetWallpapersRequest::builder().build();

        let mock = server
            .mock(
                "GET",
                mockito::Matcher::Regex(r"^/v2/core/wallpapers.*$".to_string()),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(fs::read_to_string(
                "resources/get_wallpapers_success_minimal_populated.json",
            )?)
            .create_async()
            .await;

        let client =
            DigitalBlasphemyClient::new_test("api_key".to_string(), server.url().to_string())?;

        let get_wallpapers_response = client.get_wallpapers(get_wallpapers_request).await.unwrap();

        let expected_wallpaper_7 = Wallpaper {
            id: 7,
            all_free: None,
            comments: None,
            content: None,
            free: None,
            name: "Vulcan".to_string(),
            paths: Paths {
                api: "/wallpaper/13".to_string(),
                thumb: "/thumbnail/21x22/vulcan_thumbnail_21x22.jpg".to_string(),
                web: "/sec/vulcan/".to_string(),
            },
            pickle_jar: None,
            rating: Some("8".to_string()),
            resolutions: Some(Resolutions {
                single: vec![
                    Resolution {
                        label: "9x10".to_string(),
                        width: "9".to_string(),
                        height: "10".to_string(),
                        image: "/single/9x10/vulcan_single_9x10.jpg".to_string(),
                    },
                    Resolution {
                        label: "11x12".to_string(),
                        width: "11".to_string(),
                        height: "12".to_string(),
                        image: "/single/11x12/vulcan_single_11x12.jpg".to_string(),
                    },
                ],
                dual: None,
                triple: None,
                mobile: None,
            }),
            sku: None,
            tags: None,
            timestamp: None,
        };
        let expected_wallpaper_13 = Wallpaper {
            id: 13,
            all_free: None,
            comments: None,
            content: None,
            free: None,
            name: "Valley I".to_string(),
            paths: Paths {
                api: "/wallpaper/13".to_string(),
                thumb: "/thumbnail/14x15/valley_thumbnail_14x15.jpg".to_string(),
                web: "/sec/valley/".to_string(),
            },
            pickle_jar: None,
            rating: None,
            resolutions: Some(Resolutions {
                single: vec![
                    Resolution {
                        label: "14x15".to_string(),
                        width: "14".to_string(),
                        height: "15".to_string(),
                        image: "/single/14x15/valley_single_14x15.jpg".to_string(),
                    },
                    Resolution {
                        label: "16x17".to_string(),
                        width: "16".to_string(),
                        height: "17".to_string(),
                        image: "/single/16x17/valley_single_16x17.jpg".to_string(),
                    },
                ],
                dual: None,
                triple: None,
                mobile: None,
            }),
            sku: None,
            tags: None,
            timestamp: None,
        };
        let expected_get_wallpapers_response = GetWallpapersResponse {
            db_core: GetWallpapersDBCore {
                timestamp: 1,
                endpoints: Endpoints {
                    api: "https://api.digitalblasphemy.com/v2/core".to_string(),
                    image: "https://arcadia.digitalblasphemy.com".to_string(),
                    thumb: "https://cdn.digitalblasphemy.com".to_string(),
                    web: "https://digitalblasphemy.com".to_string(),
                },
                request: GetWallpapersDBCoreRequest {
                    query: GetWallpapersResponseQuery {
                        filter_date_day: None,
                        filter_date_month: None,
                        filter_date_year: None,
                        filter_date_operator: Operator::Equal,
                        filter_gallery: None,
                        filter_rating: None,
                        filter_rating_operator: None,
                        filter_res_operator_height: None,
                        filter_res_operator_width: None,
                        filter_res_height: 2,
                        filter_res_operator: Operator::GreaterThanOrEqual,
                        filter_res_width: 3,
                        filter_tag: None,
                        limit: 4,
                        order: Order::Ascending,
                        order_by: GetWallpapersOrderBy::Name,
                        page: 5,
                        s: None,
                        show_comments: false,
                        show_pickle_jar: false,
                        show_resolutions: false,
                    },
                },
                total_pages: 6,
                wallpapers: HashMap::from([
                    ("7".to_string(), expected_wallpaper_7),
                    ("13".to_string(), expected_wallpaper_13),
                ]),
            },
        };
        assert_eq!(get_wallpapers_response, expected_get_wallpapers_response);

        mock.assert_async().await;

        Ok(())
    }
}
