pub mod api {
    tonic::include_proto!("ping");
}

mod in_memory_network;

use tokio::sync::RwLock;
use std::sync::Arc;
use rand::Rng;
use tonic::service::Interceptor;
use tonic::{Request, Status};
use tonic::transport::Endpoint;
use crate::api::ping_service_client::PingServiceClient;
use crate::api::PingRequest;
use crate::in_memory_network::InMemoryNetwork;


struct NetworkInterceptor {
    network: Arc<RwLock<InMemoryNetwork>>,
    host: String,
    target: String,
}

impl NetworkInterceptor {
    pub fn new(network: Arc<RwLock<InMemoryNetwork>>, host: String, target: String) -> Self {
        Self { network, host, target }
    }
}


impl Interceptor for NetworkInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        // if host and target are connected, allow the request
        // otherwise have it fail
        Ok(request)
    }
}



struct RandomFailInterceptor;

impl Interceptor for RandomFailInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        if rand::thread_rng().gen_bool(0.5) {
            println!("Allowing request to proceed.");
            Ok(request)
        } else {
            println!("Blocking request with an error.");
            Err(Status::internal("Randomly blocked by interceptor"))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the interceptor
    let interceptor = RandomFailInterceptor;

    // Create a channel and attach the interceptor
    let channel = Endpoint::from_static("http://[::1]:50051")
        .connect()
        .await?;

    // Create the client with the intercepted channel
    let mut client = PingServiceClient::with_interceptor(channel,interceptor);

    // Example request
    let request = tonic::Request::new(PingRequest {
        message: "hello there".to_string(),
    });

    match client.ping(request).await {
        Ok(response) => println!("RESPONSE={:?}", response.get_ref()),
        Err(e) => println!("Request failed: {:?}", e),
    }

    Ok(())
}