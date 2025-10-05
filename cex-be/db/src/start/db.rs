use db::{self, establish_connection, add_db};
use dotenv::dotenv;
use env_logger;
use log::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");
    env_logger::init();

    let pool = establish_connection();
    add_db(pool).await;
}