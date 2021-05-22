use std::fs::{self, File};
use std::{error::Error, io::Write};
use structopt::StructOpt;
mod models;
use models::{Cli, Doujin};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let data = get_gallery_info(args.digits).await?;

    fs::create_dir(format!("{}", data.id))?;

    gallery(data).await?;
    // let date = &data.uploaded_date_str();

    Ok(())
}

async fn get_gallery_info(gallery_id: u32) -> Result<Doujin, Box<dyn Error>> {
    let url = format!("https://nhentai.net/api/gallery/{}", gallery_id);
    let resp = reqwest::get(url).await?.json::<Doujin>().await?;

    return Ok(resp);
}

fn generate_image_urls(response: &Doujin) -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();

    for (i, img) in response.images.pages.iter().enumerate() {
        urls.push(img.media_url(&response.media_id, (i as u16) + 1))
    }

    return urls;
}

async fn gallery(gallery_response: Doujin) -> Result<(), Box<dyn Error>> {
    println!(
        "name: {:#?}\nmedia: {:#?}\npages: {:#?}\nupload date: {}",
        gallery_response.title.pretty,
        gallery_response.media_id,
        gallery_response.num_pages,
        gallery_response.uploaded_date_str()
    );

    let urls = generate_image_urls(&gallery_response);

    for (i, url) in urls.iter().enumerate() {
        let resp = reqwest::get(url).await?;
        let data = resp.bytes().await?;
        let mut file = File::create(format!(
            "{}/{}.{}",
            gallery_response.id,
            (i + 1).to_string(),
            url[url.len() - 3..].to_string()
        ))?;
        file.write_all(&data)?;
    }

    Ok(())
}
