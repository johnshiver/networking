
pub mod api {
    tonic::include_proto!("ping");
}

use tokio::signal;
use tonic::{async_trait, Request, Response, Status};
use tonic::transport::Server;
use crate::api::ping_service_server::{PingService, PingServiceServer};
use crate::api::{PingRequest, PingResponse};
use tokio::sync::oneshot::{self, Receiver, Sender};

#[derive(Default)]
struct MyServer {
    // peers: Vec<Server>,
    // state: State,
}

#[async_trait]
impl PingService for MyServer {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let reply = PingResponse { healthy: true, };
        Ok(Response::new(reply))
    }
}


pub fn signal_channel() -> (Sender<()>, Receiver<()>) {
    oneshot::channel()
}

pub async fn wait_for_signal(tx: Sender<()>) {
    let _ = signal::ctrl_c().await;
    println!("SIGINT received: shutting down");
    let _ = tx.send(());
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (signal_tx, signal_rx) = signal_channel();
    let _ = tokio::spawn(wait_for_signal(signal_tx));


    let addr = "[::1]:50051".parse()?;
    let srv = MyServer::default();

    println!("Starting gRPC Server...");
    let server = Server::builder()
        .add_service(PingServiceServer::new(srv))
        .serve_with_shutdown(addr, async {
            signal_rx.await.ok();
            println!("Graceful context shutdown");
        });

    server.await?;
    Ok(())
}