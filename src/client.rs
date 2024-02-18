pub mod api {
    tonic::include_proto!("ping");
}

use crate::api::ping_service_client::PingServiceClient;
use crate::api::PingRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PingServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(PingRequest {
        message: "hello there".to_string(),
    });

    println!("Sending request to gRPC Server...");
    let response = client.ping(request).await?;
    let res = response.get_ref();
    println!("RESPONSE={:?}", response);
    println!("RESPONSE={:?}", res);

    Ok(())
}