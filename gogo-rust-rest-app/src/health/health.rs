use crate::routes::response::ApiResponse;

pub async fn health_check() -> ApiResponse {
    ApiResponse::Ok
}
