use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use shared::{AppError, AppResult};

use crate::{
    domain::{Image, ImageInput},
    repository::ImageRepository,
};

pub fn router(repository: ImageRepository) -> Router {
    Router::new()
        .route(
            "/products/{id}/images",
            post(create_image).get(get_product_images),
        )
        .route(
            "/images/{id}",
            get(get_image).put(update_image).delete(delete_image),
        )
        .with_state(repository)
}

#[derive(Deserialize)]
struct ImageRequest {
    file_path: String,
    #[serde(default)]
    is_primary: bool,
    #[serde(default)]
    display_order: i32,
}

impl From<ImageRequest> for ImageInput {
    fn from(request: ImageRequest) -> Self {
        Self {
            file_path: request.file_path,
            is_primary: request.is_primary,
            display_order: request.display_order,
        }
    }
}

#[derive(Serialize)]
struct ImageResponse {
    id: i64,
    product_id: i64,
    file_path: String,
    is_primary: bool,
    display_order: i32,
}

impl From<Image> for ImageResponse {
    fn from(image: Image) -> Self {
        Self {
            id: image.id,
            product_id: image.product_id,
            file_path: image.file_path,
            is_primary: image.is_primary,
            display_order: image.display_order,
        }
    }
}

async fn create_image(
    State(repository): State<ImageRepository>,
    Path(product_id): Path<i64>,
    Json(payload): Json<ImageRequest>,
) -> AppResult<(StatusCode, Json<ImageResponse>)> {
    let image = repository.create(product_id, &payload.into()).await?;

    Ok((StatusCode::CREATED, Json(image.into())))
}

async fn get_product_images(
    State(repository): State<ImageRepository>,
    Path(product_id): Path<i64>,
) -> AppResult<Json<Vec<ImageResponse>>> {
    let images = repository.list_for_product(product_id).await?;

    Ok(Json(images.into_iter().map(Into::into).collect()))
}

async fn get_image(
    State(repository): State<ImageRepository>,
    Path(id): Path<i64>,
) -> AppResult<Json<ImageResponse>> {
    let image = repository.find(id).await?.ok_or(AppError::NotFound)?;

    Ok(Json(image.into()))
}

async fn update_image(
    State(repository): State<ImageRepository>,
    Path(id): Path<i64>,
    Json(payload): Json<ImageRequest>,
) -> AppResult<Json<ImageResponse>> {
    let image = repository
        .update(id, &payload.into())
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(image.into()))
}

async fn delete_image(
    State(repository): State<ImageRepository>,
    Path(id): Path<i64>,
) -> AppResult<StatusCode> {
    repository.delete(id).await?.ok_or(AppError::NotFound)?;

    Ok(StatusCode::NO_CONTENT)
}
