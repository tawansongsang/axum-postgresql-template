pub mod bmc;

// use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserInfo {
    pub user_info_id: Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub email_verified: PrimitiveDateTime,

    // -- pwd and token info
    pub password: String,
    pub password_salt: Uuid,
    pub token_salt: Uuid,
    pub role: String,
    pub create_by: Uuid,
    pub create_on: PrimitiveDateTime,
    pub active: String,
    pub update_by: Uuid,
    pub update_on: PrimitiveDateTime,
    pub deleted: String,
    pub delete_by: Uuid,
    pub delete_on: PrimitiveDateTime,
}

// #[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserInfoGet {
    pub user_info_id: Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub email_verified: PrimitiveDateTime,
    pub role: String,
    pub create_by: Uuid,
    pub create_on: PrimitiveDateTime,
    pub active: String,
    pub update_by: Uuid,
    pub update_on: PrimitiveDateTime,
    pub deleted: String,
    pub delete_by: Uuid,
    pub delete_on: PrimitiveDateTime,
}

// #[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct UserInfoForCreate {
    pub username: String,
    pub email: String,
    // pub email_verified: PrimitiveDateTime,
    pub name: String,
    pub password: String,
}

// #[allow(non_snake_case)]
#[derive(Debug, Deserialize, FromRow)]
pub struct UserInfoCreated {
    pub password_salt: Option<Uuid>,
}

// #[allow(non_snake_case)]
#[derive(Debug, Deserialize, FromRow)]
pub struct UserInfoForLogin {
    pub user_info_id: Uuid,
    pub username: String,
    pub name: String,
    pub password: String, // encrypted, #_scheme_id_#....
    pub password_salt: Uuid,
    pub token_salt: Uuid,
    pub role: String,
}

// #[allow(non_snake_case)]
#[derive(Debug, Deserialize, FromRow)]
pub struct UserInfoForAuth {
    pub user_info_id: Uuid,
    pub username: String,

    // -- token info
    pub token_salt: Uuid,
}

// #[allow(non_snake_case)]
#[derive(Debug, Deserialize, FromRow)]
pub struct UserInfoRecord {
    pub user_info_id: Uuid,
}

/// Marker Trait
pub trait UserInfoBy: for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserInfoBy for UserInfoForAuth {}
impl UserInfoBy for UserInfoGet {}
impl UserInfoBy for UserInfo {}
impl UserInfoBy for UserInfoForLogin {}
impl UserInfoBy for UserInfoRecord {}
impl UserInfoBy for UserInfoCreated {}
