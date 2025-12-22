use anyhow::Error;
use reqwest;
use select::document::Document;
use select::predicate::Name;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let res = reqwest::get("https://www.rust-lang.org/")
        .await?
        .text()
        .await?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}
