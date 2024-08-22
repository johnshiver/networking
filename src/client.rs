pub mod api {
    tonic::include_proto!("ping");
}

mod in_memory_network;

use crate::api::ping_service_client::PingServiceClient;
use crate::api::PingRequest;
use crate::in_memory_network::InMemoryNetwork;
use rand::Rng;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use tonic::metadata::MetadataValue;
use tonic::service::Interceptor;
use tonic::transport::Endpoint;
use tonic::{Request, Status};

#[derive(Clone)]
struct NetworkInterceptor {
    network: Arc<RwLock<InMemoryNetwork>>,
    host: String,
}

impl NetworkInterceptor {
    pub fn new(network: Arc<RwLock<InMemoryNetwork>>, host: String) -> Self {
        Self { network, host }
    }

    pub fn clear_connections(&mut self) {
        let mut network = self.network.write().unwrap();
        network.clear_connections();
    }
}

impl Interceptor for NetworkInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        // Retrieve target-host metadata
        // Retrieve target-host or raise an error if it is missing
        let target_host = request
            .metadata()
            .get("target-host")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| Status::invalid_argument("target-host metadata is missing"))?;

        let network = self.network.read().unwrap();
        // if host and target are connected, allow the request
        // otherwise have it fail
        if !network.are_connected(target_host, &self.host) {
            return Err(Status::internal("Network nodes not connected"));
        }
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
    // let interceptor = RandomFailInterceptor;
    let client_name = "host1";
    let target_name = "http://[::1]:50051";
    let mut in_memory_network = InMemoryNetwork::new();
    in_memory_network.add_node(client_name);
    in_memory_network.add_node(target_name);
    in_memory_network.add_edge(client_name, target_name);
    let interceptor = NetworkInterceptor::new(
        Arc::new(RwLock::new(in_memory_network)),
        client_name.to_string(),
    );

    // Create a channel and attach the interceptor
    let channel = Endpoint::from_static(target_name).connect().await?;

    // Create the client with the intercepted channel
    let mut client = PingServiceClient::with_interceptor(channel, interceptor.clone());

    // Example request
    let mut request = Request::new(PingRequest {
        message: "hello there".to_string(),
    });
    request
        .metadata_mut()
        .insert("target-host", MetadataValue::from_str(target_name).unwrap());

    match client.ping(request).await {
        Ok(response) => println!("RESPONSE={:?}", response.get_ref()),
        Err(e) => println!("Request failed: {:?}", e),
    }

    interceptor.network.write().unwrap().clear_connections();

    let mut request = Request::new(PingRequest {
        message: "hello there 2".to_string(),
    });
    // Add target-host metadata
    request
        .metadata_mut()
        .insert("target-host", MetadataValue::from_str(target_name).unwrap());
    match client.ping(request).await {
        Ok(response) => println!("RESPONSE={:?}", response.get_ref()),
        Err(e) => println!("Request failed: {:?}", e),
    }

    Ok(())
}
