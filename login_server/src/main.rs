mod data_access;
mod error;
mod get_env;
mod log;
mod utils;

pub use error::{Error, Result};

use data_access::DataAccessManager;
use log::tracing::enable_tracing;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    enable_tracing();
    serve_server().await?;
    
    info!("Hello, world!");

    Ok(())
}

async fn serve_server() -> Result<()> {
    let dam = DataAccessManager::new().await?;
    
    Ok(())
}
