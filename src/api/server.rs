use crate::{database::provider::DatabaseProvider, evm::evm_service::EvmHandler, model::UserInfo};
use axum::{
    http::StatusCode,
    routing::{delete, get, post, put},
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

    pub async fn deploy_contract(self: Arc<Self>) -> StatusCode {
        info!("Deploying  contract");
        let mut evm_handler = EvmHandler::new(self.database.clone());
        match evm_handler.excute_contract() {
            Ok(_) => StatusCode::CREATED,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    pub async fn create_user(self: Arc<Self>, user: Json<UserInfo>) -> StatusCode {
        info!("Creating user");
        let mut evm_handler = EvmHandler::new(self.database.clone());
        let result = evm_handler.call_add_user_entry(*user);
        match self.database.create_user(*user) {
            Ok(_) => StatusCode::CREATED,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn update_user(self: Arc<Self>, user: Json<UserInfo>) -> StatusCode {
        info!("Updating user");
        match self.database.update_user(*user) {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn create_router(self: Arc<Self>) -> Router {
        Router::new()
            .route(
                "/deploy_contract",
                post({
                    let handler = Arc::clone(&self);
                    move || handler.deploy_contract()
                }),
            )
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
    }
}
