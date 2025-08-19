use poem::{get, handler, web::Json, Route};

#[handler]
async fn get_klines() -> Json<String> {
    Json("yoo klines here man".to_string())
}

pub fn klines_routes() -> Route {
    Route::new()
        .at("/", get(get_klines))
}