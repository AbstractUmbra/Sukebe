use eos::{DateTime, Utc, format_dt, serde::timestamp};
use serde::Deserialize;

#[derive(Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct CDNResponse {
    pub thumb_servers: Vec<String>,
    pub image_servers: Vec<String>,
}

/// An image representation.
#[derive(Deserialize)]
pub struct Image {
    /// 'width' is the image width.
    pub width: u16,
    /// 'height' is the image height.
    pub height: u16,
    /// 'path' is the CDN image path.
    pub path: String,
}

impl Image {
    pub fn media_url(&self, cdn_url: &str) -> String {
        format!("{}/galleries/{}", cdn_url, self.path)
    }
}

/// nHentai tags
#[derive(Deserialize)]
pub struct Tag {
    id: u32,
    r#type: String,
    name: String,
    slug: String,
    url: String,
    count: u32,
}

/// Title is two of three formats, English OR Japanese AND pretty.
#[derive(Deserialize)]
pub struct Title {
    pub english: Option<String>,
    pub japanese: Option<String>,
    pub pretty: String,
}

#[derive(Deserialize, Eq, PartialEq)]
pub struct Page {
    pub number: u32,
    pub path: String,
    pub width: u32,
    pub height: u32,
    #[serde(rename = "thumbnail")]
    pub thumbnail_path: String,
    pub thumbnail_width: u32,
    pub thumbnail_height: u32,
}

impl Page {
    pub fn thumbnail_url(&self, thumbnail_cdn_url: &str) -> String {
        format!("{}/{}", thumbnail_cdn_url, self.thumbnail_path)
    }
    pub fn image_url(&self, image_cdn_url: &str) -> String {
        format!("{}/{}", image_cdn_url, self.path)
    }
}

/// The full API response per Gallery.
#[derive(Deserialize)]
pub struct Doujin {
    pub id: u32,
    pub media_id: String,
    pub title: Title,
    pub cover: Image,
    pub thumbnail: Image,
    pub scanlator: String,
    #[serde(with = "timestamp")]
    pub upload_date: DateTime<Utc>,
    pub tags: Vec<Tag>,
    pub num_favorites: u32,
    pub num_pages: u32,
    pub pages: Vec<Page>,
}

impl Doujin {
    pub fn pretty_date(&self) -> String {
        format_dt!("%d-%m-%Y %H:%M:%S", self.upload_date).to_string()
    }
}
