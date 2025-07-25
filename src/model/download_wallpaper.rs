use core::panic;

use crate::{model::WallpaperType, model::common_response::Endpoints};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadWallpaperResponse {
    pub db_core: DownloadWallpaperDBCore,
    pub download: Download,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadWallpaperDBCore {
    pub timestamp: i64,
    pub endpoints: Endpoints,
    pub request: DownloadWallpaperDBCoreRequest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadWallpaperDBCoreRequest {
    pub params: DownloadWallpaperResponseParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadWallpaperResponseParams {
    #[serde(rename = "type")]
    pub wallpaper_type: WallpaperType,
    pub width: u16,
    pub height: u16,
    pub wallpaper_id: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Download {
    pub expiration: i64,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadWallpaperRequest {
    #[serde(rename = "type")]
    pub wallpaper_type: WallpaperType,
    pub width: u16,
    pub height: u16,
    pub wallpaper_id: u16,
    pub show_watermark: bool,
}

impl DownloadWallpaperRequest {
    pub fn builder() -> DownloadWallpaperRequestBuilder {
        DownloadWallpaperRequestBuilder::new()
    }
}

pub struct DownloadWallpaperRequestBuilder {
    download_wallpaper_request: DownloadWallpaperRequest,
}

impl DownloadWallpaperRequestBuilder {
    pub(crate) fn new() -> Self {
        DownloadWallpaperRequestBuilder {
            download_wallpaper_request: DownloadWallpaperRequest {
                wallpaper_type: WallpaperType::Single,
                width: 0,
                height: 0,
                wallpaper_id: 0,
                show_watermark: true,
            },
        }
    }

    pub fn build(self) -> DownloadWallpaperRequest {
        if self.download_wallpaper_request.width == 0 {
            panic!("Width must be provided.")
        }
        if self.download_wallpaper_request.height == 0 {
            panic!("Height must be provided.")
        }
        if self.download_wallpaper_request.wallpaper_id == 0 {
            panic!("Wallpaper ID must be provided.")
        }
        self.download_wallpaper_request
    }

    pub fn wallpaper_type(mut self, wallpaper_type: WallpaperType) -> Self {
        self.download_wallpaper_request.wallpaper_type = wallpaper_type;
        self
    }

    pub fn width(mut self, width: u16) -> Self {
        self.download_wallpaper_request.width = width;
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.download_wallpaper_request.height = height;
        self
    }

    pub fn wallpaper_id(mut self, wallpaper_id: u16) -> Self {
        self.download_wallpaper_request.wallpaper_id = wallpaper_id;
        self
    }

    pub fn show_watermark(mut self, show_watermark: bool) -> Self {
        self.download_wallpaper_request.show_watermark = show_watermark;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_wallpaper_request_builder_test {
        use super::*;

        #[test]
        #[should_panic(expected = "Width must be provided.")]
        fn test_download_wallpaper_request_query_builder_rejects_build_when_width_not_provided() {
            DownloadWallpaperRequest::builder()
                .height(2)
                .wallpaper_id(3)
                .build();
        }

        #[test]
        #[should_panic(expected = "Height must be provided.")]
        fn test_download_wallpaper_request_query_builder_rejects_build_when_height_not_provided() {
            DownloadWallpaperRequest::builder()
                .width(1)
                .wallpaper_id(3)
                .build();
        }

        #[test]
        #[should_panic(expected = "Wallpaper ID must be provided.")]
        fn test_download_wallpaper_request_query_builder_rejects_build_when_wallpaper_id_not_provided()
         {
            DownloadWallpaperRequest::builder()
                .width(1)
                .height(2)
                .build();
        }

        #[test]
        fn test_download_wallpaper_request_builder_sets_defaults() {
            let download_wallpaper_request = DownloadWallpaperRequest::builder()
                .width(1)
                .height(2)
                .wallpaper_id(3)
                .build();

            assert_eq!(
                download_wallpaper_request.wallpaper_type,
                WallpaperType::Single
            );
            assert_eq!(download_wallpaper_request.width, 1);
            assert_eq!(download_wallpaper_request.height, 2);
            assert_eq!(download_wallpaper_request.wallpaper_id, 3);
            assert!(download_wallpaper_request.show_watermark);
        }

        #[test]
        fn test_download_wallpaper_request_builder_overrides_all_defaults() {
            let download_wallpaper_request = DownloadWallpaperRequest::builder()
                .wallpaper_type(WallpaperType::Dual)
                .width(4)
                .height(5)
                .wallpaper_id(6)
                .show_watermark(false)
                .build();

            assert_eq!(
                download_wallpaper_request.wallpaper_type,
                WallpaperType::Dual
            );
            assert_eq!(download_wallpaper_request.width, 4);
            assert_eq!(download_wallpaper_request.height, 5);
            assert_eq!(download_wallpaper_request.wallpaper_id, 6);
            assert!(!download_wallpaper_request.show_watermark);
        }
    }
}
