use axum::{
    http::StatusCode,
    routing::{delete, post, put},
    Json, Router,
};
use log::info;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

pub async fn create_user(Json(user): Json<User>) -> StatusCode {
    info!("Creating user");
    StatusCode::CREATED
}

pub async fn update_user(Json(user): Json<User>) -> StatusCode {
    info!("Updating user");
    StatusCode::OK
}

pub async fn delete_user(Json(user): Json<User>) -> StatusCode {
    info!("Deleting user");
    StatusCode::OK
}

pub fn create_router() -> Router {
    Router::new()
        .route("/create_user", post(create_user))
        .route("/update_user", put(update_user))
        .route("/delete_user", delete(delete_user))
}
