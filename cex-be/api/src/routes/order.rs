use poem::{delete, get, handler, post, web::Json, Route};

use crate::types::{CreateOrder, DeleteOrder};

// / post
// / delete
// /open get
#[handler]
async fn create_order(Json(payload): Json<CreateOrder>) -> Json<String> {
    println!(
        "{} {} {} {:?} {}",
        payload.market, payload.price, payload.quantity, payload.side, payload.user_id
    );
    Json("create order".to_string())
}

#[handler]
async fn delete_order(Json(payload): Json<DeleteOrder>) -> Json<String> {
    println!(
        "{} {}",
        payload.order_id, payload.market
    );
    Json("delete order".to_string())
}

#[handler]
async fn open() -> Json<String> {
    // response type willl come from redis manager
    Json("yoo my man".to_string())
}

pub fn order_routes() -> Route {
    Route::new()
        .at("/", post(create_order))
        .at("/", delete(delete_order))
        .at("/sol_usd", get(open))
}