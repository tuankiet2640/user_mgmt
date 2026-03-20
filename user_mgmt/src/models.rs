use serde::{Deserialize, Serialize};
use sqlx::FromRow;

//users
#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
}

//posts
#[derive(Debug, Serialize, FromRow)]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub user_id: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePost {
    pub user_id: String,
    pub title: String,
    pub content: String,
}

//comments
#[derive(Debug, Serialize, FromRow)]
pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub user_id: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateComment {
    pub post_id: String,
    pub user_id: String,
    pub content: String,
}
