use axum::{Router, routing::get};
use grimoire::{
    config::{Settings, db},
    routes::folder_routes,
};

#[tokio::main]
async fn main() {
    let setting = Settings::default();
    let pool = db::init_pool(&setting.database_url);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/folders", folder_routes::router(pool));

    let listener = tokio::net::TcpListener::bind(setting.server_addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
