use cornucopia::queries::images as queries;
use deadpool_postgres::{Object, Pool};
use shared::{AppError, AppResult};

use crate::domain::{Image, ImageInput};

#[derive(Clone)]
pub struct ImageRepository {
    db: Pool,
}

impl ImageRepository {
    pub fn new(db: Pool) -> Self {
        Self { db }
    }

    async fn client(&self) -> AppResult<Object> {
        self.db.get().await.map_err(AppError::internal)
    }

    pub async fn create(&self, product_id: i64, input: &ImageInput) -> AppResult<Image> {
        let mut client = self.client().await?;
        let transaction = client.transaction().await.map_err(AppError::database)?;

        if input.is_primary {
            queries::clear_primary_image()
                .bind(&transaction, &product_id)
                .await
                .map_err(AppError::database)?;
        }

        let image = queries::create_image()
            .bind(
                &transaction,
                &product_id,
                &input.file_path,
                &input.is_primary,
                &input.display_order,
            )
            .map(|row| Image {
                id: row.id,
                product_id: row.product_id,
                file_path: row.file_path.into(),
                is_primary: row.is_primary,
                display_order: row.display_order,
            })
            .one()
            .await
            .map_err(AppError::database)?;

        transaction.commit().await.map_err(AppError::database)?;

        Ok(image)
    }

    pub async fn list_for_product(&self, product_id: i64) -> AppResult<Vec<Image>> {
        let client = self.client().await?;

        queries::get_product_images()
            .bind(&client, &product_id)
            .map(|row| Image {
                id: row.id,
                product_id: row.product_id,
                file_path: row.file_path.into(),
                is_primary: row.is_primary,
                display_order: row.display_order,
            })
            .all()
            .await
            .map_err(AppError::internal)
    }

    pub async fn find(&self, id: i64) -> AppResult<Option<Image>> {
        let client = self.client().await?;

        queries::get_image()
            .bind(&client, &id)
            .map(|row| Image {
                id: row.id,
                product_id: row.product_id,
                file_path: row.file_path.into(),
                is_primary: row.is_primary,
                display_order: row.display_order,
            })
            .opt()
            .await
            .map_err(AppError::internal)
    }

    pub async fn update(&self, id: i64, input: &ImageInput) -> AppResult<Option<Image>> {
        let mut client = self.client().await?;
        let transaction = client.transaction().await.map_err(AppError::database)?;

        let Some(existing) = queries::get_image()
            .bind(&transaction, &id)
            .opt()
            .await
            .map_err(AppError::database)?
        else {
            return Ok(None);
        };

        if input.is_primary {
            queries::clear_primary_image()
                .bind(&transaction, &existing.product_id)
                .await
                .map_err(AppError::database)?;
        }

        let image = queries::update_image()
            .bind(
                &transaction,
                &input.file_path,
                &input.is_primary,
                &input.display_order,
                &id,
            )
            .map(|row| Image {
                id: row.id,
                product_id: row.product_id,
                file_path: row.file_path.into(),
                is_primary: row.is_primary,
                display_order: row.display_order,
            })
            .opt()
            .await
            .map_err(AppError::database)?;

        transaction.commit().await.map_err(AppError::database)?;

        Ok(image)
    }

    pub async fn delete(&self, id: i64) -> AppResult<Option<i64>> {
        let client = self.client().await?;

        queries::delete_image()
            .bind(&client, &id)
            .opt()
            .await
            .map_err(AppError::database)
    }
}
