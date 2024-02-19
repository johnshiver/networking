pub mod api {
    tonic::include_proto!("ping");
}

use crate::api::ping_service_client::PingServiceClient;
use crate::api::PingRequest;
use tonic::service::Interceptor;
use tonic::transport::Channel;
use tonic::{Request, Status};
use tower::ServiceBuilder;

// Define your interceptor
// struct LoggingInterceptor;
//
// impl Interceptor for LoggingInterceptor {
//     fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
//         println!("Request to: {:?}", request.remote_addr());
//         Ok(request)
//     }
// }

// An interceptor function.
fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
    println!("received {:?}", req.remote_addr());
    println!("received {:?}", req.local_addr());
    Ok(req)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = PingServiceClient::connect("http://[::1]:50051").await?;
    // let dst = "http://[::1]:50051";
    // let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
    // let interceptor = LoggingInterceptor;
    // let mut client = PingServiceClient::with_interceptor(conn, interceptor);
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;

    let channel = ServiceBuilder::new()
        // Interceptors can be also be applied as middleware
        .layer(tonic::service::interceptor(intercept))
        .service(channel);

    let mut client = PingServiceClient::new(channel);

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
