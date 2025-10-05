use poem::{listener::TcpListener, EndpointExt, Route, Server, middleware::Cors};

use crate::{redismanager::RedisManager, routes::{depth, klines, order, ticker, trades, auth}};
mod routes {
    pub mod order;
    pub mod depth;
    pub mod trades;
    pub mod klines;
    pub mod ticker;
    pub mod auth;
}
mod types;
mod redismanager;
mod auth_service;
mod middleware;
mod validation;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    log::info!("Starting CEX API server...");

    // Load environment variables
    dotenvy::dotenv().ok();

    let manager = RedisManager::new("redis://127.0.0.1/")
        .await
        .expect("failed to connect to Redis");

    log::info!("Connected to Redis successfully");

    let app = Route::new()
                    // Public routes (no authentication required)
                    .nest("/api/v1/auth", auth::auth_routes())
                    .nest("/api/v1/depth", depth::depth_routes())
                    .nest("/api/v1/trades", trades::trade_routes())
                    .nest("/api/v1/klines", klines::klines_routes())
                    .nest("/api/v1/tickers", ticker::ticker_routes())
                    // Protected routes (authentication required)
                    .nest("/api/v1/order", order::order_routes())
                    .with(Cors::new())
                    .data(manager);

    log::info!("API routes configured");
    log::info!("Server starting on 0.0.0.0:3000");

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}