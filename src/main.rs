pub mod pb {
    tonic::include_proto!("unit.countah.v0");
}

use std::pin::Pin;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, transport::Server};

use crate::pb::{CounterRequest, CounterResponse};

#[derive(Debug, Default)]
pub struct PubdServer {}

#[tonic::async_trait]
impl pb::countah_server::Countah for PubdServer {
    type CounterStream = Pin<Box<dyn Stream<Item = Result<CounterResponse, Status>> + Send>>;

    async fn counter(
        &self,
        _request: Request<CounterRequest>,
    ) -> Result<Response<Self::CounterStream>, Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(128);

        tokio::spawn(async move {
            let mut count: u64 = 0;
            loop {
                count += 1;
                let response = CounterResponse { count };

                if tx.send(Ok(response)).await.is_err() {
                    break;
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });

        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:60069".parse().unwrap();
    let server = PubdServer::default();

    println!("Listening on {}", addr);

    Server::builder()
        .add_service(pb::countah_server::CountahServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
