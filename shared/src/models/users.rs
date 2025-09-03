use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct UserLoginVerifyDTO {
    pub user_id: Uuid,
    pub email: String,
    pub password: String
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FilteredUser {
    pub user_id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUserSchema {
    pub email: String,
    pub username: String,
    pub password: String,
}