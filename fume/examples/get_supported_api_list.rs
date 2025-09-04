use std::time::Duration;

use fume::{SteamApiKey, Unauthorize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()?;

    let apilist = match std::env::args().nth(1) {
        Some(key) => {
            let api = SteamApiKey::new(key);
            let steam = api.with_client(client);
            steam.apis().await?
        }
        None => {
            let steam = Unauthorize::with_client(client);
            steam.apis().await?
        }
    };
    println!("{:#?}", apilist);

    Ok(())
}
