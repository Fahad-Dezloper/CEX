use poem::{get, handler, web::Json, Route};

#[handler]
async fn get_ticker() -> Json<String> {
    Json("yoo ticker here".to_string())
}

pub fn ticker_routes() -> Route {
    Route::new()
        .at("/", get(get_ticker))
}