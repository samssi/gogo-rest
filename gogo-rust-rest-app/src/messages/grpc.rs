use crate::core::state::AppState;
use crate::r#gen::gogo::message::v1::message_service_server::MessageService;
use crate::r#gen::gogo::message::v1::{
    AddMessageRequest, AddMessageResponse, ReadMessageRequest, ReadMessageResponse,
};
use crate::messages::service;
use crate::messages::service::MessageServiceError;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct GrpcMessageService {
    state: Arc<AppState>,
}

impl GrpcMessageService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

impl From<MessageServiceError> for Status {
    fn from(error: MessageServiceError) -> Self {
        match error {
            MessageServiceError::QueryError(error) => Status::internal(error.to_string()),
        }
    }
}

#[tonic::async_trait]
impl MessageService for GrpcMessageService {
    async fn add_message(
        &self,
        request: Request<AddMessageRequest>,
    ) -> Result<Response<AddMessageResponse>, Status> {
        println!("Got AddMessage from {:?}", request.remote_addr());
        let add_message_request = request.into_inner();

        service::Message::add_message(self.state.clone(), add_message_request.message).await?;
        Ok(Response::new(AddMessageResponse {}))
    }

    async fn read_message(
        &self,
        request: Request<ReadMessageRequest>,
    ) -> Result<Response<ReadMessageResponse>, Status> {
        let message = service::Message::read_message(self.state.clone()).await?;

        match message {
            Some(message) => Ok(Response::new(ReadMessageResponse { message })),
            None => Err(Status::not_found("Message not found")),
        }
    }
}
