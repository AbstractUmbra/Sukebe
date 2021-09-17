use anyhow::{Context, Result};
use models::{Cli, Doujin};
use std::{fs, path::PathBuf};

mod models;

#[tokio::main]
async fn main() -> Result<()> {
    let args = <Cli as structopt::StructOpt>::from_args();

    if !args.digits.is_none() {
        download_single(args.digits.unwrap()).await?
    }
    if !args.search.is_none() {
        download_from_search(args.search.unwrap()).await?
    }
    if !args.alike.is_none() {
        download_from_alike(args.alike.unwrap()).await?
    }

    Ok(())
}

async fn download_single(digits: u32) -> Result<()> {
    let doujin = Doujin::new(digits).await?;
    let directory_path = PathBuf::from(doujin.id.to_string());

    if !directory_path.exists() {
        fs::create_dir(&directory_path)
            .with_context(|| format!("Could not create directory named `{}`", doujin.id))?;
    }

    doujin.gallery().await?;

    Ok(())
}

async fn download_from_search(search: String) -> Result<()> {
    let doujins = Doujin::search(&search).await?;
    for doujin in doujins {
        let directory_path = PathBuf::from(doujin.id.to_string());

        if !directory_path.exists() {
            fs::create_dir(&directory_path)
                .with_context(|| format!("Could not create directory named `{}`", doujin.id))?;
        }

        doujin.gallery().await?;
    }

    Ok(())
}

async fn download_from_alike(digits: u32) -> Result<()> {
    let doujins = Doujin::from_alike(digits).await?;
    for doujin in doujins {
        let directory_path = PathBuf::from(doujin.id.to_string());

        if !directory_path.exists() {
            fs::create_dir(&directory_path)
                .with_context(|| format!("Could not created directory named `{}`", doujin.id))?;
        }

        doujin.gallery().await?;
    }

    Ok(())
}
