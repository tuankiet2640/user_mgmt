use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    models::{CreateUser, Post, UpdateUser, User},
    models::{CreatePost, UpdatePost},
};

//users
pub async fn create_user(
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
            if e.to_string().contains("UNIQUE") {
                Err((StatusCode::CONFLICT, "Email already exists".into()))
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        }
    }
}

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users ORDER BY name;")
        .fetch_all(&state.db)
        .await
        .map_err(internal_error)?;

    Ok(Json(users))
}

pub async fn get_user(
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

pub async fn update_user(
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

pub async fn delete_user(
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

//posts
pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePost>,
) -> Result<(StatusCode, Json<Post>), (StatusCode, String)> {
    let id = Uuid::new_v4().to_string();

    let result = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts (id, user_id, title, content)
        VALUES (?1, ?2, ?3, ?4)
        RETURNING id, user_id, title, content;
        "#,
    )
    .bind(&id)
    .bind(&payload.user_id)
    .bind(&payload.title)
    .bind(&payload.content)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(post) => Ok((StatusCode::CREATED, Json(post))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn list_posts(
    State(state): State<AppState>,
) -> Result<Json<Vec<Post>>, (StatusCode, String)> {
    let posts =
        sqlx::query_as::<_, Post>("SELECT id, user_id, title, content FROM posts ORDER BY title;")
            .fetch_all(&state.db)
            .await
            .map_err(internal_error)?;

    Ok(Json(posts))
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Post>, (StatusCode, String)> {
    let post =
        sqlx::query_as::<_, Post>("SELECT id, user_id, title, content FROM posts WHERE id = ?1;")
            .bind(id)
            .fetch_optional(&state.db)
            .await
            .map_err(internal_error)?;
    match post {
        Some(p) => Ok(Json(p)),
        None => Err((StatusCode::NOT_FOUND, "Post not found".into())),
    }
}

pub async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdatePost>,
) -> Result<Json<Post>, (StatusCode, String)> {
    let updated = sqlx::query_as::<_, Post>(
        r#"
        UPDATE posts
        SET user_id = ?1, title = ?2, content = ?3
        WHERE id = ?4
        RETURNING id, user_id, title, content;
        "#,
    )
    .bind(&payload.user_id)
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&id)
    .fetch_optional(&state.db)
    .await;

    match updated {
        Ok(Some(post)) => Ok(Json(post)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Post not found".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
pub async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM posts WHERE id = ?1;")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(internal_error)?;

    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Post not found".into()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

//comments
pub async fn create_comment() {
    //TODO
}
pub async fn list_comments() {
    //TODO
}
pub async fn get_comment() {
    //TODO
}
pub async fn update_comment() {
    //TODO
}
pub async fn delete_comment() {
    //TODO
}
