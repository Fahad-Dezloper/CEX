use std::sync::Arc;

use poem::{get, handler, post, web::{Data, Json}, Route, Result, error::InternalServerError, http::StatusCode};
use validator::Validate;
use serde_json::json;
use log::{info, warn, error};

use crate::{redismanager::RedisManager, types::{CreateOrder, CreateOrderData, DeleteOrder, DeleteOrderData, EngineData, GetOpenOrder, MessageToEngine}, middleware::extract_claims, validation::{OrderValidator, validate_market_format}};

// / post
// / delete
// /open get
#[handler]
async fn create_order(
    Data(manager): Data<&Arc<RedisManager>>, 
    Json(payload): Json<CreateOrder>,
    request: &poem::Request,
) -> Result<Json<serde_json::Value>> {
    let claims = extract_claims(request)
        .ok_or_else(|| poem::Error::from_string("Authentication required", poem::http::StatusCode::UNAUTHORIZED))?;

    info!("Creating order for user: {}", claims.user_id);

    if let Err(validation_errors) = payload.validate() {
        warn!("Order validation failed for user {}: {:?}", claims.user_id, validation_errors);
        return Ok(Json(json!({
            "error": "Validation failed",
            "details": validation_errors.field_errors()
        })));
    }

    if !validate_market_format(&payload.market) {
        warn!("Invalid market format: {}", payload.market);
        return Ok(Json(json!({
            "error": "Invalid market format. Expected format: BASE-QUOTE (e.g., BTC-USD)"
        })));
    }

    let validator = OrderValidator::new();
    if let Err(validation_error) = validator.validate_order(&payload.market, payload.price, payload.quantity) {
        warn!("Order validation failed for user {}: {:?}", claims.user_id, validation_error);
        return Ok(Json(validation_error));
    }

    let min_order_value = 1.0;
    let order_value = payload.price * payload.quantity;
    if order_value < min_order_value {
        warn!("Order value too low: ${}", order_value);
        return Ok(Json(json!({
            "error": format!("Minimum order value is ${}", min_order_value)
        })));
    }

    let response = manager
        .send_and_await(MessageToEngine {
            type_: "CREATE_ORDER".to_string(),
            data: EngineData::Order(CreateOrderData {
                market: payload.market.clone(),
                price: payload.price,
                quantity: payload.quantity,
                side: payload.side,
                user_id: claims.user_id.clone()
            })
        });

    match response.await {
        Ok(response) => {
            info!("Order created successfully for user: {}", claims.user_id);
            Ok(Json(json!({
                "success": true,
                "message": "Order created successfully",
                "data": response
            })))
        }
        Err(e) => {
            error!("Failed to create order for user {}: {}", claims.user_id, e);
            Ok(Json(json!({
                "error": "Failed to create order",
                "details": e.to_string()
            })))
        }
    }
}


#[handler]
async fn delete_order(
    Data(manager): Data<&Arc<RedisManager>>, 
    Json(payload): Json<DeleteOrder>,
    request: &poem::Request,
) -> Result<Json<serde_json::Value>> {
    let claims = extract_claims(request)
        .ok_or_else(|| poem::Error::from_string("Authentication required", poem::http::StatusCode::UNAUTHORIZED))?;

    info!("Deleting order {} for user: {}", payload.order_id, claims.user_id);

    if payload.order_id.is_empty() || payload.market.is_empty() {
        warn!("Invalid delete order request for user {}: empty order_id or market", claims.user_id);
        return Ok(Json(json!({
            "error": "Order ID and market are required"
        })));
    }

    if !validate_market_format(&payload.market) {
        warn!("Invalid market format for delete order: {}", payload.market);
        return Ok(Json(json!({
            "error": "Invalid market format. Expected format: BASE-QUOTE (e.g., BTC-USD)"
        })));
    }

    let response = manager
        .send_and_await(
            MessageToEngine { 
                type_: "CANCEL_ORDER".to_string(), 
                data: EngineData::DeleteOrder(DeleteOrderData {
                    market: payload.market.clone(),
                    order_id: payload.order_id.clone()
                }) 
            });

    match response.await {
        Ok(response) => {
            info!("Order {} deleted successfully for user: {}", payload.order_id, claims.user_id);
            Ok(Json(json!({
                "success": true,
                "message": "Order cancelled successfully",
                "data": response
            })))
        }
        Err(e) => {
            error!("Failed to delete order {} for user {}: {}", payload.order_id, claims.user_id, e);
            Ok(Json(json!({
                "error": "Failed to cancel order",
                "details": e.to_string()
            })))
        }
    }
}

#[handler]
async fn get_open_orders(
    Data(manager): Data<&Arc<RedisManager>>, 
    request: &poem::Request,
) -> Result<Json<serde_json::Value>> {
    let claims = extract_claims(request)
        .ok_or_else(|| poem::Error::from_string("Authentication required", poem::http::StatusCode::UNAUTHORIZED))?;

    info!("Getting open orders for user: {}", claims.user_id);

    let market = request
        .uri()
        .query()
        .and_then(|q| {
            url::form_urlencoded::parse(q.as_bytes())
                .find(|(key, _)| key == "market")
                .map(|(_, value)| value.to_string())
        })
        .unwrap_or_else(|| "BTC-USD".to_string());

    if !validate_market_format(&market) {
        warn!("Invalid market format for open orders: {}", market);
        return Ok(Json(json!({
            "error": "Invalid market format. Expected format: BASE-QUOTE (e.g., BTC-USD)"
        })));
    }

    let response = manager
        .send_and_await(
            MessageToEngine { 
                type_: "GET_OPEN_ORDERS".to_string(), 
                data: EngineData::OpenOrder(GetOpenOrder {
                    user_id: claims.user_id.clone(),
                    market: market.clone()
                }) 
            });

    match response.await {
        Ok(response) => {
            info!("Open orders retrieved successfully for user: {}", claims.user_id);
            Ok(Json(json!({
                "success": true,
                "message": "Open orders retrieved successfully",
                "market": market,
                "data": response
            })))
        }
        Err(e) => {
            error!("Failed to get open orders for user {}: {}", claims.user_id, e);
            Ok(Json(json!({
                "error": "Failed to retrieve open orders",
                "details": e.to_string()
            })))
        }
    }
}

pub fn order_routes() -> Route {
    Route::new()
        .at("/", post(create_order).delete(delete_order))
        .at("/open", get(get_open_orders))
}