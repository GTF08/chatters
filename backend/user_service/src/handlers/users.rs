use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, Extension, Json};

use common::{appstate::AppState, middlewares::CurrentUser, models::users::Users};
use shared::models::users::FilteredUser;

pub async fn get_me_handler(
    Extension(current_user): Extension<CurrentUser>,
    State(appstate): State<AppState>
) -> Result<(StatusCode, Json<FilteredUser>), Response> {
    let current_user_data = Users::get_filtered_by_id(&appstate, current_user.id)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response())?
    .ok_or_else(|| (StatusCode::NOT_FOUND, "User not found").into_response())?;
    
    let json_response = serde_json::json!(
        current_user_data
    );

    Ok((StatusCode::OK, Json(current_user_data)))
}