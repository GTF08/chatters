use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json, body::Body,
};

use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{appstate::AppState, crypto::token, CONFIG};
use redis::AsyncCommands;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentUser {
    pub id: Uuid,
    pub access_token_uuid: Uuid
}

pub async fn auth_guard(
    cookie_jar: CookieJar,
    State(appstate): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let access_token = cookie_jar
        .get("access_token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });
    
    let access_token = access_token.ok_or_else(|| {
        (StatusCode::UNAUTHORIZED, "You are not logged in, please provide a token".to_string())
    })?;

    let access_token_details =
        match token::verify_jwt_token(CONFIG.access_token_public_key.to_owned(), &access_token) {
            Ok(token_details) => token_details,
            Err(e) => {
                // let error_response = ErrorResponse {
                //     status: "fail",
                //     message: format!("{:?}", e),
                // };
                // return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
                return Err((StatusCode::UNAUTHORIZED, "Failed to verify token, please log in again".to_string()))
            }
        };

    let access_token_uuid = uuid::Uuid::parse_str(&access_token_details.token_uuid.to_string())
        .map_err(|_| {
            // let error_response = ErrorResponse {
            //     status: "fail",
            //     message: "Invalid token".to_string(),
            // };
            // (StatusCode::UNAUTHORIZED, Json(error_response))
            (StatusCode::FORBIDDEN, "Token is invalid or session has expired".to_string())
        })?;

    let mut redis_client = appstate
        .redis_client()
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| {
            // let error_response = ErrorResponse {
            //     status: "error",
            //     message: format!("Redis error: {}", e),
            // };
            // (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
        })?;

    let redis_token_user_id = redis_client
        .get::<_, String>(access_token_uuid.clone().to_string())
        .await
        .map_err(|_| {
            // let error_response = ErrorResponse {
            //     status: "error",
            //     message: "Token is invalid or session has expired".to_string(),
            // };
            // (StatusCode::UNAUTHORIZED, Json(error_response))
            (StatusCode::FORBIDDEN, "Token is invalid or session has expired".to_string())
        })?;

    let user_uuid = uuid::Uuid::parse_str(&redis_token_user_id).map_err(|_| {
        // let error_response = ErrorResponse {
        //     status: "fail",
        //     message: "Token is invalid or session has expired".to_string(),
        // };
        // (StatusCode::UNAUTHORIZED, Json(error_response))
        (StatusCode::FORBIDDEN, "Token is invalid or session has expired".to_string())
    })?;


    req.extensions_mut().insert(CurrentUser {
        id: user_uuid,
        access_token_uuid,
    });
    Ok(next.run(req).await)
}