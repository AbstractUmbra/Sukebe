use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

mod cli;

use cli::Args;
use sukebe::SukebeClient;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut client = SukebeClient::new();

    if let Some(key) = std::fs::read_to_string("auth.key").ok() {
        client = client.with_api_key(key.trim());
    }

    if args.digits.is_some() {
        match args.digits {
            Some(galleries) => download_many(&client, galleries).await?,
            None => println!("No digits specified."),
        }
    }

    Ok(())
}

async fn download_single(client: &SukebeClient, digits: u32) -> Result<()> {
    let doujin = &client.get_doujin(digits).await?;
    let directory_path = PathBuf::from(doujin.id.to_string());

    if !directory_path.exists() {
        fs::create_dir(&directory_path)
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
