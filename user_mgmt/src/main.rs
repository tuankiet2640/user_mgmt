use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct User {
    id: String,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUser {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ✅ In-memory SQLite
    // NOTE: In-memory DB exists per connection. Use max_connections=1 to keep it stable.
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
        "#,
    )
    .execute(&db)
    .await?;

    let state = AppState { db };

    let app = Router::new()
        .route("/users", post(create_user).get(list_users))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .with_state(state);

    let addr = "127.0.0.1:8080";
    println!("Server running at http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// ---------------- Handlers ----------------

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let id = Uuid::new_v4().to_string();

    let result = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, name, email)
        VALUES (?1, ?2, ?3)
        RETURNING id, name, email;
        "#,
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(&payload.email)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(e) => {
            // simple unique constraint mapping
            if e.to_string().contains("UNIQUE") {
                Err((StatusCode::CONFLICT, "Email already exists".into()))
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        }
    }
}

async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users ORDER BY name;")
        .fetch_all(&state.db)
        .await
        .map_err(internal_error)?;

    Ok(Json(users))
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?1;")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(internal_error)?;

    match user {
        Some(u) => Ok(Json(u)),
        None => Err((StatusCode::NOT_FOUND, "User not found".into())),
    }
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let updated = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET name = ?1, email = ?2
        WHERE id = ?3
        RETURNING id, name, email;
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&id)
    .fetch_optional(&state.db)
    .await;

    match updated {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "User not found".into())),
        Err(e) => {
            if e.to_string().contains("UNIQUE") {
                Err((StatusCode::CONFLICT, "Email already exists".into()))
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        }
    }
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?1;")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(internal_error)?;

    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "User not found".into()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

fn internal_error<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}