use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

use axum::http::header::{
    ACCEPT,
    AUTHORIZATION,
    CONTENT_TYPE
};
use axum::http::HeaderValue;
use axum::http::Method;
use axum_server::tls_rustls::RustlsConfig;
use tower_http::cors::CorsLayer;
mod handlers;
mod routes;

use appstate::AppState;
use axum::Router;

use common::appstate;
use common::CONFIG;

use routes::webrtc;


#[tokio::main]
async fn main() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install default CryptoProvider");
    // initialize tracing
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let dbstate = AppState::new(
        &CONFIG.database_url,
        &CONFIG.redis_url
    ).await;

    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await
    .unwrap();

    // build our application with a route
    let app = Router::new()
        .merge(webrtc::routes(dbstate))
        .layer(
            CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_origin(
                [
                    "https://localhost:8080".parse::<HeaderValue>().unwrap(),
                    "http://localhost:8080".parse::<HeaderValue>().unwrap(),
                    "https://192.168.0.3:8080".parse::<HeaderValue>().unwrap(),
                ]
            )
            .allow_credentials(true)
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        );

    // run our app with hyper, listening globally on port 3000
    //let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    //axum::serve(listener, app).await.unwrap();
    axum_server::bind_rustls(SocketAddr::from_str("0.0.0.0:3003").unwrap(), config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}