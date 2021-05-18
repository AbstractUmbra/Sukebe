use chrono::{
    format::{DelayedFormat, StrftimeItems},
    prelude::NaiveDateTime,
    DateTime, Utc,
};
use serde::Deserialize;


/// An image representation.
#[derive(Deserialize, Debug)]
pub struct Image {
    /// 'h' is image height.
    pub h: u16,
    /// 't' is image type.
    pub t: String,
    /// 'w' is image width.
    pub w: u16,
}

impl Image {
    pub fn image_type(&self) -> &'static str {
        match self.t.as_str() {
            "j" => "jpg",
            "p" => "png",
            "g" => "gif",
            _ => "?"
        }
    }
}

/// Each gallery returns an array of Image.
#[derive(Deserialize, Debug)]
pub struct Images {
    pub cover: Image,
    pub pages: Vec<Image>,
    pub thumbnail: Image,
}

/// nHentai tags
#[derive(Deserialize, Debug)]
pub struct Tag {
    count: u32,
    id: u32,
    name: String,
    r#type: String,
    url: String,
}

/// Title is two of three formats, English OR Japanese AND pretty.
#[derive(Deserialize, Debug)]
pub struct Title {
    pub english: String,
    pub japanese: String,
    pub pretty: String,
}

/// The full API response per Gallery.
#[derive(Deserialize, Debug)]
pub struct Response {
    pub id: u32,
    pub images: Images,
    pub media_id: String,
    pub num_favorites: u32,
    pub num_pages: u32,
    pub scanlator: String,
    pub tags: Vec<Tag>,
    pub title: Title,
    pub upload_date: u32,
}

impl Response {
    pub fn uploaded_date_str(&self) -> DelayedFormat<StrftimeItems> {
        let naive = NaiveDateTime::from_timestamp(self.upload_date.into(), 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        datetime.format("%d-%m-%Y %H:%M:%S")
    }
}
