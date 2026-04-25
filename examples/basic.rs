use anyhow::Result;
use tokio;

use sukebe::SukebeClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = SukebeClient::new();

    // or if you have an API key
    // let client = SukebeClient::new()
    //                 .with_api_key("token");

    let doujin = client.get_doujin(177013).await?;
    println!("{:?}", doujin);

    Ok(())
}
