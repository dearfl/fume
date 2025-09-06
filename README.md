# fume

[![crates.io](https://img.shields.io/crates/v/fume)](https://crates.io/crates/fume)
[![docs.rs](https://img.shields.io/docsrs/fume)](https://docs.rs/fume)

A strongly-opinionated Rust wrapper for steam web APIs.

## Example

get list of apis that don't require api key.

```rust,no_run
use std::time::Duration;

use fume::{Auth, Unauthorize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()?;

    let steam = Unauthorize.with_client(client);
    let apilist = steam.apis().await?;
    println!("{:#?}", apilist);

    Ok(())
}
```

## Limitations

- Steam Web API does't have enough official documents, so this crate probably won't ever be stable or even usable.

## License

This project is licensed under the GLWTPL (Good Luck With That Public License). See the `LICENSE` file for more details.
