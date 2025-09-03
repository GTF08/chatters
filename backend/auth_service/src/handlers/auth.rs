use axum::{extract::State, http::{header, HeaderMap, Response, StatusCode}, response::IntoResponse, Extension, Json};
use axum_extra::extract::{cookie::{Cookie, SameSite}, CookieJar};
use serde_json::json;
use uuid::Uuid;
use redis::AsyncCommands;


use common::{
    appstate::AppState, crypto::{
        hash::{hash_password, verify_password}, 
        token::{self, TokenDetails}}, 
        middlewares::CurrentUser, 
        models::users::Users, 
        CONFIG
};
use shared::models::users::{
    LoginUserSchema,
    RegisterUserSchema,
    UserLoginVerifyDTO
};

pub(crate) fn generate_token(
    user_id: uuid::Uuid,
    max_age: i64,
    private_key: String,
) -> Result<TokenDetails, StatusCode> {
    token::generate_jwt_token(user_id, max_age, private_key).map_err(|e| {
        // let error_response = serde_json::json!({
        //     "status": "error",
        //     "message": format!("error generating token: {}", e),
        // });
        //(StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub(crate) async fn save_token_data_to_redis(
    appstate: &AppState,
    token_details: &TokenDetails,
    max_age: i64,
) -> Result<(), StatusCode> {
    let mut redis_client = appstate
        .redis_client()
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| {
            // let error_response = serde_json::json!({
            //     "status": "error",
            //     "message": format!("Redis error: {}", e),
            // });
            //(StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    redis_client
        .set_ex(
            token_details.token_uuid.to_string(),
            token_details.user_id.to_string(),
            (max_age * 60) as u64,
        )
        .await
        .map_err(|e| {
            // let error_response = serde_json::json!({
            //     "status": "error",
            //     "message": format_args!("{}", e),
            // });
            // (StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
            StatusCode::UNPROCESSABLE_ENTITY
        })?;
    Ok(())
}


pub async fn register_handler(
    State(database_state): State<AppState>,
    Json(payload) : Json<RegisterUserSchema>
) -> Result<impl IntoResponse, impl IntoResponse> {
    let password_hash = hash_password(&payload.password)
                                .await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()))?;

    if Users::exists(&database_state, &payload.email)
        .await
        .map_err(|e| {(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())})? {
        return Err((StatusCode::BAD_REQUEST, "User already exists".to_string()));
    }
    
    let created_user_id: Uuid = Users::insert(
        &database_state, 
        &payload.email, 
        &payload.username, 
        &password_hash)
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
        })?;
    Ok(
        (StatusCode::CREATED, Json(created_user_id))
    )
}


pub async fn login_handler(
    State(database_state): State<AppState>,
    Json(payload) : Json<LoginUserSchema>
) -> Result<impl IntoResponse, impl IntoResponse> {

    let found_user : UserLoginVerifyDTO = 
        Users::get_login_verification_data(&database_state, &payload.email)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()))?
        .ok_or_else(|| {
            (StatusCode::NOT_FOUND, "User does not exist".to_string())
        })?;
    

    //QUIT EARLY IF NOT VALID
    if !verify_password(&payload.password, &found_user.password).await.is_ok() {
        return Err((StatusCode::BAD_REQUEST, "Invalid username or password".to_string()))
    }

    //TOKEN GEN

    let access_token_details = generate_token(
        found_user.user_id,
        CONFIG.access_token_max_age,
        CONFIG.access_token_private_key.to_owned(),
    )
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()))?;
    let refresh_token_details = generate_token(
        found_user.user_id,
        CONFIG.refresh_token_max_age,
        CONFIG.refresh_token_private_key.to_owned(),
    )
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()))?;

    save_token_data_to_redis(
        &database_state, 
        &access_token_details, 
        CONFIG.access_token_max_age)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()))?;

    save_token_data_to_redis(
        &database_state,
        &refresh_token_details,
        CONFIG.refresh_token_max_age,
    )
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()))?;

    let access_cookie = Cookie::build(
        ("access_token",
        access_token_details.token.clone().unwrap_or_default()),
    )
    .path("/")
    .max_age(time::Duration::minutes(CONFIG.access_token_max_age * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let refresh_cookie = Cookie::build(
        ("refresh_token",
        refresh_token_details.token.unwrap_or_default()),
    )
    .path("/")
    .max_age(time::Duration::minutes(CONFIG.refresh_token_max_age * 60))
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
        refresh_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        logged_in_cookie.to_string().parse().unwrap(),
    );

    response.headers_mut().extend(headers);
    Ok(response)

}



pub async fn logout_handler(
    cookie_jar: CookieJar,
    Extension(auth_guard): Extension<CurrentUser>,
    State(appstate): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {//Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let message = "Token is invalid or session has expired";

    let refresh_token = cookie_jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| {
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
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Redis error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
        })?;

    redis_client
        .del(&[
            refresh_token_details.token_uuid.to_string(),
            auth_guard.access_token_uuid.to_string(),
        ])
        .await
        .map_err(|e| {
            // let error_response = serde_json::json!({
            //     "status": "error",
            //     "message": format_args!("{:?}", e)
            // });
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
        })?;

    let access_cookie = Cookie::build(("access_token", ""))
        .path("/")
        .max_age(time::Duration::minutes(-1))
        .same_site(SameSite::Lax)
        .http_only(true);
    let refresh_cookie = Cookie::build(("refresh_token", ""))
        .path("/")
        .max_age(time::Duration::minutes(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let logged_in_cookie = Cookie::build(("logged_in", "false"))
        .path("/")
        .max_age(time::Duration::minutes(-1))
        .same_site(SameSite::Lax)
        .http_only(false);

    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        access_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        refresh_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        logged_in_cookie.to_string().parse().unwrap(),
    );

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response.headers_mut().extend(headers);
    Ok(response)
}