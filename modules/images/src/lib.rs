mod domain;
mod http;
mod repository;

use axum::Router;
use deadpool_postgres::Pool;

use repository::ImageRepository;

pub fn router(db: Pool) -> Router {
    http::router(ImageRepository::new(db))
}
