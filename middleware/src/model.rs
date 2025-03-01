use std::time::SystemTime;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::{Deserialize, Serialize};

use crate::config;
use crate::db::establish_connection;

#[derive(Queryable, Deserialize, Serialize, Clone, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub photo: Option<String>,
    pub verified: bool,
    pub provider: String,
    pub provider_id: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub photo: Option<String>,
    pub verified: bool,
    pub provider: String,
    pub provider_id: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
    pub env: config::Config,
}

impl AppState {
    pub async fn init() -> AppState {
        AppState {
            db_pool: establish_connection(),
            env: config::Config::init(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
    pub state: String,
}