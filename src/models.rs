use chrono::{
    format::{DelayedFormat, StrftimeItems},
    serde::ts_seconds,
    DateTime, Utc,
};
use serde::{de, Deserialize, Deserializer};
use structopt::StructOpt;

pub enum ImageFormat {
    Jpg,
    Png,
    Gif,
}

impl ImageFormat {
    pub fn extension(&self) -> &str {
        match self {
            ImageFormat::Gif => "gif",
            ImageFormat::Jpg => "jpg",
            ImageFormat::Png => "png",
        }
    }
}

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
    pub fn image_type(&self) -> ImageFormat {
        match self.t.as_str() {
            "j" => ImageFormat::Jpg,
            "p" => ImageFormat::Png,
            "g" => ImageFormat::Gif,
            _ => unreachable!(),
        }
    }

    pub fn media_url(&self, media_id: &u32, page_number: u16) -> String {
        format!(
            "https://i.nhentai.net/galleries/{}/{}.{}",
            media_id,
            page_number,
            self.image_type().extension()
        )
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
    #[serde(rename = "type")]
    category: String,
    url: String,
}

/// Title is two of three formats, English OR Japanese AND pretty.
#[derive(Deserialize, Debug)]
pub struct Title {
    pub english: Option<String>,
    pub japanese: Option<String>,
    pub pretty: String,
}

fn to_u32<'de, D>(value: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(value)?;
    s.parse().map_err(de::Error::custom)
}

/// The full API response per Gallery.
#[derive(Deserialize, Debug)]
pub struct Doujin {
    pub id: u32,
    pub images: Images,
    #[serde(deserialize_with = "to_u32")]
    pub media_id: u32,
    pub num_favorites: u32,
    pub num_pages: u32,
    pub scanlator: String,
    pub tags: Vec<Tag>,
    pub title: Title,
    #[serde(with = "ts_seconds")]
    pub upload_date: DateTime<Utc>,
}

impl Doujin {
    pub fn uploaded_date_str(&self) -> DelayedFormat<StrftimeItems> {
        self.upload_date.format("%d-%m-%Y %H:%M:%S")
    }
}

#[derive(StructOpt)]
#[structopt(name = "Lewd", about = "Abandon all faith, ye who enter here.")]
pub struct Cli {
    /// Specify the digits to search for.
    #[structopt(short, long)]
    pub digits: u32,
}
