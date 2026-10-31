use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use shared::{AppError, AppResult};

use crate::{
    domain::{Product, ProductDetails, ProductImage, ProductInput, ProductListItem},
    repository::ProductRepository,
};

pub fn router(repository: ProductRepository) -> Router {
    Router::new()
        .route("/products", post(create_product).get(get_products))
        .route(
            "/products/{id}",
            get(get_product)
                .put(update_product)
                .delete(delete_product),
        )
        .with_state(repository)
}

#[derive(Deserialize)]
struct ProductRequest {
    name: String,
    price: i64,
    description: Option<String>,
    category_id: i64,
}

impl From<ProductRequest> for ProductInput {
    fn from(request: ProductRequest) -> Self {
        Self {
            name: request.name,
            price: request.price,
            description: request.description.unwrap_or_default(),
            category_id: request.category_id,
        }
    }
}

#[derive(Serialize)]
struct ProductResponse {
    id: i64,
    name: String,
    price: i64,
    description: String,
    category_id: i64,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            name: product.name,
            price: product.price,
            description: product.description,
            category_id: product.category_id,
        }
    }
}

#[derive(Serialize)]
struct ProductListItemResponse {
    #[serde(flatten)]
    product: ProductResponse,
    category_name: String,
    primary_image_path: Option<String>,
}

impl From<ProductListItem> for ProductListItemResponse {
    fn from(item: ProductListItem) -> Self {
        Self {
            product: item.product.into(),
            category_name: item.category_name,
            primary_image_path: item.primary_image_path,
        }
    }
}

#[derive(Serialize)]
struct ProductDetailsResponse {
    #[serde(flatten)]
    product: ProductResponse,
    images: Vec<ProductImageResponse>,
}

impl From<ProductDetails> for ProductDetailsResponse {
    fn from(details: ProductDetails) -> Self {
        Self {
            product: details.product.into(),
            images: details.images.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Serialize)]
struct ProductImageResponse {
    id: i64,
    file_path: String,
    is_primary: bool,
    display_order: i32,
}

impl From<ProductImage> for ProductImageResponse {
    fn from(image: ProductImage) -> Self {
        Self {
            id: image.id,
            file_path: image.file_path,
            is_primary: image.is_primary,
            display_order: image.display_order,
        }
    }
}

async fn create_product(
    State(repository): State<ProductRepository>,
    Json(payload): Json<ProductRequest>,
) -> AppResult<(StatusCode, Json<ProductResponse>)> {
    let product = repository.create(&payload.into()).await?;

    Ok((StatusCode::CREATED, Json(product.into())))
}

async fn get_products(
    State(repository): State<ProductRepository>,
) -> AppResult<Json<Vec<ProductListItemResponse>>> {
    let products = repository.list().await?;

    Ok(Json(products.into_iter().map(Into::into).collect()))
}

async fn get_product(
    State(repository): State<ProductRepository>,
    Path(id): Path<i64>,
) -> AppResult<Json<ProductDetailsResponse>> {
    let product = repository.find(id).await?.ok_or(AppError::NotFound)?;

    Ok(Json(product.into()))
}

async fn update_product(
    State(repository): State<ProductRepository>,
    Path(id): Path<i64>,
    Json(payload): Json<ProductRequest>,
) -> AppResult<Json<ProductResponse>> {
    let product = repository
        .update(id, &payload.into())
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(product.into()))
}

async fn delete_product(
    State(repository): State<ProductRepository>,
    Path(id): Path<i64>,
) -> AppResult<StatusCode> {
    repository.delete(id).await?.ok_or(AppError::NotFound)?;

    Ok(StatusCode::NO_CONTENT)
}
