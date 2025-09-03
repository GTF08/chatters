use axum::{middleware, routing::{get, post}, Router};

use crate::{appstate::AppState, 
    handlers::{
        auth::{self, logout_handler}, 
        token::refresh_access_token_handler
    }
};
use common::middlewares::auth_guard;

pub fn routes(appstate: AppState) -> Router {
    Router::new()
        .route("/register", post(auth::register_handler))
        .route("/login", post(auth::login_handler))
        .route("/refresh", get(refresh_access_token_handler))
        .route("/logout",
        get(logout_handler)
                .route_layer(middleware::from_fn_with_state(appstate.clone(), auth_guard)),
        )
        .with_state(appstate)
}