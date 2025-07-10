use crate::core::state::AppState;
use crate::r#gen::gogo::message::v1::message_service_server::MessageService;
use crate::r#gen::gogo::message::v1::{
    AddMessageRequest, AddMessageResponse, ReadMessageRequest, ReadMessageResponse,
};
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

#[tonic::async_trait]
impl MessageService for GrpcMessageService {
    async fn add_message(
        &self,
        request: Request<AddMessageRequest>,
    ) -> Result<Response<AddMessageResponse>, Status> {
        todo!()
    }

    async fn read_message(
        &self,
        request: Request<ReadMessageRequest>,
    ) -> Result<Response<ReadMessageResponse>, Status> {
        todo!()
    }
}
