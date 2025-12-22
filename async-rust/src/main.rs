use anyhow::Result;
use reqwest;

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?;

    println!("Status: {}", res.status());
    println!("Headers: {:?}", res.headers());
    let body = res.text().await?;

    println!("Body: {}", body);

    Ok(())
}
