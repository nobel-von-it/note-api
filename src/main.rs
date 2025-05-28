use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use note_api::handlers;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/api/ls", get(handlers::api_ls));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
