use axum::{Router, extract::State, routing::get};
use deadpool_postgres::{Config, Pool, Runtime};
use shared::{AppError, AppResult};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    let pool = create_pool();

    let app = Router::new()
        .route("/", get(health_check))
        .route("/db-check", get(db_check))
        .with_state(pool.clone())
        .merge(module_categories::router(pool.clone()))
        .merge(module_products::router(pool.clone()))
        .merge(module_images::router(pool.clone()))
        .merge(module_authentication::router(
            pool,
            module_authentication::AuthConfig::from_env(),
        ));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind address");

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

fn create_pool() -> Pool {
    let mut config = Config::new();

    config.host = Some("localhost".to_string());
    config.port = Some(5455);
    config.user = Some("postgres".to_string());
    config.password = Some("products_dev_password".to_string());
    config.dbname = Some("products_db".to_string());

    config
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Failed to create database pool")
}

async fn health_check() -> &'static str {
    "Products API is running"
}

async fn db_check(State(db): State<Pool>) -> AppResult<String> {
    let client = db.get().await.map_err(AppError::internal)?;

    let row = client
        .query_one("SELECT current_database()", &[])
        .await
        .map_err(AppError::internal)?;

    let database: String = row.get(0);

    Ok(format!("Connected to database: {database}"))
}
