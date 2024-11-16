use std::{env, fs, option};

use context::AppContext;
use db::setup_db;
use errors::ApplicationError;
use router::init_router;
use tokio::net::TcpListener;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod actions;
mod context;
mod db;
mod errors;
mod router;
mod templates;

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    if let Err(e) = run().await {
        // Print the error using Display
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

async fn run() -> Result<(), ApplicationError> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let data_file = env::var("DATABASE_PATH")
        .map_err(|e| ApplicationError::EnvError(e, "DATABASE_PATH".to_string()))?;

    let last_number = fs::read_to_string(&data_file)
        .map(option::Option::Some)
        .unwrap_or_default();

    info!("Last number: {:?}", last_number);

    let db = setup_db(&data_file)
        .await
        .map_err(|e| ApplicationError::from(e))?;

    let app = init_router();

    let host = std::env::var("SERIGEN_HOST")
        .map_err(|e| ApplicationError::EnvError(e, "SERIGEN_HOST".to_string()))?;
    let port = std::env::var("SERIGEN_PORT")
        .map_err(|e| ApplicationError::EnvError(e, "SERIGEN_PORT".to_string()))?;

    let address = format!("{}:{}", host, port);
    info!("Starting server on {}", address);

    let listener = TcpListener::bind(address)
        .await
        .map_err(|e| ApplicationError::from(e))?;

    info!("Listening on: {}", listener.local_addr().unwrap());
    let extended = app
        .layer(AddExtensionLayer::new(AppContext::new(db)))
        .layer(TraceLayer::new_for_http());

    axum::serve(listener, extended)
        .await
        .map_err(|e| ApplicationError::CannotServe(e))?;
    Ok(())
}
