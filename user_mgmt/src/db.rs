use anyhow::Result;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn init_db() -> Result<SqlitePool> {
    let db = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;

    // Create schema
    sqlx::query(
        r#"
        CREATE TABLE users (
            id    TEXT PRIMARY KEY,
            name  TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        );
        CREATE TABLE posts (
            id      TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            title   TEXT NOT NULL,
            content TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
        CREATE TABLE comments (
            id      TEXT PRIMARY KEY,
            post_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            content TEXT NOT NULL,
            FOREIGN KEY (post_id) REFERENCES posts(id),
            FOREIGN KEY (user_id) REFERENCES users(id)
        ); 
        "#,
    )
    .execute(&db)
    .await?;

    Ok(db)
}
