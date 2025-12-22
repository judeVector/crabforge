use error_chain::error_chain;
use reqwest;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
    HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    let _ = res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Header: {:?}", res.headers());
    println!("Body: {}", body);

    Ok(())
}
