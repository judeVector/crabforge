use http_body_util::Empty;
use hyper::Request;
use hyper::Uri;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = "http://localhost:3000".parse::<Uri>()?;
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let address = format!("{}:{}", host, port);

    let stream = TcpStream::connect(address).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    // Spawn the connection handler
    tokio::spawn(async move {
        if let Err(err) = conn.await {
            eprintln!("Connection failed: {}", err);
        }
    });

    let authority = url.authority().unwrap().clone();
    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    let res = sender.send_request(req).await?;
    println!("Response status: {}", res.status());

    // Read the response body
    use http_body_util::BodyExt;
    let body_bytes = res.collect().await?.to_bytes();
    let body_str = String::from_utf8_lossy(&body_bytes);
    println!("Response body: {}", body_str);

    Ok(())
}
