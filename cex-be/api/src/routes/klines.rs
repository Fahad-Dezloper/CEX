use std::sync::Arc;

use poem::{get, handler, http::StatusCode, web::{Data, Json, Query}, Route};
// use tokio_postgres::Client;

use crate::{redismanager::RedisManager, types::KlinesQuery};

#[handler]
async fn get_klines(
    Data(_manager): Data<&Arc<RedisManager>>,
    Query(query): Query<KlinesQuery>
) -> poem::Result<Json<String>> {
    let _sql = match query.interval.as_str() {
        "1m" => "SELECT * FROM klines_1m WHERE bucket >= $1 AND bucket <= $2",
        "1h" => "SELECT * FROM klines_1h WHERE bucket >= $1 AND bucket <= $2",
        "1w" => "SELECT * FROM klines_1w WHERE bucket >= $1 AND bucket <= $2",
        _ => return Err(poem::Error::from_string("Invalid interval", poem::http::StatusCode::BAD_REQUEST)),
    };

    // TODO: replace this with a real Postgres query when you wire the client
    return Err(poem::Error::from_status(StatusCode::NOT_IMPLEMENTED));

    /*
    match Client::query(query, &[])
            .await
    {
        Ok(rows) => {
            let result: Vec<KlinesData> = rows
                .into_iter()
                .map(|row| KlinesData {
                    close: row.get("close"),
                    end: row.get("bucket"),
                    high: row.get("high"),
                    low: row.get("low"),
                    open: row.get("open"),
                    quote_volume: row.get("quoteVolume"),
                    start: row.get("start"),
                    trades: row.get("trades"),
                    volume: row.get("volume"),
                })
                .collect();
        
                Json(result).into_response()
        }
        Err(err) => {
            eprintln!("Database error: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
    */
}

pub fn klines_routes() -> Route {
    Route::new()
        .at("/", get(get_klines))
}