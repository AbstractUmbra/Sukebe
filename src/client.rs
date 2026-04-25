use crate::models::{CDNResponse, Doujin};
use anyhow::{Context, Result};
use futures::stream::TryStreamExt;
use moka::future::Cache;
use rand::seq::IndexedRandom;
use reqwest::Client;
use std::sync::Arc;
use std::{fs::File, io::Write};

pub(crate) const API_BASE: &'static str = "https://nhentai.net/api/v2";
pub(crate) const USER_AGENT: &'static str = "Sukebe/v1 (https://github.com/AbstractUmbra/Sukebe)";

pub struct SukebeClient {
    client: Arc<Client>,
    image_cdn_cache: Cache<u8, Arc<CDNResponse>>,
    auth_key: String,
}

impl SukebeClient {
    pub fn new() -> Self {
        let contents = std::fs::read_to_string("auth.key");
        let auth = match contents {
            Ok(key) => key,
            Err(_) => "".into(),
        };

        Self {
            client: Arc::new(Client::new()),
            image_cdn_cache: Cache::builder()
                .time_to_live(std::time::Duration::from_hours(1))
                .build(),
            auth_key: auth,
        }
    }

    pub async fn get_doujin(&self, doujin_id: u32) -> Result<Doujin> {
        let url = format!("{}/galleries/{}", API_BASE, doujin_id);
        let mut req = self.client.get(&url).header("User-Agent", USER_AGENT);
        if !self.auth_key.is_empty() {
            req = req.header("Authorization", &self.auth_key);
        };
        let result = req
            .send()
            .await
            .with_context(|| format!("Could not fetch URL `{}`", &url))?
            .json::<Doujin>()
            .await
            .context("Invalid API response")?;

        Ok(result)
    }

    pub async fn get_cdn_data(&self) -> Result<Arc<CDNResponse>> {
        let cdn_config = self
            .client
            .get(format!("{}/cdn", API_BASE))
            .header("User-Agent", USER_AGENT)
            .send()
            .await
            .with_context(|| "Could not fetch cdn config")?
            .json::<CDNResponse>()
            .await
            .context("Invalid API response")?;

        let arced = Arc::new(cdn_config);
        self.image_cdn_cache.insert(0, arced.clone()).await;

        Ok(arced)
    }

    pub async fn get_page(&self, doujin: &Doujin) -> Result<()> {
        let cdn = { self.image_cdn_cache.get(&0).await };

        let cdn = match cdn {
            Some(cdn) => cdn,
            None => self.get_cdn_data().await?,
        };

        let cdn_url = match cdn.image_servers.choose(&mut rand::rng()) {
            Some(url) => url,
            None => panic!("Unable to rng choose an image cdn."),
        };

        for page in doujin.pages.iter() {
            let url = format!("{}/{}", cdn_url, page.path);

            let resp = self
                .client
                .get(&url)
                .header("User-Agent", USER_AGENT)
                .send()
                .await
                .with_context(|| format!("Unable to download page from url: {}", &url))?;

            let path = std::path::Path::new(&page.path);

            let stem = path.file_stem().unwrap().to_str().unwrap(); // "1"
            let ext = path.extension().unwrap().to_str().unwrap(); // "jpg"

            let num: u32 = stem.parse().unwrap();
            let padded = format!("{:03}", num);

            let resolved_filename = format!("{}.{}", padded, ext);

            let file_path = format!("{}/{}", doujin.id, resolved_filename);
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
}
