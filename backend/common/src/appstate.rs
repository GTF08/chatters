use std::{sync::Arc, time::Duration};

use axum::extract::ws::Message;
use redis::Client;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::{broadcast::{channel, Receiver, Sender}, Mutex};

#[derive(Clone)]
pub struct AppState {
    pool : PgPool,
    redis_client: Client,
    websocket_broadcast_tx: Arc<Mutex<Sender<Message>>>
}

impl AppState {
    pub async fn new(
        db_connection_url : &str,
        redis_url: &str
    ) -> Self {
        let options = PgPoolOptions::new()
            .min_connections(5)
            .max_connections(10)
            .idle_timeout(Duration::new(5, 0));

        let db_pool = match options.connect(db_connection_url).await {
            Ok(pool) => pool,
            Err(e) => panic!("Failed to connect to a database: {}", e)
        };

        let redis_client = match Client::open(redis_url) {
            Ok(client) => client,
            Err(e) => panic!("Failed to connect to Redis: {}", e)
        };

        let (tx, _rx) = channel::<Message>(100);

        Self {
            pool : db_pool,
            redis_client: redis_client,
            websocket_broadcast_tx : Arc::new(Mutex::new(tx))
        }
    }

    pub fn db(&self) -> PgPool {
        self.pool.clone()
    }

    pub fn redis_client(&self) -> Client {
        self.redis_client.clone()
    }

    pub fn get_broadcast_sender(&self) -> &Arc<Mutex<Sender<Message>>> {
        &self.websocket_broadcast_tx
    }
}