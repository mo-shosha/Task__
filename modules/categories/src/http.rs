use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use shared::{AppError, AppResult};

use crate::{
    domain::{Category, CategoryInput},
    repository::CategoryRepository,
};

pub fn router(repository: CategoryRepository) -> Router {
    Router::new()
        .route("/categories", post(create_category).get(get_categories))
        .route(
            "/categories/{id}",
            get(get_category)
                .put(update_category)
                .delete(delete_category),
        )
        .with_state(repository)
}

#[derive(Deserialize)]
struct CategoryRequest {
    name: String,
    description: Option<String>,
}

impl From<CategoryRequest> for CategoryInput {
    fn from(request: CategoryRequest) -> Self {
        Self {
            name: request.name,
            description: request.description.unwrap_or_default(),
        }
    }
}

#[derive(Serialize)]
struct CategoryResponse {
    id: i64,
    name: String,
    description: String,
}

impl From<Category> for CategoryResponse {
    fn from(category: Category) -> Self {
        Self {
            id: category.id,
            name: category.name,
            description: category.description,
        }
    }
}

async fn create_category(
    State(repository): State<CategoryRepository>,
    Json(payload): Json<CategoryRequest>,
) -> AppResult<(StatusCode, Json<CategoryResponse>)> {
    let category = repository.create(&payload.into()).await?;

    Ok((StatusCode::CREATED, Json(category.into())))
}

async fn get_categories(
    State(repository): State<CategoryRepository>,
) -> AppResult<Json<Vec<CategoryResponse>>> {
    let categories = repository.list().await?;

    Ok(Json(categories.into_iter().map(Into::into).collect()))
}

async fn get_category(
    State(repository): State<CategoryRepository>,
    Path(id): Path<i64>,
) -> AppResult<Json<CategoryResponse>> {
    let category = repository.find(id).await?.ok_or(AppError::NotFound)?;

    Ok(Json(category.into()))
}

async fn update_category(
    State(repository): State<CategoryRepository>,
    Path(id): Path<i64>,
    Json(payload): Json<CategoryRequest>,
) -> AppResult<Json<CategoryResponse>> {
    let category = repository
        .update(id, &payload.into())
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(category.into()))
}

async fn delete_category(
    State(repository): State<CategoryRepository>,
    Path(id): Path<i64>,
) -> AppResult<StatusCode> {
    repository.delete(id).await?.ok_or(AppError::NotFound)?;

    Ok(StatusCode::NO_CONTENT)
}
