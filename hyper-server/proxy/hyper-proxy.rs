use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, body::Incoming};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("🚀 Reverse proxy running on http://127.0.0.1:3000");
    println!("   Forwarding all requests to httpbin.org\n");

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            if let Err(e) = http1::Builder::new()
                .serve_connection(io, service_fn(proxy_handler))
                .await
            {
                eprintln!("Error serving connection: {:?}", e);
            }
        });
    }
}

async fn proxy_handler(
    req: Request<Incoming>,
) -> Result<Response<Empty<Bytes>>, Box<dyn std::error::Error + Send + Sync>> {
    println!("📥 Received: {} {}", req.method(), req.uri());

    // Extract the original request details
    let method = req.method().clone();
    let path = req.uri().path();
    let query = req.uri().query().unwrap_or("");

    // Build NEW request to the backend server (httpbin.org)
    let backend_url = format!("http://httpbin.org{}?{}", path, query);
    println!("📤 Forwarding to: {}", backend_url);

    // Create HTTP client using Hyper
    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build_http();

    // Make the proxied request
    let backend_req = Request::builder()
        .method(method)
        .uri(backend_url)
        .body(Empty::<Bytes>::new())?;

    let backend_response = client.request(backend_req).await?;

    println!("✅ Backend responded: {}\n", backend_response.status());

    // Forward the backend's response back to the original client
    let (parts, body) = backend_response.into_parts();

    // Consume the body (in real proxy you'd stream it)
    let _ = body.collect().await?;

    Ok(Response::from_parts(parts, Empty::<Bytes>::new()))
}
