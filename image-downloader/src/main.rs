use anyhow::Error;
use reqwest;
use std::env;
use std::fs::File;
use std::io::{Cursor, copy};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let current_dir = env::current_dir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("File to download: {}", fname);

        let fname = current_dir.join(fname);
        println!("Saving to: {:?}", fname);

        File::create(&fname)?
    };

    let bytes = response.bytes().await?;
    let mut content = Cursor::new(bytes);

    copy(&mut content, &mut dest)?;
    Ok(())
}
