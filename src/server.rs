pub mod api {
    tonic::include_proto!("ping");
}

use crate::api::ping_service_server::{PingService, PingServiceServer};
use crate::api::{PingRequest, PingResponse};
use tokio::signal;
use tokio::sync::oneshot::{self, Receiver, Sender};
use tonic::transport::Server;
use tonic::{async_trait, Request, Response, Status};

#[derive(Default)]
struct SimpleServer {
    // peers: Vec<Server>,
    // state: State,
}

#[async_trait]
impl PingService for SimpleServer {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        println!(
            "Got a request from {}",
            request.remote_addr().unwrap().to_string()
        );
        let reply = PingResponse { healthy: true };
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

pub fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
    println!("received {:?}", req.remote_addr());
    println!("received {:?}", req.local_addr());
    println!("received {:?}", req.extensions());
    Ok(req)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (signal_tx, signal_rx) = signal_channel();
    let _ = tokio::spawn(wait_for_signal(signal_tx));

    let addr = "[::1]:50051".parse()?;
    let srv = SimpleServer::default();

    println!("Starting gRPC Server...");
    let server = Server::builder()
        .layer(tonic::service::interceptor(intercept))
        .add_service(PingServiceServer::new(srv))
        .serve_with_shutdown(addr, async {
            signal_rx.await.ok();
            println!("Graceful context shutdown");
        });

    server.await?;
    Ok(())
}
