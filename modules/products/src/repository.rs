use cornucopia::queries::products as queries;
use deadpool_postgres::{Object, Pool};
use shared::{AppError, AppResult};

use crate::domain::{
    Pagination, Product, ProductDetails, ProductFilter, ProductImage, ProductInput,
    ProductListItem, ProductPage,
};

#[derive(Clone)]
pub struct ProductRepository {
    db: Pool,
}

impl ProductRepository {
    pub fn new(db: Pool) -> Self {
        Self { db }
    }

    async fn client(&self) -> AppResult<Object> {
        self.db.get().await.map_err(AppError::internal)
    }

    pub async fn create(&self, input: &ProductInput) -> AppResult<Product> {
        let client = self.client().await?;

        queries::create_product()
            .bind(
                &client,
                &input.name,
                &input.price,
                &input.description,
                &input.category_id,
            )
            .map(|row| Product {
                id: row.id,
                name: row.name.into(),
                price: row.price,
                description: row.description.into(),
                category_id: row.category_id,
            })
            .one()
            .await
            .map_err(AppError::database)
    }

    pub async fn list(
        &self,
        filter: &ProductFilter,
        pagination: &Pagination,
    ) -> AppResult<ProductPage> {
        let client = self.client().await?;

        let total_items = queries::count_products()
            .bind(
                &client,
                &filter.search,
                &filter.category_id,
                &filter.min_price,
                &filter.max_price,
            )
            .one()
            .await
            .map_err(AppError::internal)?;

        let items = queries::get_products()
            .bind(
                &client,
                &filter.search,
                &filter.category_id,
                &filter.min_price,
                &filter.max_price,
                &pagination.page_size,
                &pagination.offset(),
            )
            .map(|row| ProductListItem {
                product: Product {
                    id: row.id,
                    name: row.name.into(),
                    price: row.price,
                    description: row.description.into(),
                    category_id: row.category_id,
                },
                category_name: row.category_name.into(),
                primary_image_path: row.primary_image_path.map(Into::into),
            })
            .all()
            .await
            .map_err(AppError::internal)?;

        Ok(ProductPage { items, total_items })
    }

    pub async fn find(&self, id: i64) -> AppResult<Option<ProductDetails>> {
        let client = self.client().await?;

        // One row per image; product columns repeat on every row.
        let rows = queries::get_product()
            .bind(&client, &id)
            .all()
            .await
            .map_err(AppError::internal)?;

        let Some(first) = rows.first() else {
            return Ok(None);
        };

        let product = Product {
            id: first.id,
            name: first.name.clone(),
            price: first.price,
            description: first.description.clone(),
            category_id: first.category_id,
        };

        let images = rows
            .into_iter()
            .filter_map(|row| {
                Some(ProductImage {
                    id: row.image_id?,
                    file_path: row.file_path?,
                    is_primary: row.is_primary?,
                    display_order: row.display_order?,
                })
            })
            .collect();

        Ok(Some(ProductDetails { product, images }))
    }

    pub async fn update(&self, id: i64, input: &ProductInput) -> AppResult<Option<Product>> {
        let client = self.client().await?;

        queries::update_product()
            .bind(
                &client,
                &input.name,
                &input.price,
                &input.description,
                &input.category_id,
                &id,
            )
            .map(|row| Product {
                id: row.id,
                name: row.name.into(),
                price: row.price,
                description: row.description.into(),
                category_id: row.category_id,
            })
            .opt()
            .await
            .map_err(AppError::database)
    }

    pub async fn delete(&self, id: i64) -> AppResult<Option<i64>> {
        let client = self.client().await?;

        queries::delete_product()
            .bind(&client, &id)
            .opt()
            .await
            .map_err(AppError::database)
    }
}
