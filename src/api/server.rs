use crate::{database::provider::DatabaseProvider, evm::evm_service::EvmHandler, model::User};
use axum::{
    http::StatusCode,
    routing::{delete, post, put},
    Json, Router,
};
use log::info;
use std::sync::Arc;
#[derive(Clone)]
pub struct ServiceHandler<D>
where
    D: DatabaseProvider + Send + Sync + 'static,
{
    database: Arc<D>,
}

impl<D> ServiceHandler<D>
where
    D: DatabaseProvider + Send + Sync + 'static,
{
    pub fn new(database: Arc<D>) -> Self {
        Self { database }
    }

    pub async fn create_user(self: Arc<Self>, user: Json<User>) -> StatusCode {
        info!("Creating user");
        // let evm_handler = EvmHandler::new(self.database.clone());
        match self.database.create_user(*user) {
            Ok(_) => StatusCode::CREATED,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn update_user(self: Arc<Self>, user: Json<User>) -> StatusCode {
        info!("Updating user");
        match self.database.update_user(*user) {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn remove_user(self: Arc<Self>, user: Json<User>) -> StatusCode {
        info!("Deleting user");
        match self.database.get_info(&user) {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn create_router(self: Arc<Self>) -> Router {
        Router::new()
            .route(
                "/create_user",
                post({
                    let handler = Arc::clone(&self);
                    move |payload| handler.create_user(payload)
                }),
            )
            .route(
                "/update_user",
                put({
                    let handler = Arc::clone(&self);
                    move |payload| handler.update_user(payload)
                }),
            )
            .route(
                "/delete_user",
                delete({
                    let handler = Arc::clone(&self);
                    move |payload| handler.remove_user(payload)
                }),
            )
    }
}
