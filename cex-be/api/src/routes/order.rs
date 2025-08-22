use std::sync::Arc;

use poem::{get, handler, post, web::{Data, Json}, Route};

use crate::{redismanager::RedisManager, types::{CreateOrder, CreateOrderData, DeleteOrder, DeleteOrderData, EngineData, GetOpenOrder, MessageToEngine, OpenMarketRequest}};

// / post
// / delete
// /open get
#[handler]
async fn create_order(Data(manager): Data<&Arc<RedisManager>>, Json(payload): Json<CreateOrder>) -> poem::Result<Json<String>> {
    // println!(
    //     "{} {} {} {:?} {}",
    //     payload.market, payload.price, payload.quantity, payload.side, payload.user_id
    // );
    let response = manager
                .send_and_await(MessageToEngine {
                    type_: "CREATE_ORDER".to_string(),
                    data: EngineData::Order(CreateOrderData {
                        market: payload.market,
                        price: payload.price,
                        quantity: payload.quantity,
                        side: payload.side,
                        user_id: payload.user_id
                    })
                });
    println!("order all good");
    let response = response.await.map_err(|e| poem::Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Json(response))
}

#[handler]
async fn delete_order(Data(manager): Data<&Arc<RedisManager>>, Json(payload): Json<DeleteOrder>) -> poem::Result<Json<String>> {
    println!(
        "{} {}",
        payload.order_id, payload.market
    );
    let response = manager
                                        .send_and_await(
                                            MessageToEngine { 
                                                type_: "CANCEL_ORDER".to_string(), 
                                                data: EngineData::DeleteOrder(DeleteOrderData {
                                                    market: payload.market,
                                                    order_id: payload.order_id
                                            }) 
                                        });
    println!("delete order all good");
    let response = response.await.map_err(|e| poem::Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Json(response))
}

#[handler]
async fn open(Data(manager): Data<&Arc<RedisManager>>, Json(payload): Json<OpenMarketRequest>) -> poem::Result<Json<String>> {
    println!(
        "{} {}",
        payload.user_id, payload.market
    );
    let response = manager
                                        .send_and_await(
                                            MessageToEngine { 
                                                type_: "GET_OPEN_ORDERS".to_string(), 
                                                data: EngineData::OpenOrder(GetOpenOrder {
                                                    user_id: payload.user_id,
                                                    market: payload.market
                                                }) 
                                            });

    println!("open order all good");
    let response = response.await.map_err(|e| poem::Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Json(response))
}

pub fn order_routes() -> Route {
    Route::new()
        .at("/", post(create_order).delete(delete_order))
        .at("/open", get(open))
}