use std::fs;
use std::fs::File;
use std::io;
use std::{error::Error, io::Write};
mod models;
use models::Response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut guess = String::new();

    println!("Please input the forbidden numbers:");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read in the 6 digies.");

    let digits: u32 = guess.trim().parse().expect("Failed to read the digies.");
    let data = get_gallery_info(digits).await?;

    fs::create_dir(format!("{}", data.id))?;

    gallery(data).await?;
    // let date = &data.uploaded_date_str();

    Ok(())
}

async fn get_gallery_info(gallery_id: u32) -> Result<Response, Box<dyn Error>> {
    let url = format!("https://nhentai.net/api/gallery/{}", gallery_id);
    let resp = reqwest::get(url).await?.json::<Response>().await?;

    return Ok(resp);
}

fn generate_image_urls(response: &Response) -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();

    for (i, img) in response.images.pages.iter().enumerate() {
        urls.push(format!(
            "https://i.nhentai.net/galleries/{}/{}.{}",
            response.media_id,
            i + 1,
            img.image_type()
        ))
    }

    return urls;
}

async fn gallery(gallery_response: Response) -> Result<(), Box<dyn Error>> {
    println!("media: {:#?}", gallery_response.media_id);
    println!("pages: {:#?}", gallery_response.num_pages);

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
