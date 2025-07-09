use crate::core::errors::ApplicationError;
use crate::core::state::AppState;
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;
use tonic_health::server::health_reporter;

pub struct Tonic;

impl Tonic {
    pub async fn run(state: Arc<AppState>, address: &str) -> Result<(), ApplicationError> {
        let (health_reporter, health_service) = health_reporter();

        let socket_address: SocketAddr = address.parse()?;

        Server::builder()
            .add_service(health_service)
            .serve(socket_address)
            .await?;

        Ok(())
    }
}
