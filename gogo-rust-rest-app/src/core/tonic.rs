use crate::core::errors::ApplicationError;
use crate::core::state::AppState;
use crate::r#gen::gogo::message::v1::message_service_server::MessageServiceServer;
use crate::messages::grpc::GrpcMessageService;
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;
use tonic_health::server::health_reporter;

pub struct Tonic;

impl Tonic {
    pub async fn run(state: Arc<AppState>, address: &str) -> Result<(), ApplicationError> {
        let (health_reporter, health_service) = health_reporter();

        let socket_address: SocketAddr = address.parse()?;

        let mut server = Server::builder().add_service(health_service);

        let message_server = MessageServiceServer::new(GrpcMessageService::new(state.clone()));
        server = server.add_service(message_server);

        server.serve(socket_address).await?;

        Ok(())
    }
}
