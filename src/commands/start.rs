use crate::api::config::Config as APIConfig;
use crate::api::server::ServiceHandler;
use crate::database::postgres::config::Config as PostgresConfig;
use crate::database::postgres::postgres::PostgresDB;
use crate::evm::evm_service::EvmHandler;
use ctrlc;
use log::info;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{panic, sync::Arc};
use structopt::StructOpt;
use tokio::sync::Notify;

#[derive(Debug, StructOpt)]
#[structopt(name = "start")]
pub struct StartCmd {
    #[structopt(flatten)]
    pub api_config: APIConfig,
    #[structopt(flatten)]
    pub postgres_config: PostgresConfig,
}

impl StartCmd {
    pub async fn execute(&self) {
        exit_on_panic();
        info!(
            "Starting Record Keeper service. version: {}.",
            env!("CARGO_PKG_VERSION")
        );
        let notify = Arc::new(Notify::new());
        let n = notify.clone();
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
        let pq = PostgresDB::from_config(&self.postgres_config).unwrap();
        let _evm_handler = EvmHandler::new(Arc::new(pq));
        let pq = PostgresDB::from_config(&self.postgres_config).unwrap();
        let handler = Arc::new(ServiceHandler::new(Arc::new(pq)));
        let app = handler.create_router();
        info!("Listening on {}", self.api_config.address);
        let addr: std::net::SocketAddr = self.api_config.address.parse().expect("Invalid address");
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .expect("Failed to bind");
        info!("Listening on {}", self.api_config.address);
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal(n))
            .await
            .unwrap();
    }
}

pub fn exit_on_panic() {
    // take_hook() returns the default hook in case when a custom one is not set
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        log::error!("Exiting On : {:#?}", panic_info);
        orig_hook(panic_info);
        process::exit(1);
    }));
}
async fn shutdown_signal(notify: Arc<Notify>) {
    notify.notified().await;
}
