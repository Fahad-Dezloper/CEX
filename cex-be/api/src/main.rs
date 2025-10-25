use poem::{listener::TcpListener, EndpointExt, Route, Server, middleware::Cors};

use crate::{redismanager::RedisManager, routes::{auth, depth, klines, markets, order, ticker, trades}};
use db::{establish_connection, fetch_enabled_markets};
mod routes {
    pub mod order;
    pub mod depth;
    pub mod trades;
    pub mod klines;
    pub mod ticker;
    pub mod auth;
    pub mod markets;
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

    dotenvy::dotenv().ok();

    let manager = RedisManager::new("redis://127.0.0.1/")
        .await
        .expect("failed to connect to Redis");
    // Preload markets from DB into Redis cache if missing
    if manager.get_cached_markets().await.map_err(|e| {
        log::error!("Failed to get cached markets in Redis: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to get cached markets in Redis")
    })?.is_none() {
        let pool = establish_connection();
        let markets = fetch_enabled_markets(&pool).expect("failed to fetch markets");
        let minimal: Vec<serde_json::Value> = markets.into_iter().map(|m| serde_json::json!({
            "base": m.base_asset,
            "quote": m.quote_asset,
            "symbol": m.symbol,
            "price_precision": m.price_precision,
            "quantity_precision": m.quantity_precision,
            "min_price": m.min_price,
            "max_price": m.max_price,
            "min_order_size": m.min_order_size,
            "max_order_size": m.max_order_size,
        })).collect();
        let json = serde_json::to_string(&minimal).expect("serialize markets");
        manager.cache_markets(&json).await.map_err(|e| {
            log::error!("Failed to cache markets in Redis: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to cache markets in Redis")
        })?;
        log::info!("Markets cached in Redis");
    }

    log::info!("Connected to Redis successfully");

    let cors = Cors::new()
        .allow_origin("http://localhost:3000")
        .allow_credentials(true)
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_headers(vec!["content-type", "authorization"]);


    let app = Route::new()
                    // Public routes (no authentication required)
                    .nest("/api/v1/auth", auth::auth_routes())
                    .nest("/api/v1/depth", depth::depth_routes())
                    .nest("/api/v1/trades", trades::trade_routes())
                    .nest("/api/v1/klines", klines::klines_routes())
                    .nest("/api/v1/tickers", ticker::ticker_routes())
                    .nest("/api/v1/markets", markets::markets_routes())
                    // Protected routes (authentication required)
                    .nest("/api/v1/order", order::order_routes())
                    .with(cors)
                    .data(manager);

    log::info!("API routes configured");
    log::info!("Server starting on 0.0.0.0:3010");

    Server::new(TcpListener::bind("0.0.0.0:3010"))
        .run(app)
        .await
}