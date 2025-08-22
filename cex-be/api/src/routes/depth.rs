use poem::{get, handler, web::{Data, Json, Query}, Route};
use std::sync::Arc;

use crate::{redismanager::RedisManager, types::{DepthQuery, MessageToEngine, SymbolData}};

#[handler]
async fn depth_order(
    Data(manager): Data<&Arc<RedisManager>>,
    Query(query): Query<DepthQuery>,
) -> poem::Result<Json<String>> {
    let response = manager
        .send_and_await(MessageToEngine {
            type_: "GET_DEPTH".to_string(),
            data: SymbolData {
                market: query.symbol.clone().to_string(),
            },
        })
        .await
        .map_err(poem::error::InternalServerError)?;
    println!("all good");
    Ok(Json(response))
}


pub fn depth_routes() -> Route {
    Route::new()
        .at("/", get(depth_order))
}