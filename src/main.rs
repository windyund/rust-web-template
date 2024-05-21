use axum::{Extension, Router};
use serde::{Deserialize, Serialize};
use rust_web::{config, db, router};
use std::sync::Arc;

mod models;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();

    db::axredis::init_redis_pool().await.unwrap();

    let pool = db::axmongo::init_db_pool().await.unwrap();

    let app_data = config::AppState { db_pool: pool };
    
    // register routers
    let app = Router::new()
        .nest("/", router::routers())
        .layer(Extension(Arc::new(app_data)));
    
    let addr = "0.0.0.0:3000";
    tracing::info!("Serve running at http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
