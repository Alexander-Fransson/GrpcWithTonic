mod data_access;
mod error;
mod get_env;

use data_access::DataAccessManager;
pub use error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    serve_server().await?;
    
    println!("Hello, world!");

    Ok(())
}

async fn serve_server() -> Result<()> {
    let dam = DataAccessManager::new().await?;
    
    Ok(())
}
