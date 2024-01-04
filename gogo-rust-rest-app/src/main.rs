use crate::routes::routes::router;

mod routes;
mod messages;
mod health;
mod db;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let router = router().await;
    axum::serve(listener, router).await.unwrap();
}
