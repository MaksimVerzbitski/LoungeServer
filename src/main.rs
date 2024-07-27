mod db;
mod handlers;
mod models;

use axum::{
    routing::{get, post, put, delete, get_service},
    Router,
    extract::Extension,
};
use std::net::SocketAddr;
use tower_http::services::{ServeFile, ServeDir};
use handlers::user::{create_user, get_user, update_user, delete_user, get_all_users, json_hello, root};
use db::create_pool;
use std::io;
use tracing_subscriber;
use deadpool_postgres::Pool; // Add this import

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let pool = create_pool().await;

    let app = Router::new()
        .route("/", get(root))
        .route("/user", get_service(ServeFile::new("static/user.html")).handle_error(handle_file_error))
        .route("/user", post(create_user))
        .route("/user/:id", get(get_user))
        .route("/user/:id", put(update_user))
        .route("/user/:id", delete(delete_user))
        .route("/users", get(get_all_users))
        .route("/hello/:name", get(json_hello))
        .route("/health", get(health_check))
        .nest("/static", get_service(ServeDir::new("static")).handle_error(handle_file_error))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    println!("Server running at: http://127.0.0.1:3000");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check(Extension(pool): Extension<Pool>) -> &'static str {
    match pool.get().await {
        Ok(_) => "Database connection is healthy",
        Err(_) => "Failed to connect to the database",
    }
}

async fn handle_file_error(error: io::Error) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {}", error),
    )
}
