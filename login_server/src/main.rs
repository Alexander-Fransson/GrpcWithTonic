mod data_access;
mod error;
mod get_env;
mod log;
mod utils;
mod views;
mod crypt;
mod grpc;
mod proto;
mod request_context;

#[cfg(test)]
mod integration_tests;

use std::net::SocketAddr;

pub use error::{Error, Result};

use data_access::DataAccessManager;
use grpc::{middlewares::mw_implant_request_context::InterceptorImplantingRequestContext, services::auth::AuthService};
use log::tracing::enable_tracing;
use tonic::transport::Server;
use tonic_middleware::RequestInterceptorLayer;
use tracing::info;
use proto::authenticate_server::AuthenticateServer;

pub const SERVER_ADRESS: &str = "127.0.0.1:50051";

#[tokio::main]
async fn main() -> Result<()> {
    enable_tracing();
    serve_server().await?;
    
    info!("Hello, world!");

    Ok(())
}

pub async fn serve_server() -> Result<()> {
    let addr: SocketAddr = SERVER_ADRESS.parse()
    .map_err(|_| Error::FailedToParse("server address".to_string()))?;

    let port = addr.to_string();

    info!("Starting server on port {}", port);

    let dam = DataAccessManager::new().await?;
    let auth = AuthService { dam: dam.clone() };
    let implant_request_context = InterceptorImplantingRequestContext { dam: dam.clone() };

    let file_descriptor_service = 
    tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
    .build_v1()
    .map_err(|e| Error::FailedToBuildFileDescriptorService(e.to_string()))?;

    Server::builder()
    .accept_http1(true)
    .layer(RequestInterceptorLayer::new(implant_request_context))
    .add_service(file_descriptor_service)
    .add_service(AuthenticateServer::new(auth))
    .serve(addr)
    .await
    .map_err(|e| Error::FailedToStartGrpcServer(e.to_string()))?;

    Ok(())
}
