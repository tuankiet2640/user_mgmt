use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::{create_comment, delete_comment, get_comment, list_comments, update_comment};
use crate::handlers::{create_post, delete_post, get_post, list_posts, update_post};
use crate::handlers::{create_user, delete_user, get_user, list_users, update_user};
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

pub fn build_app(state: AppState) -> Router {
    Router::new()
        //users
        .route("/users", post(create_user).get(list_users))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        //posts
        .route("/posts", post(create_post).get(list_posts))
        .route(
            "/posts/:id",
            get(get_post).put(update_post).delete(delete_post),
        )
        //comments
        .route("/comments", post(create_comment).get(list_comments))
        .route(
            "/comments/:id",
            get(get_comment).put(update_comment).delete(delete_comment),
        )
        .with_state(state)
}
