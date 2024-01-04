use crate::routes::routes::router;

mod routes;
mod messages;
mod health;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router()).await.unwrap();
}
