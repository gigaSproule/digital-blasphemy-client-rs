use crate::{model::common_response::Endpoints, model::WallpaperType};
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
    // TODO: Remove allow dead code when tests are implemented
    #[allow(dead_code)]
    pub fn show_watermark(mut self, show_watermark: bool) -> Self {
        self.download_wallpaper_request.show_watermark = show_watermark;
        self
    }
}
