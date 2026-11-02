// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct CreateProductParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub name: T1,
    pub price: i64,
    pub description: T2,
    pub category_id: i64,
}
#[derive(Debug)]
pub struct GetProductsParams<T1: crate::StringSql> {
    pub search: Option<T1>,
    pub category_id: Option<i64>,
    pub min_price: Option<i64>,
    pub max_price: Option<i64>,
    pub limit: i64,
    pub offset: i64,
}
#[derive(Debug)]
pub struct CountProductsParams<T1: crate::StringSql> {
    pub search: Option<T1>,
    pub category_id: Option<i64>,
    pub min_price: Option<i64>,
    pub max_price: Option<i64>,
}
#[derive(Debug)]
pub struct UpdateProductParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub name: T1,
    pub price: i64,
    pub description: T2,
    pub category_id: i64,
    pub id: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CreateProduct {
    pub id: i64,
    pub name: String,
    pub price: i64,
    pub description: String,
    pub category_id: i64,
}
pub struct CreateProductBorrowed<'a> {
    pub id: i64,
    pub name: &'a str,
    pub price: i64,
    pub description: &'a str,
    pub category_id: i64,
}
impl<'a> From<CreateProductBorrowed<'a>> for CreateProduct {
    fn from(
        CreateProductBorrowed {
            id,
            name,
            price,
            description,
            category_id,
        }: CreateProductBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            price,
            description: description.into(),
            category_id,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetProducts {
    pub id: i64,
    pub name: String,
    pub price: i64,
    pub description: String,
    pub category_id: i64,
    pub category_name: String,
    pub primary_image_path: Option<String>,
}
pub struct GetProductsBorrowed<'a> {
    pub id: i64,
    pub name: &'a str,
    pub price: i64,
    pub description: &'a str,
    pub category_id: i64,
    pub category_name: &'a str,
    pub primary_image_path: Option<&'a str>,
}
impl<'a> From<GetProductsBorrowed<'a>> for GetProducts {
    fn from(
        GetProductsBorrowed {
            id,
            name,
            price,
            description,
            category_id,
            category_name,
            primary_image_path,
        }: GetProductsBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            price,
            description: description.into(),
            category_id,
            category_name: category_name.into(),
            primary_image_path: primary_image_path.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetProduct {
    pub id: i64,
    pub name: String,
    pub price: i64,
    pub description: String,
    pub category_id: i64,
    pub image_id: Option<i64>,
    pub file_path: Option<String>,
    pub is_primary: Option<bool>,
    pub display_order: Option<i32>,
}
pub struct GetProductBorrowed<'a> {
    pub id: i64,
    pub name: &'a str,
    pub price: i64,
    pub description: &'a str,
    pub category_id: i64,
    pub image_id: Option<i64>,
    pub file_path: Option<&'a str>,
    pub is_primary: Option<bool>,
    pub display_order: Option<i32>,
}
impl<'a> From<GetProductBorrowed<'a>> for GetProduct {
    fn from(
        GetProductBorrowed {
            id,
            name,
            price,
            description,
            category_id,
            image_id,
            file_path,
            is_primary,
            display_order,
        }: GetProductBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            price,
            description: description.into(),
            category_id,
            image_id,
            file_path: file_path.map(|v| v.into()),
            is_primary,
            display_order,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateProduct {
    pub id: i64,
    pub name: String,
    pub price: i64,
    pub description: String,
    pub category_id: i64,
}
pub struct UpdateProductBorrowed<'a> {
    pub id: i64,
    pub name: &'a str,
    pub price: i64,
    pub description: &'a str,
    pub category_id: i64,
}
impl<'a> From<UpdateProductBorrowed<'a>> for UpdateProduct {
    fn from(
        UpdateProductBorrowed {
            id,
            name,
            price,
            description,
            category_id,
        }: UpdateProductBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            price,
            description: description.into(),
            category_id,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct CreateProductQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CreateProductBorrowed, tokio_postgres::Error>,
    mapper: fn(CreateProductBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CreateProductQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CreateProductBorrowed) -> R,
    ) -> CreateProductQuery<'c, 'a, 's, C, R, N> {
        CreateProductQuery {
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
pub struct GetProductsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetProductsBorrowed, tokio_postgres::Error>,
    mapper: fn(GetProductsBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetProductsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetProductsBorrowed) -> R,
    ) -> GetProductsQuery<'c, 'a, 's, C, R, N> {
        GetProductsQuery {
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
pub struct GetProductQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetProductBorrowed, tokio_postgres::Error>,
    mapper: fn(GetProductBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetProductQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetProductBorrowed) -> R,
    ) -> GetProductQuery<'c, 'a, 's, C, R, N> {
        GetProductQuery {
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
pub struct UpdateProductQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UpdateProductBorrowed, tokio_postgres::Error>,
    mapper: fn(UpdateProductBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UpdateProductQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(UpdateProductBorrowed) -> R,
    ) -> UpdateProductQuery<'c, 'a, 's, C, R, N> {
        UpdateProductQuery {
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
pub struct CreateProductStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_product() -> CreateProductStmt {
    CreateProductStmt(
        "INSERT INTO products (name, price, description, category_id) VALUES ($1, $2, $3, $4) RETURNING id, name, price, description, category_id",
        None,
    )
}
impl CreateProductStmt {
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
        price: &'a i64,
        description: &'a T2,
        category_id: &'a i64,
    ) -> CreateProductQuery<'c, 'a, 's, C, CreateProduct, 4> {
        CreateProductQuery {
            client,
            params: [name, price, description, category_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CreateProductBorrowed, tokio_postgres::Error> {
                    Ok(CreateProductBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        price: row.try_get(2)?,
                        description: row.try_get(3)?,
                        category_id: row.try_get(4)?,
                    })
                },
            mapper: |it| CreateProduct::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateProductParams<T1, T2>,
        CreateProductQuery<'c, 'a, 's, C, CreateProduct, 4>,
        C,
    > for CreateProductStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateProductParams<T1, T2>,
    ) -> CreateProductQuery<'c, 'a, 's, C, CreateProduct, 4> {
        self.bind(
            client,
            &params.name,
            &params.price,
            &params.description,
            &params.category_id,
        )
    }
}
pub struct GetProductsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_products() -> GetProductsStmt {
    GetProductsStmt(
        "SELECT p.id, p.name, p.price, p.description, p.category_id, c.name AS category_name, i.file_path AS primary_image_path FROM products p JOIN categories c ON p.category_id = c.id LEFT JOIN images i ON p.id = i.product_id AND i.is_primary = TRUE WHERE ($1::TEXT IS NULL OR p.name ILIKE '%' || $1 || '%' OR p.description ILIKE '%' || $1 || '%') AND ($2::BIGINT IS NULL OR p.category_id = $2) AND ($3::BIGINT IS NULL OR p.price >= $3) AND ($4::BIGINT IS NULL OR p.price <= $4) ORDER BY p.id LIMIT $5 OFFSET $6",
        None,
    )
}
impl GetProductsStmt {
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
        search: &'a Option<T1>,
        category_id: &'a Option<i64>,
        min_price: &'a Option<i64>,
        max_price: &'a Option<i64>,
        limit: &'a i64,
        offset: &'a i64,
    ) -> GetProductsQuery<'c, 'a, 's, C, GetProducts, 6> {
        GetProductsQuery {
            client,
            params: [search, category_id, min_price, max_price, limit, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetProductsBorrowed, tokio_postgres::Error> {
                    Ok(GetProductsBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        price: row.try_get(2)?,
                        description: row.try_get(3)?,
                        category_id: row.try_get(4)?,
                        category_name: row.try_get(5)?,
                        primary_image_path: row.try_get(6)?,
                    })
                },
            mapper: |it| GetProducts::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetProductsParams<T1>,
        GetProductsQuery<'c, 'a, 's, C, GetProducts, 6>,
        C,
    > for GetProductsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetProductsParams<T1>,
    ) -> GetProductsQuery<'c, 'a, 's, C, GetProducts, 6> {
        self.bind(
            client,
            &params.search,
            &params.category_id,
            &params.min_price,
            &params.max_price,
            &params.limit,
            &params.offset,
        )
    }
}
pub struct CountProductsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn count_products() -> CountProductsStmt {
    CountProductsStmt(
        "SELECT COUNT(*) AS total FROM products p WHERE ($1::TEXT IS NULL OR p.name ILIKE '%' || $1 || '%' OR p.description ILIKE '%' || $1 || '%') AND ($2::BIGINT IS NULL OR p.category_id = $2) AND ($3::BIGINT IS NULL OR p.price >= $3) AND ($4::BIGINT IS NULL OR p.price <= $4)",
        None,
    )
}
impl CountProductsStmt {
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
        search: &'a Option<T1>,
        category_id: &'a Option<i64>,
        min_price: &'a Option<i64>,
        max_price: &'a Option<i64>,
    ) -> I64Query<'c, 'a, 's, C, i64, 4> {
        I64Query {
            client,
            params: [search, category_id, min_price, max_price],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CountProductsParams<T1>,
        I64Query<'c, 'a, 's, C, i64, 4>,
        C,
    > for CountProductsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CountProductsParams<T1>,
    ) -> I64Query<'c, 'a, 's, C, i64, 4> {
        self.bind(
            client,
            &params.search,
            &params.category_id,
            &params.min_price,
            &params.max_price,
        )
    }
}
pub struct GetProductStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_product() -> GetProductStmt {
    GetProductStmt(
        "SELECT p.id, p.name, p.price, p.description, p.category_id, i.id AS image_id, i.file_path, i.is_primary, i.display_order FROM products p LEFT JOIN images i ON p.id = i.product_id WHERE p.id = $1 ORDER BY i.display_order, i.id",
        None,
    )
}
impl GetProductStmt {
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
    ) -> GetProductQuery<'c, 'a, 's, C, GetProduct, 1> {
        GetProductQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetProductBorrowed, tokio_postgres::Error> {
                    Ok(GetProductBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        price: row.try_get(2)?,
                        description: row.try_get(3)?,
                        category_id: row.try_get(4)?,
                        image_id: row.try_get(5)?,
                        file_path: row.try_get(6)?,
                        is_primary: row.try_get(7)?,
                        display_order: row.try_get(8)?,
                    })
                },
            mapper: |it| GetProduct::from(it),
        }
    }
}
pub struct UpdateProductStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_product() -> UpdateProductStmt {
    UpdateProductStmt(
        "UPDATE products SET name = $1, price = $2, description = $3, category_id = $4 WHERE id = $5 RETURNING id, name, price, description, category_id",
        None,
    )
}
impl UpdateProductStmt {
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
        price: &'a i64,
        description: &'a T2,
        category_id: &'a i64,
        id: &'a i64,
    ) -> UpdateProductQuery<'c, 'a, 's, C, UpdateProduct, 5> {
        UpdateProductQuery {
            client,
            params: [name, price, description, category_id, id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<UpdateProductBorrowed, tokio_postgres::Error> {
                    Ok(UpdateProductBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        price: row.try_get(2)?,
                        description: row.try_get(3)?,
                        category_id: row.try_get(4)?,
                    })
                },
            mapper: |it| UpdateProduct::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpdateProductParams<T1, T2>,
        UpdateProductQuery<'c, 'a, 's, C, UpdateProduct, 5>,
        C,
    > for UpdateProductStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpdateProductParams<T1, T2>,
    ) -> UpdateProductQuery<'c, 'a, 's, C, UpdateProduct, 5> {
        self.bind(
            client,
            &params.name,
            &params.price,
            &params.description,
            &params.category_id,
            &params.id,
        )
    }
}
pub struct DeleteProductStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_product() -> DeleteProductStmt {
    DeleteProductStmt("DELETE FROM products WHERE id = $1 RETURNING id", None)
}
impl DeleteProductStmt {
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
