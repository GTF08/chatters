use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

use appstate::AppState;
use axum::http::header::ACCEPT;
use axum::http::header::AUTHORIZATION;
use axum::http::header::CONTENT_TYPE;
use axum::http::HeaderValue;
use axum::http::Method;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use tower_http::cors::CorsLayer;

use common::appstate;
mod handlers;
mod routes;

use common::CONFIG;


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
        .nest("/api", routes::routes(dbstate))
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
            //.allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        
    );

    // run our app with hyper, listening globally on port 3000
    //let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    //let addr = SocketAddr::from(([127, 0, 0, 1], ports.https));
    axum_server::bind_rustls(SocketAddr::from_str("0.0.0.0:3000").unwrap(), config)
        .serve(app.into_make_service())
        .await
        .unwrap();
    //axum::serve(listener, app).await.unwrap();
}