use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use crate::routes::routes::router;

mod routes;
mod messages;
mod health;

async fn init_deadpool() -> Pool {
    let mut config = Config::new();
    config.user = Some("gogo".to_string());
    config.password = Some("gogo".to_string());
    config.dbname = Some("gogo".to_string());
    config.host = Some("127.0.0.1".into());
    config.port = Some(5432);

    config.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    return config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
}

#[tokio::main]
async fn main() {
    let pool= init_deadpool().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router(pool)).await.unwrap();
}
