use std::fs;

use anyhow::{Context, Result, bail};
use clap::Parser;

use sukebe::SukebeClient;
use sukebe::cli::CliArgs;
use sukebe::cli::SearchSort;

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    let mut client = SukebeClient::new();

    if let Ok(key) = std::fs::read_to_string("auth.key") {
        client = client.with_api_key(key.trim());
    }

    if args.digits.is_some() {
        match args.digits {
            Some(galleries) => download_many(&client, galleries).await?,
            None => println!("No digits specified."),
        }
    } else if args.search.tags.is_some() {
        match args.search.tags {
            Some(tags) => download_tags(&client, tags, args.search.sort).await?,
            None => bail!("No search tags provided."),
        }
    }

    Ok(())
}

async fn download_single(client: &SukebeClient, digits: u32) -> Result<()> {
    let doujin = &client.get_doujin(digits).await?;
    let directory_path = doujin.download_path();

    if !directory_path.exists() {
        fs::create_dir_all(&directory_path)
            .with_context(|| format!("Could not create directory named `{}`", doujin.id))?;
    }

    client.get_page(doujin).await?;
    Ok(())
}

async fn download_many(client: &SukebeClient, digits: Vec<u32>) -> Result<()> {
    for gallery in digits {
        download_single(client, gallery).await?;
    }

    Ok(())
}

async fn download_tags(client: &SukebeClient, tags: Vec<String>, sort: SearchSort) -> Result<()> {
    let search_results = client.search_tags(tags, sort).await?;

    if search_results.is_empty() {
        bail!("Unable to find any doujin with the provided search tags.")
    }

    let ids = search_results.into_iter().map(|item| item.id).collect();

    download_many(client, ids).await
}
