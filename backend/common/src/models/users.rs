use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{Error, FromRow};
use uuid::Uuid;

use crate::appstate::AppState;
use shared::models::users::{FilteredUser, UserLoginVerifyDTO};

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct Users {
    pub user_id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Users {
    pub async fn exists(appstate: &AppState, email: &str) -> Result<bool, Error> {
        sqlx::query_scalar(
            "SELECT EXISTS ( SELECT 1 from users WHERE email = $1 )"
        )
        .bind(email)
        .fetch_one(&appstate.db())
        .await
    }

    pub async fn insert(appstate: &AppState, email: &str, username: &str, password_hash: &str) -> Result<Uuid, Error> {
        sqlx::query_scalar(
            "INSERT INTO users 
            (email, username, password) VALUES
            ($1, $2, $3)
            RETURNING user_id")
            .bind(email)
            .bind(username)
            .bind(password_hash)
            .fetch_one(&appstate.db())
            .await
    }

    pub async fn get_by_id(appstate: &AppState, id: Uuid) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM users WHERE user_id = $1")
        .bind(id)
        .fetch_optional(&appstate.db())
        .await
    }

    pub async fn get_filtered_by_id(appstate: &AppState, id : Uuid) -> Result<Option<FilteredUser>, Error> {
        sqlx::query_as(
            "SELECT user_id, email, username, created_at, updated_at FROM users
            WHERE user_id=$1")
            .bind(id)
            .fetch_optional(&appstate.db())
            .await
    }

    pub async fn get_login_verification_data(appstate: &AppState, email: &str) -> Result<Option<UserLoginVerifyDTO>, Error> {
        sqlx::query_as(
            "SELECT user_id, email, password FROM users
                WHERE LOWER(email)=LOWER($1)")
            .bind(email)
            .fetch_optional(&appstate.db())
            .await
    }
}