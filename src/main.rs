use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ResultPayload {
    fact: String,
    length: u64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://catfact.ninja/fact")
        .header("User-Agent", "MyApp")
        .send()
        .await?;
    let payload = res.json::<ResultPayload>().await?;
    println!("{:?}", payload);
    Ok(())
}
