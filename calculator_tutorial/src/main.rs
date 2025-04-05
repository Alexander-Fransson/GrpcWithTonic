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
use tonic::transport::Server;

#[derive(Debug)]
struct CalculatorService {} // the container for the implementation of the calculator service

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?; // [::1] = 127.0.0.1 = localhost  

    let calc = CalculatorService{};  

    // enables reflection
    let service = 
    tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
    .build_v1()?;

    Server::builder()
    .add_service(service)
    .add_service(CalculatorServer::new(calc))
    .serve(addr)
    .await?;

    Ok(())
}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self, 
        request: tonic::Request<proto::CalculationRequest>
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        
        println!("Got a request: {:?}", request);

        let input = request.get_ref();

        let response = proto::CalculationResponse {
            result: input.a + input.b
        };

        Ok(tonic::Response::new(response))
    }
}