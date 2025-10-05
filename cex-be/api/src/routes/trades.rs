use poem::{get, handler, web::{Json, Query}, Route, Result, error::InternalServerError};
use serde::{Serialize, Deserialize};
use db::{establish_connection, Trade, trades};
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::expression_methods::ExpressionMethods;

#[derive(Deserialize)]
pub struct TradesParams {
    pub market: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct TradesResponse {
    pub trades: Vec<Trade>,
    pub total: usize,
}

#[handler]
async fn get_trades(Query(params): Query<TradesParams>) -> Result<Json<TradesResponse>> {
    // Establish database connection
    let pool = establish_connection();
    let mut conn = pool.get()
        .map_err(|e| InternalServerError(e))?;

    // Build query
    let mut query = trades::table.into_boxed();
    
    // Filter by market if provided
    if let Some(market) = &params.market {
        query = query.filter(trades::market.eq(market));
    }
    
    // Order by timestamp descending (most recent first)
    query = query.order(trades::timestamp.desc());
    
    // Apply limit (default to 100 if not specified)
    let limit = params.limit.unwrap_or(100).min(1000); // Cap at 1000
    query = query.limit(limit);

    // Execute query
    let trades_result = query
        .load::<Trade>(&mut conn)
        .map_err(|e| InternalServerError(e))?;

    Ok(Json(TradesResponse {
        total: trades_result.len(),
        trades: trades_result,
    }))
}

pub fn trade_routes() -> Route {
    Route::new()
        .at("/", get(get_trades))
}