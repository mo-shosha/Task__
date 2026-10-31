// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct CreateImageParams<T1: crate::StringSql> {
    pub product_id: i64,
    pub file_path: T1,
    pub is_primary: bool,
    pub display_order: i32,
}
#[derive(Debug)]
pub struct UpdateImageParams<T1: crate::StringSql> {
    pub file_path: T1,
    pub is_primary: bool,
    pub display_order: i32,
    pub id: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CreateImage {
    pub id: i64,
    pub product_id: i64,
    pub file_path: String,
    pub is_primary: bool,
    pub display_order: i32,
}
pub struct CreateImageBorrowed<'a> {
    pub id: i64,
    pub product_id: i64,
    pub file_path: &'a str,
    pub is_primary: bool,
    pub display_order: i32,
}
impl<'a> From<CreateImageBorrowed<'a>> for CreateImage {
    fn from(
        CreateImageBorrowed {
            id,
            product_id,
            file_path,
            is_primary,
            display_order,
        }: CreateImageBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            product_id,
            file_path: file_path.into(),
            is_primary,
            display_order,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetProductImages {
    pub id: i64,
    pub product_id: i64,
    pub file_path: String,
    pub is_primary: bool,
    pub display_order: i32,
}
pub struct GetProductImagesBorrowed<'a> {
    pub id: i64,
    pub product_id: i64,
    pub file_path: &'a str,
    pub is_primary: bool,
    pub display_order: i32,
}
impl<'a> From<GetProductImagesBorrowed<'a>> for GetProductImages {
    fn from(
        GetProductImagesBorrowed {
            id,
            product_id,
            file_path,
            is_primary,
            display_order,
        }: GetProductImagesBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            product_id,
            file_path: file_path.into(),
            is_primary,
            display_order,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetImage {
    pub id: i64,
    pub product_id: i64,
    pub file_path: String,
    pub is_primary: bool,
    pub display_order: i32,
}
pub struct GetImageBorrowed<'a> {
    pub id: i64,
    pub product_id: i64,
    pub file_path: &'a str,
    pub is_primary: bool,
    pub display_order: i32,
}
impl<'a> From<GetImageBorrowed<'a>> for GetImage {
    fn from(
        GetImageBorrowed {
            id,
            product_id,
            file_path,
            is_primary,
            display_order,
        }: GetImageBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            product_id,
            file_path: file_path.into(),
            is_primary,
            display_order,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateImage {
    pub id: i64,
    pub product_id: i64,
    pub file_path: String,
    pub is_primary: bool,
    pub display_order: i32,
}
pub struct UpdateImageBorrowed<'a> {
    pub id: i64,
    pub product_id: i64,
    pub file_path: &'a str,
    pub is_primary: bool,
    pub display_order: i32,
}
impl<'a> From<UpdateImageBorrowed<'a>> for UpdateImage {
    fn from(
        UpdateImageBorrowed {
            id,
            product_id,
            file_path,
            is_primary,
            display_order,
        }: UpdateImageBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            product_id,
            file_path: file_path.into(),
            is_primary,
            display_order,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct CreateImageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CreateImageBorrowed, tokio_postgres::Error>,
    mapper: fn(CreateImageBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CreateImageQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CreateImageBorrowed) -> R,
    ) -> CreateImageQuery<'c, 'a, 's, C, R, N> {
        CreateImageQuery {
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
pub struct GetProductImagesQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetProductImagesBorrowed, tokio_postgres::Error>,
    mapper: fn(GetProductImagesBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetProductImagesQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetProductImagesBorrowed) -> R,
    ) -> GetProductImagesQuery<'c, 'a, 's, C, R, N> {
        GetProductImagesQuery {
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
pub struct GetImageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetImageBorrowed, tokio_postgres::Error>,
    mapper: fn(GetImageBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetImageQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(GetImageBorrowed) -> R) -> GetImageQuery<'c, 'a, 's, C, R, N> {
        GetImageQuery {
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
pub struct UpdateImageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UpdateImageBorrowed, tokio_postgres::Error>,
    mapper: fn(UpdateImageBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UpdateImageQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(UpdateImageBorrowed) -> R,
    ) -> UpdateImageQuery<'c, 'a, 's, C, R, N> {
        UpdateImageQuery {
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
pub struct CreateImageStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_image() -> CreateImageStmt {
    CreateImageStmt(
        "INSERT INTO images (product_id, file_path, is_primary, display_order) VALUES ($1, $2, $3, $4) RETURNING id, product_id, file_path, is_primary, display_order",
        None,
    )
}
impl CreateImageStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        product_id: &'a i64,
        file_path: &'a T1,
        is_primary: &'a bool,
        display_order: &'a i32,
    ) -> CreateImageQuery<'c, 'a, 's, C, CreateImage, 4> {
        CreateImageQuery {
            client,
            params: [product_id, file_path, is_primary, display_order],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CreateImageBorrowed, tokio_postgres::Error> {
                    Ok(CreateImageBorrowed {
                        id: row.try_get(0)?,
                        product_id: row.try_get(1)?,
                        file_path: row.try_get(2)?,
                        is_primary: row.try_get(3)?,
                        display_order: row.try_get(4)?,
                    })
                },
            mapper: |it| CreateImage::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateImageParams<T1>,
        CreateImageQuery<'c, 'a, 's, C, CreateImage, 4>,
        C,
    > for CreateImageStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateImageParams<T1>,
    ) -> CreateImageQuery<'c, 'a, 's, C, CreateImage, 4> {
        self.bind(
            client,
            &params.product_id,
            &params.file_path,
            &params.is_primary,
            &params.display_order,
        )
    }
}
pub struct GetProductImagesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_product_images() -> GetProductImagesStmt {
    GetProductImagesStmt(
        "SELECT id, product_id, file_path, is_primary, display_order FROM images WHERE product_id = $1 ORDER BY display_order, id",
        None,
    )
}
impl GetProductImagesStmt {
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
        product_id: &'a i64,
    ) -> GetProductImagesQuery<'c, 'a, 's, C, GetProductImages, 1> {
        GetProductImagesQuery {
            client,
            params: [product_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<GetProductImagesBorrowed, tokio_postgres::Error> {
                Ok(GetProductImagesBorrowed {
                    id: row.try_get(0)?,
                    product_id: row.try_get(1)?,
                    file_path: row.try_get(2)?,
                    is_primary: row.try_get(3)?,
                    display_order: row.try_get(4)?,
                })
            },
            mapper: |it| GetProductImages::from(it),
        }
    }
}
pub struct GetImageStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_image() -> GetImageStmt {
    GetImageStmt(
        "SELECT id, product_id, file_path, is_primary, display_order FROM images WHERE id = $1",
        None,
    )
}
impl GetImageStmt {
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
    ) -> GetImageQuery<'c, 'a, 's, C, GetImage, 1> {
        GetImageQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetImageBorrowed, tokio_postgres::Error> {
                    Ok(GetImageBorrowed {
                        id: row.try_get(0)?,
                        product_id: row.try_get(1)?,
                        file_path: row.try_get(2)?,
                        is_primary: row.try_get(3)?,
                        display_order: row.try_get(4)?,
                    })
                },
            mapper: |it| GetImage::from(it),
        }
    }
}
pub struct UpdateImageStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_image() -> UpdateImageStmt {
    UpdateImageStmt(
        "UPDATE images SET file_path = $1, is_primary = $2, display_order = $3 WHERE id = $4 RETURNING id, product_id, file_path, is_primary, display_order",
        None,
    )
}
impl UpdateImageStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        file_path: &'a T1,
        is_primary: &'a bool,
        display_order: &'a i32,
        id: &'a i64,
    ) -> UpdateImageQuery<'c, 'a, 's, C, UpdateImage, 4> {
        UpdateImageQuery {
            client,
            params: [file_path, is_primary, display_order, id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<UpdateImageBorrowed, tokio_postgres::Error> {
                    Ok(UpdateImageBorrowed {
                        id: row.try_get(0)?,
                        product_id: row.try_get(1)?,
                        file_path: row.try_get(2)?,
                        is_primary: row.try_get(3)?,
                        display_order: row.try_get(4)?,
                    })
                },
            mapper: |it| UpdateImage::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpdateImageParams<T1>,
        UpdateImageQuery<'c, 'a, 's, C, UpdateImage, 4>,
        C,
    > for UpdateImageStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpdateImageParams<T1>,
    ) -> UpdateImageQuery<'c, 'a, 's, C, UpdateImage, 4> {
        self.bind(
            client,
            &params.file_path,
            &params.is_primary,
            &params.display_order,
            &params.id,
        )
    }
}
pub struct ClearPrimaryImageStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn clear_primary_image() -> ClearPrimaryImageStmt {
    ClearPrimaryImageStmt(
        "UPDATE images SET is_primary = FALSE WHERE product_id = $1 AND is_primary",
        None,
    )
}
impl ClearPrimaryImageStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        product_id: &'a i64,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[product_id]).await
    }
}
pub struct DeleteImageStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_image() -> DeleteImageStmt {
    DeleteImageStmt("DELETE FROM images WHERE id = $1 RETURNING id", None)
}
impl DeleteImageStmt {
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
