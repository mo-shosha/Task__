// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct CreateUserParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub username: T1,
    pub email: T2,
    pub password_hash: T3,
}
#[derive(Debug)]
pub struct CreateRefreshTokenParams<T1: crate::StringSql> {
    pub user_id: uuid::Uuid,
    pub token_hash: T1,
    pub expires_at: chrono::DateTime<chrono::FixedOffset>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CreateUser {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct CreateUserBorrowed<'a> {
    pub id: uuid::Uuid,
    pub username: &'a str,
    pub email: &'a str,
    pub role: &'a str,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<CreateUserBorrowed<'a>> for CreateUser {
    fn from(
        CreateUserBorrowed {
            id,
            username,
            email,
            role,
            is_active,
            created_at,
        }: CreateUserBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            username: username.into(),
            email: email.into(),
            role: role.into(),
            is_active,
            created_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetUserByEmail {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetUserByEmailBorrowed<'a> {
    pub id: uuid::Uuid,
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
    pub role: &'a str,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetUserByEmailBorrowed<'a>> for GetUserByEmail {
    fn from(
        GetUserByEmailBorrowed {
            id,
            username,
            email,
            password_hash,
            role,
            is_active,
            created_at,
        }: GetUserByEmailBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            username: username.into(),
            email: email.into(),
            password_hash: password_hash.into(),
            role: role.into(),
            is_active,
            created_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetUserById {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetUserByIdBorrowed<'a> {
    pub id: uuid::Uuid,
    pub username: &'a str,
    pub email: &'a str,
    pub role: &'a str,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetUserByIdBorrowed<'a>> for GetUserById {
    fn from(
        GetUserByIdBorrowed {
            id,
            username,
            email,
            role,
            is_active,
            created_at,
        }: GetUserByIdBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            username: username.into(),
            email: email.into(),
            role: role.into(),
            is_active,
            created_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetUserByUsername {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetUserByUsernameBorrowed<'a> {
    pub id: uuid::Uuid,
    pub username: &'a str,
    pub email: &'a str,
    pub role: &'a str,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetUserByUsernameBorrowed<'a>> for GetUserByUsername {
    fn from(
        GetUserByUsernameBorrowed {
            id,
            username,
            email,
            role,
            is_active,
            created_at,
        }: GetUserByUsernameBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            username: username.into(),
            email: email.into(),
            role: role.into(),
            is_active,
            created_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CreateRefreshToken {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub expires_at: chrono::DateTime<chrono::FixedOffset>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetRefreshToken {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub token_hash: String,
    pub expires_at: chrono::DateTime<chrono::FixedOffset>,
    pub revoked_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetRefreshTokenBorrowed<'a> {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub token_hash: &'a str,
    pub expires_at: chrono::DateTime<chrono::FixedOffset>,
    pub revoked_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetRefreshTokenBorrowed<'a>> for GetRefreshToken {
    fn from(
        GetRefreshTokenBorrowed {
            id,
            user_id,
            token_hash,
            expires_at,
            revoked_at,
            created_at,
        }: GetRefreshTokenBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            user_id,
            token_hash: token_hash.into(),
            expires_at,
            revoked_at,
            created_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct CreateUserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CreateUserBorrowed, tokio_postgres::Error>,
    mapper: fn(CreateUserBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CreateUserQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CreateUserBorrowed) -> R,
    ) -> CreateUserQuery<'c, 'a, 's, C, R, N> {
        CreateUserQuery {
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
pub struct GetUserByEmailQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetUserByEmailBorrowed, tokio_postgres::Error>,
    mapper: fn(GetUserByEmailBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetUserByEmailQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetUserByEmailBorrowed) -> R,
    ) -> GetUserByEmailQuery<'c, 'a, 's, C, R, N> {
        GetUserByEmailQuery {
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
pub struct GetUserByIdQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetUserByIdBorrowed, tokio_postgres::Error>,
    mapper: fn(GetUserByIdBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetUserByIdQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetUserByIdBorrowed) -> R,
    ) -> GetUserByIdQuery<'c, 'a, 's, C, R, N> {
        GetUserByIdQuery {
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
pub struct GetUserByUsernameQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetUserByUsernameBorrowed, tokio_postgres::Error>,
    mapper: fn(GetUserByUsernameBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetUserByUsernameQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetUserByUsernameBorrowed) -> R,
    ) -> GetUserByUsernameQuery<'c, 'a, 's, C, R, N> {
        GetUserByUsernameQuery {
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
pub struct CreateRefreshTokenQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CreateRefreshToken, tokio_postgres::Error>,
    mapper: fn(CreateRefreshToken) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CreateRefreshTokenQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CreateRefreshToken) -> R,
    ) -> CreateRefreshTokenQuery<'c, 'a, 's, C, R, N> {
        CreateRefreshTokenQuery {
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
pub struct GetRefreshTokenQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetRefreshTokenBorrowed, tokio_postgres::Error>,
    mapper: fn(GetRefreshTokenBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetRefreshTokenQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetRefreshTokenBorrowed) -> R,
    ) -> GetRefreshTokenQuery<'c, 'a, 's, C, R, N> {
        GetRefreshTokenQuery {
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
pub struct CreateUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_user() -> CreateUserStmt {
    CreateUserStmt(
        "WITH inserted AS ( INSERT INTO users ( username, email, password_hash, role_id ) VALUES ( $1, $2, $3, (SELECT id FROM roles WHERE name = 'customer') ) RETURNING id, username, email, role_id, is_active, created_at ) SELECT inserted.id, inserted.username, inserted.email, roles.name AS role, inserted.is_active, inserted.created_at FROM inserted JOIN roles ON roles.id = inserted.role_id",
        None,
    )
}
impl CreateUserStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        username: &'a T1,
        email: &'a T2,
        password_hash: &'a T3,
    ) -> CreateUserQuery<'c, 'a, 's, C, CreateUser, 3> {
        CreateUserQuery {
            client,
            params: [username, email, password_hash],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CreateUserBorrowed, tokio_postgres::Error> {
                    Ok(CreateUserBorrowed {
                        id: row.try_get(0)?,
                        username: row.try_get(1)?,
                        email: row.try_get(2)?,
                        role: row.try_get(3)?,
                        is_active: row.try_get(4)?,
                        created_at: row.try_get(5)?,
                    })
                },
            mapper: |it| CreateUser::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateUserParams<T1, T2, T3>,
        CreateUserQuery<'c, 'a, 's, C, CreateUser, 3>,
        C,
    > for CreateUserStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateUserParams<T1, T2, T3>,
    ) -> CreateUserQuery<'c, 'a, 's, C, CreateUser, 3> {
        self.bind(
            client,
            &params.username,
            &params.email,
            &params.password_hash,
        )
    }
}
pub struct GetUserByEmailStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_user_by_email() -> GetUserByEmailStmt {
    GetUserByEmailStmt(
        "SELECT users.id, users.username, users.email, users.password_hash, roles.name AS role, users.is_active, users.created_at FROM users JOIN roles ON roles.id = users.role_id WHERE users.email = $1",
        None,
    )
}
impl GetUserByEmailStmt {
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
        email: &'a T1,
    ) -> GetUserByEmailQuery<'c, 'a, 's, C, GetUserByEmail, 1> {
        GetUserByEmailQuery {
            client,
            params: [email],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<GetUserByEmailBorrowed, tokio_postgres::Error> {
                Ok(GetUserByEmailBorrowed {
                    id: row.try_get(0)?,
                    username: row.try_get(1)?,
                    email: row.try_get(2)?,
                    password_hash: row.try_get(3)?,
                    role: row.try_get(4)?,
                    is_active: row.try_get(5)?,
                    created_at: row.try_get(6)?,
                })
            },
            mapper: |it| GetUserByEmail::from(it),
        }
    }
}
pub struct GetUserByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_user_by_id() -> GetUserByIdStmt {
    GetUserByIdStmt(
        "SELECT users.id, users.username, users.email, roles.name AS role, users.is_active, users.created_at FROM users JOIN roles ON roles.id = users.role_id WHERE users.id = $1",
        None,
    )
}
impl GetUserByIdStmt {
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
        id: &'a uuid::Uuid,
    ) -> GetUserByIdQuery<'c, 'a, 's, C, GetUserById, 1> {
        GetUserByIdQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetUserByIdBorrowed, tokio_postgres::Error> {
                    Ok(GetUserByIdBorrowed {
                        id: row.try_get(0)?,
                        username: row.try_get(1)?,
                        email: row.try_get(2)?,
                        role: row.try_get(3)?,
                        is_active: row.try_get(4)?,
                        created_at: row.try_get(5)?,
                    })
                },
            mapper: |it| GetUserById::from(it),
        }
    }
}
pub struct GetUserByUsernameStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_user_by_username() -> GetUserByUsernameStmt {
    GetUserByUsernameStmt(
        "SELECT users.id, users.username, users.email, roles.name AS role, users.is_active, users.created_at FROM users JOIN roles ON roles.id = users.role_id WHERE users.username = $1",
        None,
    )
}
impl GetUserByUsernameStmt {
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
        username: &'a T1,
    ) -> GetUserByUsernameQuery<'c, 'a, 's, C, GetUserByUsername, 1> {
        GetUserByUsernameQuery {
            client,
            params: [username],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<GetUserByUsernameBorrowed, tokio_postgres::Error> {
                Ok(GetUserByUsernameBorrowed {
                    id: row.try_get(0)?,
                    username: row.try_get(1)?,
                    email: row.try_get(2)?,
                    role: row.try_get(3)?,
                    is_active: row.try_get(4)?,
                    created_at: row.try_get(5)?,
                })
            },
            mapper: |it| GetUserByUsername::from(it),
        }
    }
}
pub struct CreateRefreshTokenStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_refresh_token() -> CreateRefreshTokenStmt {
    CreateRefreshTokenStmt(
        "INSERT INTO refresh_tokens ( user_id, token_hash, expires_at ) VALUES ( $1, $2, $3 ) RETURNING id, user_id, expires_at, created_at",
        None,
    )
}
impl CreateRefreshTokenStmt {
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
        user_id: &'a uuid::Uuid,
        token_hash: &'a T1,
        expires_at: &'a chrono::DateTime<chrono::FixedOffset>,
    ) -> CreateRefreshTokenQuery<'c, 'a, 's, C, CreateRefreshToken, 3> {
        CreateRefreshTokenQuery {
            client,
            params: [user_id, token_hash, expires_at],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CreateRefreshToken, tokio_postgres::Error> {
                    Ok(CreateRefreshToken {
                        id: row.try_get(0)?,
                        user_id: row.try_get(1)?,
                        expires_at: row.try_get(2)?,
                        created_at: row.try_get(3)?,
                    })
                },
            mapper: |it| CreateRefreshToken::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateRefreshTokenParams<T1>,
        CreateRefreshTokenQuery<'c, 'a, 's, C, CreateRefreshToken, 3>,
        C,
    > for CreateRefreshTokenStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateRefreshTokenParams<T1>,
    ) -> CreateRefreshTokenQuery<'c, 'a, 's, C, CreateRefreshToken, 3> {
        self.bind(
            client,
            &params.user_id,
            &params.token_hash,
            &params.expires_at,
        )
    }
}
pub struct GetRefreshTokenStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_refresh_token() -> GetRefreshTokenStmt {
    GetRefreshTokenStmt(
        "SELECT id, user_id, token_hash, expires_at, revoked_at, created_at FROM refresh_tokens WHERE token_hash = $1",
        None,
    )
}
impl GetRefreshTokenStmt {
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
        token_hash: &'a T1,
    ) -> GetRefreshTokenQuery<'c, 'a, 's, C, GetRefreshToken, 1> {
        GetRefreshTokenQuery {
            client,
            params: [token_hash],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<GetRefreshTokenBorrowed, tokio_postgres::Error> {
                Ok(GetRefreshTokenBorrowed {
                    id: row.try_get(0)?,
                    user_id: row.try_get(1)?,
                    token_hash: row.try_get(2)?,
                    expires_at: row.try_get(3)?,
                    revoked_at: row.try_get(4)?,
                    created_at: row.try_get(5)?,
                })
            },
            mapper: |it| GetRefreshToken::from(it),
        }
    }
}
pub struct RevokeRefreshTokenStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn revoke_refresh_token() -> RevokeRefreshTokenStmt {
    RevokeRefreshTokenStmt(
        "UPDATE refresh_tokens SET revoked_at = NOW() WHERE token_hash = $1 AND revoked_at IS NULL",
        None,
    )
}
impl RevokeRefreshTokenStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        token_hash: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[token_hash]).await
    }
}
pub struct RevokeAllUserRefreshTokensStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn revoke_all_user_refresh_tokens() -> RevokeAllUserRefreshTokensStmt {
    RevokeAllUserRefreshTokensStmt(
        "UPDATE refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL",
        None,
    )
}
impl RevokeAllUserRefreshTokensStmt {
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
        user_id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[user_id]).await
    }
}
