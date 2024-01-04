use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use crate::routes::routes::router;

mod routes;
mod messages;
mod health;

async fn init_deadpool() {
    let mut config = Config::new();
    config.user = Some("gogo".to_string());
    config.password = Some("gogo".to_string());
    config.dbname = Some("gogo".to_string());
    config.host = Some("127.0.0.1".into());
    config.port = Some(5432);

    config.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    for i in 1..10 {
        let client = pool.get().await.unwrap();
        let stmt = client.prepare_cached("SELECT 1 + $1").await.unwrap();
        let rows = client.query(&stmt, &[&i]).await.unwrap();
        let value: i32 = rows[0].get(0);
        assert_eq!(value, i + 1);
    }
}

#[tokio::main]
async fn main() {
    init_deadpool().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router()).await.unwrap();
}
