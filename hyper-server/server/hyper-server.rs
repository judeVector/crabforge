use http_body_util::Full;
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::http::Error;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

async fn hello(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Error> {
    match (req.method(), req.uri().path()) {
        //GET
        (&Method::GET, "/") => {
            let body = Bytes::from("Hello from pure Hyper");
            Ok(Response::new(Full::new(body)))
        }

        // GET /health
        (&Method::GET, "/health") => {
            let response = ApiResponse {
                message: "Ok".to_string(),
            };

            let json = serde_json::to_string(&response).unwrap();
            Ok(Response::new(Full::new(Bytes::from(json))))
        }

        _ => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from("Not Found"))),
    }

    // Ok(Response::new(Full::new(Bytes::from("Hello, World"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
            {
                eprintln!("Error serving connection: {}", err);
            }
        });
    }
}
