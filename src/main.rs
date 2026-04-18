use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum::{
    Router,
    http::{Request, Response},
    routing::get,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let host = std::env::var("APP_HOST").expect("APP_HOST must be set");
    let port = std::env::var("APP_PORT")
        .expect("APP_PORT must be set")
        .parse::<u16>()
        .unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(
            TraceLayer::new_for_http()
                .on_request(|req: &Request<_>, _span: &tracing::Span| {
                    let method = req.method();
                    let uri = req.uri();
                    let agent = req
                        .headers()
                        .get("user-agent")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("unknown");

                    tracing::info!("--> {} {} (agent: {})", method, uri, agent);
                })
                .on_response(
                    |res: &Response<_>, latency: Duration, _span: &tracing::Span| {
                        let status = res.status();
                        let ms = latency.as_millis();

                        tracing::info!("<-- {} ({}ms)", status, ms);
                    },
                )
                .on_failure(|error, latency: Duration, _span: &tracing::Span| {
                    tracing::error!("ERR {:?} after {}ms", error, latency.as_millis());
                }),
        );

    let listener = tokio::net::TcpListener::bind((host, port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
