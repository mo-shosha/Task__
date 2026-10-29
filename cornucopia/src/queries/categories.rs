// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct CreateCategoryParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub name: T1,
    pub description: T2,
}
#[derive(Debug)]
pub struct UpdateCategoryParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub name: T1,
    pub description: T2,
    pub id: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CreateCategory {
    pub id: i64,
    pub name: String,
    pub description: String,
}
pub struct CreateCategoryBorrowed<'a> {
    pub id: i64,
    pub name: &'a str,
    pub description: &'a str,
}
impl<'a> From<CreateCategoryBorrowed<'a>> for CreateCategory {
    fn from(
        CreateCategoryBorrowed {
            id,
            name,
            description,
        }: CreateCategoryBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetCategories {
    pub id: i64,
    pub name: String,
    pub description: String,
}
pub struct GetCategoriesBorrowed<'a> {
    pub id: i64,
    pub name: &'a str,
    pub description: &'a str,
}
impl<'a> From<GetCategoriesBorrowed<'a>> for GetCategories {
    fn from(
        GetCategoriesBorrowed {
            id,
            name,
            description,
        }: GetCategoriesBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetCategory {
    pub id: i64,
    pub name: String,
    pub description: String,
}
pub struct GetCategoryBorrowed<'a> {
    pub id: i64,
    pub name: &'a str,
    pub description: &'a str,
}
impl<'a> From<GetCategoryBorrowed<'a>> for GetCategory {
    fn from(
        GetCategoryBorrowed {
            id,
            name,
            description,
        }: GetCategoryBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateCategory {
    pub id: i64,
    pub name: String,
    pub description: String,
}
pub struct UpdateCategoryBorrowed<'a> {
    pub id: i64,
    pub name: &'a str,
    pub description: &'a str,
}
impl<'a> From<UpdateCategoryBorrowed<'a>> for UpdateCategory {
    fn from(
        UpdateCategoryBorrowed {
            id,
            name,
            description,
        }: UpdateCategoryBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct CreateCategoryQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CreateCategoryBorrowed, tokio_postgres::Error>,
    mapper: fn(CreateCategoryBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CreateCategoryQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CreateCategoryBorrowed) -> R,
    ) -> CreateCategoryQuery<'c, 'a, 's, C, R, N> {
        CreateCategoryQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct GetCategoriesQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetCategoriesBorrowed, tokio_postgres::Error>,
    mapper: fn(GetCategoriesBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetCategoriesQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetCategoriesBorrowed) -> R,
    ) -> GetCategoriesQuery<'c, 'a, 's, C, R, N> {
        GetCategoriesQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct GetCategoryQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetCategoryBorrowed, tokio_postgres::Error>,
    mapper: fn(GetCategoryBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetCategoryQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetCategoryBorrowed) -> R,
    ) -> GetCategoryQuery<'c, 'a, 's, C, R, N> {
        GetCategoryQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct UpdateCategoryQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UpdateCategoryBorrowed, tokio_postgres::Error>,
    mapper: fn(UpdateCategoryBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UpdateCategoryQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(UpdateCategoryBorrowed) -> R,
    ) -> UpdateCategoryQuery<'c, 'a, 's, C, R, N> {
        UpdateCategoryQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct I64Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<i64, tokio_postgres::Error>,
    mapper: fn(i64) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> I64Query<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(i64) -> R) -> I64Query<'c, 'a, 's, C, R, N> {
        I64Query {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct CreateCategoryStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_category() -> CreateCategoryStmt {
    CreateCategoryStmt(
        "INSERT INTO categories (name, description) VALUES ($1, $2) RETURNING id, name, description",
        None,
    )
}
impl CreateCategoryStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        name: &'a T1,
        description: &'a T2,
    ) -> CreateCategoryQuery<'c, 'a, 's, C, CreateCategory, 2> {
        CreateCategoryQuery {
            client,
            params: [name, description],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<CreateCategoryBorrowed, tokio_postgres::Error> {
                Ok(CreateCategoryBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    description: row.try_get(2)?,
                })
            },
            mapper: |it| CreateCategory::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateCategoryParams<T1, T2>,
        CreateCategoryQuery<'c, 'a, 's, C, CreateCategory, 2>,
        C,
    > for CreateCategoryStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateCategoryParams<T1, T2>,
    ) -> CreateCategoryQuery<'c, 'a, 's, C, CreateCategory, 2> {
        self.bind(client, &params.name, &params.description)
    }
}
pub struct GetCategoriesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_categories() -> GetCategoriesStmt {
    GetCategoriesStmt(
        "SELECT id, name, description FROM categories ORDER BY id",
        None,
    )
}
impl GetCategoriesStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> GetCategoriesQuery<'c, 'a, 's, C, GetCategories, 0> {
        GetCategoriesQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetCategoriesBorrowed, tokio_postgres::Error> {
                    Ok(GetCategoriesBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        description: row.try_get(2)?,
                    })
                },
            mapper: |it| GetCategories::from(it),
        }
    }
}
pub struct GetCategoryStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_category() -> GetCategoryStmt {
    GetCategoryStmt(
        "SELECT id, name, description FROM categories WHERE id = $1",
        None,
    )
}
impl GetCategoryStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        id: &'a i64,
    ) -> GetCategoryQuery<'c, 'a, 's, C, GetCategory, 1> {
        GetCategoryQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetCategoryBorrowed, tokio_postgres::Error> {
                    Ok(GetCategoryBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        description: row.try_get(2)?,
                    })
                },
            mapper: |it| GetCategory::from(it),
        }
    }
}
pub struct UpdateCategoryStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_category() -> UpdateCategoryStmt {
    UpdateCategoryStmt(
        "UPDATE categories SET name = $1, description = $2 WHERE id = $3 RETURNING id, name, description",
        None,
    )
}
impl UpdateCategoryStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        name: &'a T1,
        description: &'a T2,
        id: &'a i64,
    ) -> UpdateCategoryQuery<'c, 'a, 's, C, UpdateCategory, 3> {
        UpdateCategoryQuery {
            client,
            params: [name, description, id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<UpdateCategoryBorrowed, tokio_postgres::Error> {
                Ok(UpdateCategoryBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    description: row.try_get(2)?,
                })
            },
            mapper: |it| UpdateCategory::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpdateCategoryParams<T1, T2>,
        UpdateCategoryQuery<'c, 'a, 's, C, UpdateCategory, 3>,
        C,
    > for UpdateCategoryStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpdateCategoryParams<T1, T2>,
    ) -> UpdateCategoryQuery<'c, 'a, 's, C, UpdateCategory, 3> {
        self.bind(client, &params.name, &params.description, &params.id)
    }
}
pub struct DeleteCategoryStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_category() -> DeleteCategoryStmt {
    DeleteCategoryStmt("DELETE FROM categories WHERE id = $1 RETURNING id", None)
}
impl DeleteCategoryStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        id: &'a i64,
    ) -> I64Query<'c, 'a, 's, C, i64, 1> {
        I64Query {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
