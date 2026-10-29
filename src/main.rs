
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use deadpool_postgres::{Config, Pool, Runtime};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

#[derive(Clone)]
struct AppState {
    db: Pool,
}

#[derive(Deserialize)]
struct CreateCategoryRequest {
    name: String,
    description: Option<String>,
}

#[derive(Deserialize)]
struct UpdateCategoryRequest {
    name: String,
    description: Option<String>,
}

#[derive(Serialize)]
struct CategoryResponse {
    id: i64,
    name: String,
    description: String,
}

#[derive(Deserialize)]
struct CreateProductRequest {
    name: String,
    price: i64,
    description: Option<String>,
    category_id: i64,
}

#[derive(Deserialize)]
struct UpdateProductRequest {
    name: String,
    price: i64,
    description: Option<String>,
    category_id: i64,
}

#[derive(Serialize)]
struct ProductResponse {
    id: i64,
    name: String,
    price: i64,
    description: String,
    category_id: i64,
}

#[tokio::main]
async fn main() {
    let mut config = Config::new();

    config.host = Some("localhost".to_string());
    config.port = Some(5455);
    config.user = Some("postgres".to_string());
    config.password = Some("products_dev_password".to_string());
    config.dbname = Some("products_db".to_string());

    let pool = config
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Failed to create database pool");

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/", get(health_check))
        .route("/db-check", get(db_check))
        .route(
            "/categories",
            post(create_category).get(get_categories),
        )
        .route(
            "/categories/{id}",
            get(get_category)
                .put(update_category)
                .delete(delete_category),
        )
        .route(
            "/products",
            post(create_product).get(get_products),
        )
        .route(
            "/products/{id}",
            get(get_product)
                .put(update_product)
                .delete(delete_product),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind address");

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

async fn health_check() -> &'static str {
    "Products API is running"
}

async fn db_check(
    State(state): State<AppState>,
) -> Result<String, (StatusCode, String)> {
    let client = state.db.get().await.map_err(internal_error)?;

    let row = client
        .query_one("SELECT current_database()", &[])
        .await
        .map_err(internal_error)?;

    let database: String = row.get(0);

    Ok(format!("Connected to database: {database}"))
}


async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<(StatusCode, Json<CategoryResponse>), (StatusCode, String)> {
    let client = state.db.get().await.map_err(internal_error)?;
    let description = payload.description.unwrap_or_default();

    let category = cornucopia::queries::categories::create_category()
        .bind(&client, &payload.name, &description)
        .one()
        .await
        .map_err(database_error)?;

    Ok((
        StatusCode::CREATED,
        Json(CategoryResponse {
            id: category.id,
            name: category.name,
            description: category.description,
        }),
    ))
}

async fn get_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<CategoryResponse>>, (StatusCode, String)> {
    let client = state.db.get().await.map_err(internal_error)?;

    let categories = cornucopia::queries::categories::get_categories()
        .bind(&client)
        .all()
        .await
        .map_err(internal_error)?;

    let response = categories
        .into_iter()
        .map(|category| CategoryResponse {
            id: category.id,
            name: category.name,
            description: category.description,
        })
        .collect();

    Ok(Json(response))
}

async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<CategoryResponse>, (StatusCode, String)> {
    let client = state.db.get().await.map_err(internal_error)?;

    let category = cornucopia::queries::categories::get_category()
        .bind(&client, id)
        .opt()
        .await
        .map_err(internal_error)?
        .ok_or_else(not_found)?;

    Ok(Json(CategoryResponse {
        id: category.id,
        name: category.name,
        description: category.description,
    }))
}

async fn update_category(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<Json<CategoryResponse>, (StatusCode, String)> {
    let client = state.db.get().await.map_err(internal_error)?;
    let description = payload.description.unwrap_or_default();

    let category = cornucopia::queries::categories::update_category()
        .bind(&client, &payload.name, &description, id)
        .opt()
        .await
        .map_err(database_error)?
        .ok_or_else(not_found)?;

    Ok(Json(CategoryResponse {
        id: category.id,
        name: category.name,
        description: category.description,
    }))
}

async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    let client = state.db.get().await.map_err(internal_error)?;

    let deleted_id = cornucopia::queries::categories::delete_category()
        .bind(&client, id)
        .opt()
        .await
        .map_err(database_error)?
        .ok_or_else(not_found)?;

    let _ = deleted_id;

    Ok(StatusCode::NO_CONTENT)
}

// Products

async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<ProductResponse>), (StatusCode, String)> {
    let client = state.db.get().await.map_err(internal_error)?;
    let description = payload.description.unwrap_or_default();

    let product = cornucopia::queries::products::create_product()
        .bind(
            &client,
            &payload.name,
            payload.price,
            &description,
            payload.category_id,
        )
        .one()
        .await
        .map_err(database_error)?;

    Ok((
        StatusCode::CREATED,
        Json(ProductResponse {
            id: product.id,
            name: product.name,
            price: product.price,
            description: product.description,
            category_id: product.category_id,
        }),
    ))
}

async fn get_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductResponse>>, (StatusCode, String)> {
    let client = state.db.get().await.map_err(internal_error)?;

    let products = cornucopia::queries::products::get_products()
        .bind(&client)
        .all()
        .await
        .map_err(internal_error)?;

    let response = products
        .into_iter()
        .map(|product| ProductResponse {
            id: product.id,
            name: product.name,
            price: product.price,
            description: product.description,
            category_id: product.category_id,
        })
        .collect();

    Ok(Json(response))
}

async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ProductResponse>, (StatusCode, String)> {
    let client = state.db.get().await.map_err(internal_error)?;

    let product = cornucopia::queries::products::get_product()
        .bind(&client, id)
        .opt()
        .await
        .map_err(internal_error)?
        .ok_or_else(not_found)?;

    Ok(Json(ProductResponse {
        id: product.id,
        name: product.name,
        price: product.price,
        description: product.description,
        category_id: product.category_id,
    }))
}

async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateProductRequest>,
) -> Result<Json<ProductResponse>, (StatusCode, String)> {
    let client = state.db.get().await.map_err(internal_error)?;
    let description = payload.description.unwrap_or_default();

    let product = cornucopia::queries::products::update_product()
        .bind(
            &client,
            &payload.name,
            payload.price,
            &description,
            payload.category_id,
            id,
        )
        .opt()
        .await
        .map_err(database_error)?
        .ok_or_else(not_found)?;

    Ok(Json(ProductResponse {
        id: product.id,
        name: product.name,
        price: product.price,
        description: product.description,
        category_id: product.category_id,
    }))
}

async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    let client = state.db.get().await.map_err(internal_error)?;

    let deleted_id = cornucopia::queries::products::delete_product()
        .bind(&client, id)
        .opt()
        .await
        .map_err(database_error)?
        .ok_or_else(not_found)?;

    let _ = deleted_id;

    Ok(StatusCode::NO_CONTENT)
}


fn not_found() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, "Resource not found".to_string())
}

fn internal_error(error: impl std::fmt::Display,) -> (StatusCode, String) {
    eprintln!("Internal error: {error}");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "An internal server error occurred".to_string(),
    )
}

fn database_error(error: impl std::fmt::Display,) -> (StatusCode, String) {
    eprintln!("Database error: {error}");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "A database operation failed".to_string(),
    )
}
