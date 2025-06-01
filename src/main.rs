use axum::Router;
use axum::routing::get;
use note_api::handlers;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/api/ls", get(handlers::api_ls))
        .route("/api/cd", get(handlers::api_cd))
        .route("/api/open", get(handlers::api_get_file))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(tower_http::compression::CompressionLayer::new().br(true).gzip(true).deflate(true));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
