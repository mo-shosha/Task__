mod domain;
mod http;
mod repository;

use axum::Router;
use deadpool_postgres::Pool;

use repository::ProductRepository;

pub fn router(db: Pool) -> Router {
    http::router(ProductRepository::new(db))
}
