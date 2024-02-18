
pub mod api {
    tonic::include_proto!("ping");
}

use tonic::{Request, Response, Status};
use crate::api::ping_service_server::{PingService, PingServiceServer};
use crate::api::{PingRequest, PingResponse};

#[derive(Default)]
struct Server {
    peers: Vec<Server>,
    // state: State,
}

impl PingService for Server {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        println!("got a request from {:?}", request.remote_addr());
        let reply = PingResponse {
            healthy: true,
        };
        Ok(Response::new(reply))
    }
}


// Runtime to run our server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let srv = Server::default();

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(PingServiceServer::new(srv))
        .serve(addr)
        .await?;

    Ok(())
}