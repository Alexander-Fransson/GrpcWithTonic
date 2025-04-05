mod proto {
    tonic::include_proto!("calculator");
}

use std::error::Error;
use proto::calculator_client::CalculatorClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50051";

    let mut client = CalculatorClient::connect(url).await?;

    // test add

    let req = proto::CalculationRequest {a: 1, b: 2};
    let request = tonic::Request::new(req);

    let res = client.add(request).await?;

    println!("Response of add was: {}", res.get_ref().result);

    // test admin

    let req = proto::RequestCountRequest {};
    let request = tonic::Request::new(req);

    let mut admin_clint = proto::admin_client::AdminClient::connect(url).await?;

    let res = admin_clint.get_request_count(request).await?;

    println!("Response of get_request_count was: {}", res.get_ref().count);

    // test devide

    let req = proto::CalculationRequest {a: 1, b: 0};
    let request = tonic::Request::new(req);

    let res = client.devide(request).await?;

    println!("Response of devide was: {}", res.get_ref().result);

    Ok(())
}