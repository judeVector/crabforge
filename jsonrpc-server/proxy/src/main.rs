use anyhow::Result;
use axum::{
    Router,
    body::Body,
    extract::{Path, State},
    http::{Request, StatusCode, Uri, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
};
use http_body_util::BodyExt;
use reqwest::Client;
use std::sync::Arc;
use tracing_subscriber;

#[derive(Clone)]
struct AppState {
    valid_token: String,
    backend_url: String,
    client: Client,
}

async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if token != state.valid_token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

async fn proxy_handler(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let path_query = path
        .strip_prefix(&format!("/{}/", state.valid_token))
        .unwrap_or(path)
        .to_string();

    let path_query = if path_query.is_empty() {
        "/".to_string()
    } else {
        path_query.to_string()
    };

    println!("path: {}", path);
    println!("path_query: {}", path_query);

    let uri: Uri = format!("{}{}", state.backend_url, path_query)
        .parse()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    println!("url: {}", uri);

    let (parts, body) = req.into_parts();

    let body_bytes = body
        .collect()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .to_bytes();

    println!("body_bytes: {:?}", body_bytes);

    let mut backend_req = state.client.request(parts.method, uri.to_string());

    for (key, value) in parts.headers.into_iter() {
        if let Some(key) = key {
            if key != header::HOST && key != header::CONNECTION && key != header::TRANSFER_ENCODING
            {
                backend_req = backend_req.header(key, value);
            }
        }
    }

    if !body_bytes.is_empty() {
        backend_req = backend_req.body(body_bytes)
    }

    let backend_req = backend_req
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let backend_res = state
        .client
        .execute(backend_req)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let mut res_builder = Response::builder().status(backend_res.status());

    for (key, value) in backend_res.headers() {
        res_builder = res_builder.header(key, value)
    }

    let backend_body_bytes = backend_res
        .bytes()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let res = res_builder
        .body(Body::from(backend_body_bytes))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(res.into_response())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let state = Arc::new(AppState {
        valid_token: "abc123-secret-token".to_string(),
        backend_url: "http://127.0.0.1:9944".to_string(),
        client: Client::new(),
    });

    let app = Router::new()
        .route("/{token}", post(proxy_handler))
        .route("/{token}/{*path}", post(proxy_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Proxy server listening on http://127.0.0.1:3000");
    println!("Example usage:");
    println!(
        r#"curl -X POST http://127.0.0.1:3000/abc123-secret-token/ \
      -H "Content-Type: application/json" \
      -d '{{"jsonrpc":"2.0","method":"play_add","params":[5,8],"id":1}}'"#
    );

    axum::serve(listener, app).await?;

    Ok(())
}
