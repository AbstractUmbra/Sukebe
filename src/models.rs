use anyhow::{Context, Result};
use chrono::{
    format::{DelayedFormat, StrftimeItems},
    serde::ts_seconds,
    DateTime, Utc,
};
use serde::{de, Deserialize, Deserializer};
use std::fmt;
use structopt::StructOpt;

#[derive(Debug)]
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

impl fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.extension())
    }
}

impl<'de> Deserialize<'de> for ImageFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let file_type: &str = Deserialize::deserialize(deserializer)?;

        match file_type {
            "j" => Ok(ImageFormat::Jpg),
            "p" => Ok(ImageFormat::Png),
            "g" => Ok(ImageFormat::Gif),
            _ => Err(de::Error::custom("unrecognised file format")),
        }
    }
}

/// An image representation.
#[derive(Deserialize, Debug)]
pub struct Image {
    /// 'width' is the image width.
    #[serde(rename = "w")]
    pub width: u16,
    /// 'height' is the image height.
    #[serde(rename = "h")]
    pub height: u16,
    /// 'format' is the image format.
    #[serde(rename = "t")]
    pub format: ImageFormat,
}

impl Image {
    pub fn media_url(&self, media_id: u32, page_number: u16) -> String {
        format!(
            "https://i.nhentai.net/galleries/{}/{}.{}",
            media_id, page_number, self.format
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
    pub async fn new(gallery_id: u32) -> Result<Self> {
        let url = format!("https://nhentai.net/api/gallery/{}", gallery_id);
        let result = reqwest::get(&url)
            .await
            .with_context(|| format!("Could not fetch URL `{}`", &url))?
            .json::<Self>()
            .await
            .context("Invalid API response")?;

        Ok(result)
    }

    pub fn pretty_date(&self) -> DelayedFormat<StrftimeItems> {
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
