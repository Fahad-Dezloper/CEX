use poem::{listener::TcpListener, EndpointExt, Route, Server};

use crate::{redismanager::RedisManager, routes::{depth, klines, order, ticker, trades}};
mod routes {
    pub mod order;
    pub mod depth;
    pub mod trades;
    pub mod klines;
    pub mod ticker;
}
mod types;
mod redismanager;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let manager = RedisManager::new("redis://127.0.0.1/")
        .await
        .expect("failed to connect to Redis");

    let app = Route::new()
                    .nest("/api/v1/order", order::order_routes())
                    .nest("/api/v1/depth", depth::depth_routes())
                    .nest("/api/v1/trades", trades::trade_routes())
                    .nest("/api/v1/klines", klines::klines_routes())
                    .nest("/api/v1/tickers", ticker::ticker_routes())
                    .data(manager);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}