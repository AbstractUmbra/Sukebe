use anyhow::{Context, Result};
use chrono::{
    format::{DelayedFormat, StrftimeItems},
    serde::ts_seconds,
    DateTime, Utc,
};
use futures::stream::TryStreamExt;
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::{fmt, fs::File, io::Write};
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

struct ToU32Visitor;

impl<'de> de::Visitor<'de> for ToU32Visitor {
    type Value = u32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a String or Integer")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        serde_json::from_str(v).map_err(E::custom)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v as u32)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v as u32)
    }
}

fn to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(ToU32Visitor)
}

/// The full API response per Gallery.
#[derive(Deserialize, Debug)]
pub struct Doujin {
    #[serde(deserialize_with = "to_u32")]
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

#[derive(Deserialize, Debug)]
pub struct DoujinSearch {
    pub result: Vec<Doujin>,
    pub num_pages: Value,
    pub per_page: Value,
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

    pub async fn search(query: &str) -> Result<Vec<Self>> {
        let url = "https://nhentai.net/api/galleries/search";
        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .query(&[("query", query)])
            .send()
            .await
            .with_context(|| format!("Could not perform search with query `{}", query))?
            .json::<DoujinSearch>()
            .await
            .with_context(|| format!("Could not parse response from url `{}`", &url))?;

        Ok(response.result)
    }

    pub async fn from_tag(tag_id: u32) -> Result<Vec<Self>> {
        let url = "https://nhentai.net/api/galleries/tagged";
        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .query(&[("tag_id", &tag_id)])
            .send()
            .await
            .with_context(|| format!("Could not search tags with id `{}", &tag_id))?
            .json::<DoujinSearch>()
            .await
            .with_context(|| format!("Could not parse response from url `{}`", &url))?;

        Ok(response.result)
    }

    pub async fn from_alike(doujin_id: u32) -> Result<Vec<Self>> {
        let url = format!("https://nhentai.net/api/gallery/{}/related", doujin_id);
        let client = reqwest::Client::new();

        let response = client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("Could not search related doujin with id `{}", &doujin_id))?
            .json::<DoujinSearch>()
            .await
            .with_context(|| format!("Could not parse response from url `{}`", &url))?;

        Ok(response.result)
    }

    pub async fn gallery(&self) -> Result<()> {
        println!(
            "name: {:#?}\nmedia: {:#?}\npages: {:#?}\nupload date: {}",
            self.title.pretty,
            self.media_id,
            self.num_pages,
            self.pretty_date()
        );

        for (i, image) in self.images.pages.iter().enumerate() {
            let page_number = i + 1;
            let url = image.media_url(self.media_id, page_number as u16);
            let resp = reqwest::get(&url)
                .await
                .with_context(|| format!("Could not fetch URL `{}`", &url))?;

            let file_path = format!("{}/{}.{}", self.id, page_number, image.format);
            let mut file = File::create(&file_path)
                .with_context(|| format!("Could not create file at `{}`", &file_path))?;

            let mut stream = resp.bytes_stream();
            while let Some(chunk) = stream.try_next().await? {
                file.write_all(&chunk)
                    .with_context(|| format!("Could not write to `{}`", &file_path))?;
            }
        }

        Ok(())
    }

    pub fn pretty_date(&self) -> DelayedFormat<StrftimeItems> {
        self.upload_date.format("%d-%m-%Y %H:%M:%S")
    }
}

#[derive(StructOpt)]
#[structopt(name = "Lewd", about = "Abandon all faith, ye who enter here.")]
pub struct Cli {
    /// Specify the term to search for.
    #[structopt(short, long)]
    pub search: Option<String>,
    /// Specify the digits to search for.
    #[structopt(short, long)]
    pub digits: Option<u32>,
}
