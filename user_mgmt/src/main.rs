mod app;
mod db;
mod handlers;
mod models;

use crate::app::AppState;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let db = db::init_db().await?;
    let state = AppState { db };
    let app = app::build_app(state);

    let addr = "127.0.0.1:8080";
    println!("Server running at http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
