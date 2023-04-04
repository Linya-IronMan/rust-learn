use std::collections::HashMap;

async fn fetch_ip() -> Result<HashMap<String, String>, reqwest::Error> {
    let resp = reqwest::get("https://httpbin.org/ip").await?.json().await?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let result = fetch_ip().await?;
    println!("{:#?}", result);
    Ok(())
}
