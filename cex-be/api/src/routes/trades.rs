use poem::{get, handler, web::Json, Route};

#[handler]
async fn trades() -> Json<String> {
    // get trades from database
    Json("hi there".to_string())
}

pub fn trade_routes() -> Route {
    Route::new()
        .at("/", get(trades))
}