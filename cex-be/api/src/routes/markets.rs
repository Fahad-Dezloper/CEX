use poem::{handler, get, Result, web::{Data, Json}, Route, Error};
use poem::http::StatusCode;
use std::sync::Arc;

use crate::redismanager::RedisManager;
use db::{establish_connection, fetch_enabled_markets};

#[handler]
async fn get_markets(Data(manager): Data<&Arc<RedisManager>>) -> Result<Json<serde_json::Value>> {
    if let Some(json) = manager.get_cached_markets().await.map_err(|e| {
        log::error!("Failed to get cached markets in Redis: {}", e);
        Error::from_string("Failed to get cached markets in Redis", StatusCode::INTERNAL_SERVER_ERROR)
    })? {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&json) {
            return Ok(Json(value));
        }
    }

    let pool = establish_connection();
    let markets = fetch_enabled_markets(&pool)
        .map_err(|e| {
            log::error!("Failed to fetch enabled markets from DB: {}", e);
            Error::from_string("DB error while fetching markets", StatusCode::INTERNAL_SERVER_ERROR)
        })?;

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

    let json_str = serde_json::to_string(&minimal).unwrap_or("[]".to_string());
    if let Err(e) = manager.cache_markets(&json_str).await {
        log::warn!("Could not refresh Redis markets cache: {}", e);
    }

    Ok(Json(serde_json::json!(minimal)))
}

pub fn markets_routes() -> Route {
    Route::new()
        .at("/", get(get_markets))
}