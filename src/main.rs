use std::env;

use db::create_db_pool;
use errors::ApplicationError;
use router::setup_router;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod actions;
mod db;
mod errors;
mod forms;
mod jwt;
mod middleware;
mod models;
mod router;
mod state;
mod templates;
mod utils;

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
    setup_tracing();

    let (host, port, jwt_secret, data_file) = setup_env()?;

    let db = setup_db(data_file).await?;

    let app = setup_router(db, &jwt_secret);

    let address = format!("{}:{}", host, port);
    info!("Starting server on {}", address);

    let listener = TcpListener::bind(address)
        .await
        .map_err(|e| ApplicationError::from(e))?;

    info!("Listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .map_err(|e| ApplicationError::CannotServe(e))?;
    Ok(())
}

fn setup_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{crate_name}=debug,*=info,tower_http=debug",
                    crate_name = env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn setup_db(data_file: String) -> Result<sqlx::Pool<sqlx::Sqlite>, ApplicationError> {
    let db = create_db_pool(&data_file)
        .await
        .map_err(|e| ApplicationError::from(e))?;
    Ok(db)
}

fn setup_env() -> Result<(String, String, String, String), ApplicationError> {
    dotenv::dotenv().ok();

    let host = std::env::var("SERIGEN_HOST")
        .map_err(|e| ApplicationError::EnvError(e, "SERIGEN_HOST".to_string()))?;
    let port = std::env::var("SERIGEN_PORT")
        .map_err(|e| ApplicationError::EnvError(e, "SERIGEN_PORT".to_string()))?;
    let jwt_secret = std::env::var("SERIGEN_JWT_SECRET")
        .map_err(|e| ApplicationError::EnvError(e, "SERIGEN_JWT_SECRET".to_string()))?;
    let data_file = env::var("DATABASE_PATH")
        .map_err(|e| ApplicationError::EnvError(e, "DATABASE_PATH".to_string()))?;
    Ok((host, port, jwt_secret, data_file))
}
