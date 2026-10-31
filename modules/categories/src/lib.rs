mod domain;
mod http;
mod repository;

use axum::Router;
use deadpool_postgres::Pool;

use repository::CategoryRepository;

pub fn router(db: Pool) -> Router {
    http::router(CategoryRepository::new(db))
}
