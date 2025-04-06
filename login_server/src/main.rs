mod data_access;
mod error;
mod get_env;
mod log;

use data_access::DataAccessManager;
pub use error::{Error, Result};
use log::tracing::enable_tracing;

#[tokio::main]
async fn main() -> Result<()> {
    enable_tracing();
    serve_server().await?;
    
    println!("Hello, world!");

    Ok(())
}

async fn serve_server() -> Result<()> {
    let dam = DataAccessManager::new().await?;
    
    Ok(())
}
