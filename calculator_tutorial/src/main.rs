// creates a proto module and includes the calculator proto file
// this must be where the (file_name)_server module is generated

// is a server generated for each service?
mod proto {
    tonic::include_proto!("calculator");

    pub (crate) const FILE_DESCRIPTOR_SET: &[u8] = 
    tonic::include_file_descriptor_set!("calculator_descriptor");
}

use proto::calculator_server::{
    Calculator, // the trait generated requiering the functions specified in the .proto file 
    CalculatorServer // the generated server which turns the service implementing the calculator trait into a grpc server
}; 
use proto::admin_server::{Admin, AdminServer};
use tonic::transport::Server;

// for middlewares
use tonic::metadata::MetadataValue;
use tonic::{Request, Status};

// thread safe value which can be shared between threads
type State = std::sync::Arc<tokio::sync::RwLock<u64>>;

#[derive(Debug, Default)]
struct CalculatorService {
    uses: State
} // the container for the implementation of the calculator service

impl CalculatorService {
    async fn increment(&self) {
        let mut counter = self.uses.write().await;
        *counter += 1;
        println!("Current count is {}", *counter);
    }
}

struct AdminService {
    uses: State
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_request_count(
        &self, 
        _request: tonic::Request<proto::RequestCountRequest>
    ) -> Result<tonic::Response<proto::RequestCountResponse>, tonic::Status> {

        let count = self.uses.read().await;

        let response = proto::RequestCountResponse {
            count: *count as i64
        };

        Ok(tonic::Response::new(response))
    }
}

// an interceptor checking for token
fn check_auth(req:Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer some secret token".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("invalid token"))
    }    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = "[::1]:50051".parse()?; // [::1] = 127.0.0.1 = localhost  

    let state = State::default();

    let calc = CalculatorService{
        uses: state.clone()
    };  
    let admin = AdminService{
        uses: state.clone()
    };

    // enables reflection
    let service = 
    tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
    .build_v1()?;

    Server::builder()
    .accept_http1(true)
    .layer(tower_http::cors::CorsLayer::permissive())
    .add_service(service)
    .add_service(CalculatorServer::new(calc))

    // adds the admin service with an interceptor middleware
    .add_service(AdminServer::with_interceptor(admin, check_auth))
    .serve(addr)
    .await?;

    Ok(())
}

// implementation of the service defined in the proto file
#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self, 
        request: tonic::Request<proto::CalculationRequest>
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);

        self.increment().await;

        let input = request.get_ref();

        let response = proto::CalculationResponse {
            result: input.a + input.b
        };

        Ok(tonic::Response::new(response))
    }

    async fn devide(
        &self, 
        request: tonic::Request<proto::CalculationRequest>
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);

        self.increment().await;

        let input = request.get_ref();

        if input.b == 0 {
            // creates an error message and statuscode 
            return Err(tonic::Status::invalid_argument("Cannot devide by 0"));
        }

        let response = proto::CalculationResponse {
            result: input.a / input.b
        };

        Ok(tonic::Response::new(response))
    }
}