mod data_access;
mod error;
mod get_env;
mod log;
mod utils;
mod views;
mod crypt;
mod grpc;

mod proto {
    tonic::include_proto!("login_server");
}

pub use error::{Error, Result};

use data_access::DataAccessManager;
use grpc::services::auth::AuthService;
use log::tracing::enable_tracing;
use tonic::transport::Server;
use tracing::info;
use proto::authenticate_server::AuthenticateServer;

#[tokio::main]
async fn main() -> Result<()> {
    enable_tracing();
    serve_server().await?;
    
    info!("Hello, world!");

    Ok(())
}

async fn serve_server() -> Result<()> {
    let addr = "[::1]:50051".parse()
    .map_err(|_| Error::FailedToParse("server address".to_string()))?;

    let dam = DataAccessManager::new().await?;
    let auth = AuthService { dam };

    Server::builder()
    .accept_http1(true)
    .add_service(AuthenticateServer::new(auth))
    .serve(addr)
    .await
    .map_err(|e| Error::FailedToStartGrpcServer(e.to_string()))?;

    Ok(())
}
