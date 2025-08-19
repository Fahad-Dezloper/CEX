use poem::{listener::TcpListener, Route, Server};

use crate::routes::{depth, klines, order, trades, ticker};
mod routes {
    pub mod order;
    pub mod depth;
    pub mod trades;
    pub mod klines;
    pub mod ticker;
}
mod types;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
                    .nest("/api/v1/order", order::order_routes())
                    .nest("/api/v1/depth", depth::depth_routes())
                    .nest("/api/v1/trades", trades::trade_routes())
                    .nest("/api/v1/klines", klines::klines_routes())
                    .nest("/api/v1/tickers", ticker::ticker_routes());

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}