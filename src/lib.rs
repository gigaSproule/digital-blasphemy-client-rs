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
}

impl DigitalBlasphemyClient {
    pub fn new(api_key: String) -> Result<DigitalBlasphemyClient, Box<dyn Error>> {
        Ok(DigitalBlasphemyClient {
            authorization: format!("Bearer {api_key}"),
            client: reqwest::Client::builder().build()?,
        })
    }

    pub async fn get_user_information(
        &self,
    ) -> Result<GetAccountInformationResponse, ErrorResponse> {
        let get_account_information_response = self
            .get_request_json::<GetAccountInformationResponse>(
                &vec![],
                "https://api.digitalblasphemy.com/v2/core/account".to_string(),
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
                "https://api.digitalblasphemy.com/v2/core/wallpapers".to_string(),
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

    // TODO: Remove allow dead code when tests are implemented
    #[allow(dead_code)]
    pub async fn get_wallpaper(
        &self,
        request: &GetWallpaperRequest,
    ) -> Result<Wallpaper, ErrorResponse> {
        let get_wallpaper_response = self
            .get_request_json::<GetWallpaperResponse>(
                &Self::get_wallpaper_query(request),
                format!(
                    "https://api.digitalblasphemy.com/v2/core/wallpaper/{}",
                    request.wallpaper_id
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
                    "https://api.digitalblasphemy.com/v2/core/download/wallpaper/{}/{}/{}/{}",
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

        let download_request = self
            .client
            .request(reqwest::Method::GET, url)
            .query(&query)
            .headers(headers.clone());

        Self::debug_log(&download_request);

        let response = download_request.send().await;

        if response.is_err() {
            let error = response.unwrap_err();
            let status = error.status().unwrap();
            return Err(ErrorResponse {
                code: status.as_u16() as u64,
                description: status.canonical_reason().unwrap().to_string(),
                errors: vec![error.source().unwrap().to_string()],
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
                    errors: vec![
                        unwrapped_response
                            .text()
                            .await
                            .expect("Unable to parse the body as text"),
                    ],
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
