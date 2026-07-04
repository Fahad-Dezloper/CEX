use log::info;
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
    let pool = establish_connection();
    let mut conn = pool.get()
        .map_err(|e| InternalServerError(e))?;

    let mut query = trades::table.into_boxed();
    
    if let Some(market) = &params.market {
        query = query.filter(trades::market.eq(market));
    }
    
    query = query.order(trades::timestamp.desc());
    
    let limit = params.limit.unwrap_or(100).min(1000);
    query = query.limit(limit);

    let trades_result = query
        .load::<Trade>(&mut conn)
        .map_err(|e| InternalServerError(e))?;

    info!("Retrieved {} trades", trades_result.len());
    info!("Trades: {:?}", trades_result);

    Ok(Json(TradesResponse {
        total: trades_result.len(),
        trades: trades_result,
    }))
}

pub fn trade_routes() -> Route {
    Route::new()
        .at("/", get(get_trades))
}