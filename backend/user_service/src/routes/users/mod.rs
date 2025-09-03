use axum::{middleware, routing::{get, post}, Router};


use common::{appstate::AppState, middlewares::auth_guard};
use crate::handlers::users::get_me_handler;

pub fn routes(appstate: AppState) -> Router {
    Router::new()
        .route(
            "/me",
            get(get_me_handler)
                .route_layer(middleware::from_fn_with_state(appstate.clone(), auth_guard)),
        )
        .with_state(appstate)
}