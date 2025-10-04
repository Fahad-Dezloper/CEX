use poem::{get, handler, web::Json, Route};
use serde::Serialize;


#[derive(Serialize)]
pub struct TradesQuery {
    pub market: String
}

#[handler]
async fn trades() -> Json<TradesQuery> {
    // get trades from database
    Json(TradesQuery {
        market: "BTCUSDT".to_string()
    })
}

pub fn trade_routes() -> Route {
    Route::new()
        .at("/", get(trades))
}