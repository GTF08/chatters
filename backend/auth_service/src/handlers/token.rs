use axum::{
    extract::State, http::{
        header, HeaderMap, Response, StatusCode
    }, response::IntoResponse
};

use axum_extra::extract::{cookie::{Cookie, SameSite}, CookieJar};
use redis::AsyncCommands;
use serde_json::json;

use common::{
    crypto::token, CONFIG
};
use common::{appstate::AppState, models::users::Users};

use crate::handlers::auth::{generate_token, save_token_data_to_redis};

pub async fn refresh_access_token_handler(
    cookie_jar: CookieJar,
    State(appstate): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let message = "could not refresh access token";

    let refresh_token = cookie_jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| {
            // let error_response = serde_json::json!({
            //     "status": "fail",
            //     "message": message
            // });
            // (StatusCode::FORBIDDEN, Json(error_response))
            (StatusCode::FORBIDDEN, "Token is invalid or session has expired".to_string())
        })?;

    let refresh_token_details =
        match token::verify_jwt_token(CONFIG.refresh_token_public_key.to_owned(), &refresh_token)
        {
            Ok(token_details) => token_details,
            Err(e) => {
                // let error_response = serde_json::json!({
                //     "status": "fail",
                //     "message": format_args!("{:?}", e)
                // });
                // return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
                return Err((StatusCode::UNAUTHORIZED, "Failed to verify token, please log in again".to_string()))
            }
        };

    let mut redis_client = appstate
        .redis_client()
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| {
            // let error_response = serde_json::json!({
            //     "status": "error",
            //     "message": format!("Redis error: {}", e),
            // });
            // (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
        })?;

    let redis_token_user_id = redis_client
        .get::<_, String>(refresh_token_details.token_uuid.to_string())
        .await
        .map_err(|_| {
            // let error_response = serde_json::json!({
            //     "status": "error",
            //     "message": "Token is invalid or session has expired",
            // });
            // (StatusCode::UNAUTHORIZED, Json(error_response))
            (StatusCode::FORBIDDEN, "Token is invalid or session has expired".to_string())
        })?;

    let user_uuid = uuid::Uuid::parse_str(&redis_token_user_id).map_err(|_| {
        // let error_response = serde_json::json!({
        //     "status": "error",
        //     "message": "Token is invalid or session has expired",
        // });
        // (StatusCode::UNAUTHORIZED, Json(error_response))
        (StatusCode::FORBIDDEN, "Token is invalid or session has expired".to_string())
    })?;

    let user : Option<Users> = 
        Users::get_by_id(&appstate, user_uuid)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()))?;

    let user = user.ok_or_else(|| {
        // let error_response = serde_json::json!({
        //     "status": "fail",
        //     "message": "The user belonging to this token no longer exists".to_string(),
        // });
        // (StatusCode::UNAUTHORIZED, Json(error_response))
        (StatusCode::UNAUTHORIZED, "The user belonging to this token no longer exists".to_string())
    })?;

    let access_token_details = generate_token(
        user.user_id,
        CONFIG.access_token_max_age,
        CONFIG.access_token_private_key.to_owned(),
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()))?;

    save_token_data_to_redis(&appstate, &access_token_details, CONFIG.access_token_max_age)
    .await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()))?;

    let access_cookie = Cookie::build(
        ("access_token",
        access_token_details.token.clone().unwrap_or_default()),
    )
    .path("/")
    .max_age(time::Duration::minutes(CONFIG.access_token_max_age * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let logged_in_cookie = Cookie::build(("logged_in", "true"))
        .path("/")
        .max_age(time::Duration::minutes(CONFIG.access_token_max_age * 60))
        .same_site(SameSite::Lax)
        .http_only(false);

    let mut response = Response::new(
        json!({"status": "success", "access_token": access_token_details.token.unwrap()})
            .to_string(),
    );
    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        access_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        logged_in_cookie.to_string().parse().unwrap(),
    );

    response.headers_mut().extend(headers);
    Ok(response)
}