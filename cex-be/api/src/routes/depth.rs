use poem::{get, handler, web::{Json, Query}, Route};

use crate::types::DepthQuery;

#[handler]
async fn depth_order(Query(query): Query<DepthQuery>) -> Json<String> {
    Json("yoo".to_string())
    // redis manager
}


pub fn depth_routes() -> Route {
    Route::new()
        .at("/", get(depth_order))
}