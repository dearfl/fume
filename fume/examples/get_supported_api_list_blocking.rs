use std::time::Duration;

use fume::{SteamApiKey, Unauthorized};

fn main() -> anyhow::Result<()> {
    let client = reqwest::blocking::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()?;

    let apilist = match std::env::args().nth(1) {
        Some(key) => {
            let api = SteamApiKey::new(key);
            let steam = api.with_client(client);
            steam.apis()?
        }
        None => {
            let steam = Unauthorized::with_client(client);
            steam.apis()?
        }
    };
    println!("{:#?}", apilist);

    Ok(())
}
