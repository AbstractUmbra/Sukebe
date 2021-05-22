use anyhow::{Context, Result};
use futures::stream::TryStreamExt;
use models::{Cli, Doujin};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

mod models;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::from_args();
    let doujin = Doujin::new(args.digits).await?;
    // let doujins = Doujin::search("nagatoro").await?;
    // for doujin in doujins {
    //     let directory_path = PathBuf::from(doujin.id.to_string());

    //     if !directory_path.exists() {
    //         fs::create_dir(&directory_path)
    //             .with_context(|| format!("Could not create directory named `{}`", doujin.id))?;
    //     }

    //     doujin.gallery().await?;
    // }
    let directory_path = PathBuf::from(doujin.id.to_string());

    if !directory_path.exists() {
        fs::create_dir(&directory_path)
            .with_context(|| format!("Could not create directory named `{}`", doujin.id))?;
    }

    doujin.gallery().await?;
    Ok(())
}

impl Doujin {
    async fn gallery(&self) -> Result<()> {
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
}
