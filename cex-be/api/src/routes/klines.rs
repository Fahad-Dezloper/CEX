use std::sync::Arc;

use poem::{get, handler, web::{Data, Json, Query}, Route, Result, error::InternalServerError};
use serde_json::json;
use log::{info, warn, error};
use validator::Validate;

use crate::{redismanager::RedisManager, types::KlinesQuery};
use db::{establish_connection, trades};
use diesel::prelude::*;
use chrono::{NaiveDateTime, Duration};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct KlineData {
    pub open_time: i64,
    pub close_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub quote_volume: f64,
    pub trades: i32,
}

#[handler]
async fn get_klines(
    Data(_manager): Data<&Arc<RedisManager>>,
    Query(query): Query<KlinesQuery>
) -> Result<Json<serde_json::Value>> {
    info!("Getting klines for market: {}, interval: {}", query.market, query.interval);
    let valid_intervals = ["1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "1w", "1M"];
    if !valid_intervals.contains(&query.interval.as_str()) {
        warn!("Invalid interval: {}", query.interval);
        return Ok(Json(json!({
            "error": "Invalid interval. Supported intervals: 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d, 3d, 1w, 1M"
        })));
    }

    let start_time = query.start_time;
    let end_time = query.end_time;
    
    if start_time >= end_time {
        warn!("Invalid time range: start_time >= end_time");
        return Ok(Json(json!({
            "error": "Start time must be before end time"
        })));
    }

    let interval_minutes = get_interval_minutes(&query.interval);
    let max_duration_seconds = interval_minutes * 60 * 1000; // Convert to seconds
    if (end_time - start_time) > max_duration_seconds {
        warn!("Time range too large for interval: {}", query.interval);
        return Ok(Json(json!({
            "error": "Time range too large. Maximum 1000 klines allowed."
        })));
    }

    match fetch_klines_from_db(&query.market, &query.interval, start_time, end_time).await {
        Ok(klines) => {
            info!("Retrieved {} klines for market: {}", klines.len(), query.market);
            Ok(Json(json!({
                "success": true,
                "market": query.market,
                "interval": query.interval,
                "klines": klines
            })))
        }
        Err(e) => {
            error!("Failed to fetch klines: {}", e);
            Ok(Json(json!({
                "error": "Failed to fetch klines",
                "details": e.to_string()
            })))
        }
    }
}

async fn fetch_klines_from_db(
    market: &str,
    interval: &str,
    start_time: i64,
    end_time: i64,
) -> Result<Vec<KlineData>, Box<dyn std::error::Error>> {
    let pool = establish_connection();
    let mut conn = pool.get()?;

    let start_dt = time_to_naive_datetime(start_time);
    let end_dt = time_to_naive_datetime(end_time);

    let trades_data: Vec<(NaiveDateTime, String, String)> = trades::table
        .filter(trades::market.eq(market))
        .filter(trades::timestamp.ge(start_dt))
        .filter(trades::timestamp.le(end_dt))
        .order(trades::timestamp.asc())
        .select((trades::timestamp, trades::price, trades::quantity))
        .load(&mut conn)?;

    if trades_data.is_empty() {
        return Ok(vec![]);
    }

    let klines = generate_klines_from_trades(trades_data, interval, start_dt, end_dt);
    
    Ok(klines)
}

fn generate_klines_from_trades(
    trades: Vec<(NaiveDateTime, String, String)>,
    interval: &str,
    start_dt: NaiveDateTime,
    end_dt: NaiveDateTime,
) -> Vec<KlineData> {
    let interval_minutes = get_interval_minutes(interval);
    let mut klines = Vec::new();
    
    let mut current_time = start_dt;
    let mut current_kline: Option<KlineData> = None;

    for (timestamp, price_str, quantity_str) in trades {
        let price = price_str.parse::<f64>().unwrap_or(0.0);
        let quantity = quantity_str.parse::<f64>().unwrap_or(0.0);

        let kline_start = get_kline_start_time(timestamp, interval_minutes);
        
        if current_kline.is_none() || current_kline.as_ref().unwrap().open_time != kline_start.timestamp() {
            if let Some(kline) = current_kline.take() {
                klines.push(kline);
            }
            
            current_kline = Some(KlineData {
                open_time: kline_start.timestamp(),
                close_time: (kline_start + Duration::minutes(interval_minutes)).timestamp(),
                open: price,
                high: price,
                low: price,
                close: price,
                volume: quantity,
                quote_volume: price * quantity,
                trades: 1,
            });
        } else {
            if let Some(ref mut kline) = current_kline {
                kline.high = kline.high.max(price);
                kline.low = kline.low.min(price);
                kline.close = price;
                kline.volume += quantity;
                kline.quote_volume += price * quantity;
                kline.trades += 1;
            }
        }
    }

    if let Some(kline) = current_kline {
        klines.push(kline);
    }

    klines
}

fn get_interval_minutes(interval: &str) -> i64 {
    match interval {
        "1m" => 1,
        "3m" => 3,
        "5m" => 5,
        "15m" => 15,
        "30m" => 30,
        "1h" => 60,
        "2h" => 120,
        "4h" => 240,
        "6h" => 360,
        "8h" => 480,
        "12h" => 720,
        "1d" => 1440,
        "3d" => 4320,
        "1w" => 10080,
        "1M" => 43800, // Approximate month
        _ => 1,
    }
}

fn get_kline_start_time(timestamp: NaiveDateTime, interval_minutes: i64) -> NaiveDateTime {
    let total_minutes = timestamp.timestamp() / 60;
    let kline_minutes = (total_minutes / interval_minutes) * interval_minutes;
    NaiveDateTime::from_timestamp_opt(kline_minutes * 60, 0).unwrap_or(timestamp)
}

fn time_to_naive_datetime(timestamp: i64) -> NaiveDateTime {
    // Convert Unix timestamp to NaiveDateTime
    NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .unwrap_or_else(|| chrono::Utc::now().naive_utc())
}

pub fn klines_routes() -> Route {
    Route::new()
        .at("/", get(get_klines))
}