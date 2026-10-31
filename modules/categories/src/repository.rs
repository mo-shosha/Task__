use cornucopia::queries::categories as queries;
use deadpool_postgres::{Object, Pool};
use shared::{AppError, AppResult};

use crate::domain::{Category, CategoryInput};

#[derive(Clone)]
pub struct CategoryRepository {
    db: Pool,
}

impl CategoryRepository {
    pub fn new(db: Pool) -> Self {
        Self { db }
    }

    async fn client(&self) -> AppResult<Object> {
        self.db.get().await.map_err(AppError::internal)
    }

    pub async fn create(&self, input: &CategoryInput) -> AppResult<Category> {
        let client = self.client().await?;

        queries::create_category()
            .bind(&client, &input.name, &input.description)
            .map(|row| Category {
                id: row.id,
                name: row.name.into(),
                description: row.description.into(),
            })
            .one()
            .await
            .map_err(AppError::database)
    }

    pub async fn list(&self) -> AppResult<Vec<Category>> {
        let client = self.client().await?;

        queries::get_categories()
            .bind(&client)
            .map(|row| Category {
                id: row.id,
                name: row.name.into(),
                description: row.description.into(),
            })
            .all()
            .await
            .map_err(AppError::internal)
    }

    pub async fn find(&self, id: i64) -> AppResult<Option<Category>> {
        let client = self.client().await?;

        queries::get_category()
            .bind(&client, &id)
            .map(|row| Category {
                id: row.id,
                name: row.name.into(),
                description: row.description.into(),
            })
            .opt()
            .await
            .map_err(AppError::internal)
    }

    pub async fn update(&self, id: i64, input: &CategoryInput) -> AppResult<Option<Category>> {
        let client = self.client().await?;

        queries::update_category()
            .bind(&client, &input.name, &input.description, &id)
            .map(|row| Category {
                id: row.id,
                name: row.name.into(),
                description: row.description.into(),
            })
            .opt()
            .await
            .map_err(AppError::database)
    }

    pub async fn delete(&self, id: i64) -> AppResult<Option<i64>> {
        let client = self.client().await?;

        queries::delete_category()
            .bind(&client, &id)
            .opt()
            .await
            .map_err(AppError::database)
    }
}
