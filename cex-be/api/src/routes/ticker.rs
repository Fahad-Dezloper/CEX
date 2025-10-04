use poem::{get, handler, web::{Json, Query}, Route, Result, error::InternalServerError};
use serde::{Serialize, Deserialize};
use db::{establish_connection, trades, orders};
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::expression_methods::ExpressionMethods;
use bigdecimal::BigDecimal;
use chrono::{Utc, Duration};

#[derive(Deserialize)]
pub struct TickerParams {
    pub market: Option<String>,
}

#[derive(Serialize)]
pub struct TickerResponse {
    pub market: String,
    pub last_price: String,
    pub price_change_24h: String,
    pub price_change_percent_24h: String,
    pub volume_24h: String,
    pub high_24h: String,
    pub low_24h: String,
    pub bid_price: Option<String>,
    pub ask_price: Option<String>,
    pub timestamp: i64,
}


#[handler]
async fn get_ticker(Query(params): Query<TickerParams>) -> Result<Json<TickerResponse>> {
    let market = params.market.unwrap_or_else(|| "BTCUSDT".to_string());
    let ticker = get_ticker_data(market).await?;
    Ok(Json(ticker))
}

#[handler]
async fn get_all_tickers() -> Result<Json<Vec<TickerResponse>>> {
    // For now, return a single ticker for BTCUSDT
    // In a real implementation, you'd query all markets
    let _params = TickerParams { market: Some("BTCUSDT".to_string()) };
    let ticker = get_ticker_data("BTCUSDT".to_string()).await?;
    Ok(Json(vec![ticker]))
}

async fn get_ticker_data(market: String) -> Result<TickerResponse> {
    let pool = establish_connection();
    let mut conn = pool.get()
        .map_err(|e| InternalServerError(e))?;

    // Calculate 24 hours ago timestamp
    let twenty_four_hours_ago = Utc::now() - Duration::hours(24);
    let now = Utc::now();

    // Get all trades from last 24 hours
    let recent_trades: Vec<(String, String)> = trades::table
        .filter(trades::market.eq(&market))
        .filter(trades::timestamp.ge(twenty_four_hours_ago.naive_utc()))
        .order(trades::timestamp.asc())
        .select((trades::price, trades::quote_quantity))
        .load(&mut conn)
        .map_err(|e| InternalServerError(e))?;

    let (last_price, high_24h, low_24h, volume_24h) = if !recent_trades.is_empty() {
        let prices: Vec<BigDecimal> = recent_trades.iter()
            .map(|(price, _)| price.parse::<BigDecimal>().unwrap_or_default())
            .collect();
        
        let volumes: Vec<BigDecimal> = recent_trades.iter()
            .map(|(_, volume)| volume.parse::<BigDecimal>().unwrap_or_default())
            .collect();

        let last_price = prices.last().unwrap_or(&BigDecimal::from(0)).to_string();
        let high_24h = prices.iter().max().unwrap_or(&BigDecimal::from(0)).to_string();
        let low_24h = prices.iter().min().unwrap_or(&BigDecimal::from(0)).to_string();
        let volume_24h = volumes.iter().sum::<BigDecimal>().to_string();

        (last_price, high_24h, low_24h, volume_24h)
    } else {
        ("0".to_string(), "0".to_string(), "0".to_string(), "0".to_string())
    };

    let best_bid = orders::table
        .filter(orders::market.eq(&market))
        .filter(orders::side.eq("BUY"))
        .order(orders::price.desc())
        .limit(1)
        .select(orders::price)
        .first::<String>(&mut conn)
        .optional()
        .map_err(|e| InternalServerError(e))?;

    let best_ask = orders::table
        .filter(orders::market.eq(&market))
        .filter(orders::side.eq("SELL"))
        .order(orders::price.asc())
        .limit(1)
        .select(orders::price)
        .first::<String>(&mut conn)
        .optional()
        .map_err(|e| InternalServerError(e))?;

    // For now, we'll set price change to 0 since we need historical data
    let price_change_24h = "0".to_string();
    let price_change_percent_24h = "0.00".to_string();

    Ok(TickerResponse {
        market: market.clone(),
        last_price,
        price_change_24h,
        price_change_percent_24h,
        volume_24h,
        high_24h,
        low_24h,
        bid_price: best_bid,
        ask_price: best_ask,
        timestamp: now.timestamp(),
    })
}

pub fn ticker_routes() -> Route {
    Route::new()
        .at("/", get(get_ticker))
        .at("/all", get(get_all_tickers))
}