use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use shared::{AppError, AppResult};

use crate::{
    domain::{
        Pagination, Product, ProductDetails, ProductFilter, ProductImage, ProductInput,
        ProductListItem,
    },
    repository::ProductRepository,
};

const DEFAULT_PAGE_SIZE: i64 = 20;
const MAX_PAGE_SIZE: i64 = 100;

pub fn router(repository: ProductRepository) -> Router {
    Router::new()
        .route("/products", post(create_product).get(get_products))
        .route(
            "/products/{id}",
            get(get_product).put(update_product).delete(delete_product),
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

#[derive(Deserialize)]
struct ProductQuery {
    search: Option<String>,
    category_id: Option<i64>,
    min_price: Option<i64>,
    max_price: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Serialize)]
struct ProductPageResponse {
    items: Vec<ProductListItemResponse>,
    page: i64,
    page_size: i64,
    total_items: i64,
    total_pages: i64,
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
    Query(query): Query<ProductQuery>,
) -> AppResult<Json<ProductPageResponse>> {
    let pagination = build_pagination(&query)?;
    let filter = build_filter(query)?;

    let page = repository.list(&filter, &pagination).await?;

    let total_pages = (page.total_items + pagination.page_size - 1) / pagination.page_size;

    Ok(Json(ProductPageResponse {
        items: page.items.into_iter().map(Into::into).collect(),
        page: pagination.page,
        page_size: pagination.page_size,
        total_items: page.total_items,
        total_pages,
    }))
}

fn build_pagination(query: &ProductQuery) -> AppResult<Pagination> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);

    if page < 1 {
        return Err(AppError::Validation(
            "page must be 1 or greater".to_string(),
        ));
    }

    if page_size < 1 {
        return Err(AppError::Validation(
            "page_size must be 1 or greater".to_string(),
        ));
    }

    if page_size > MAX_PAGE_SIZE {
        return Err(AppError::Validation(format!(
            "page_size cannot be greater than {MAX_PAGE_SIZE}"
        )));
    }

    Ok(Pagination { page, page_size })
}

fn build_filter(query: ProductQuery) -> AppResult<ProductFilter> {
    let mut search = None;
    if let Some(text) = query.search {
        let text = text.trim();
        if !text.is_empty() {
            search = Some(text.to_string());
        }
    }

    if let Some(min_price) = query.min_price
        && min_price < 0
    {
        return Err(AppError::Validation(
            "min_price cannot be negative".to_string(),
        ));
    }

    if let Some(max_price) = query.max_price
        && max_price < 0
    {
        return Err(AppError::Validation(
            "max_price cannot be negative".to_string(),
        ));
    }

    if let Some(min_price) = query.min_price
        && let Some(max_price) = query.max_price
        && min_price > max_price
    {
        return Err(AppError::Validation(
            "min_price cannot be greater than max_price".to_string(),
        ));
    }

    Ok(ProductFilter {
        search,
        category_id: query.category_id,
        min_price: query.min_price,
        max_price: query.max_price,
    })
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
