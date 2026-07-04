// @generated automatically by Diesel CLI.

diesel::table! {
    markets (id) {
        id -> Uuid,
        #[max_length = 16]
        base_asset -> Varchar,
        #[max_length = 16]
        quote_asset -> Varchar,
        #[max_length = 64]
        symbol -> Varchar,
        enabled -> Bool,
        price_precision -> Int4,
        quantity_precision -> Int4,
        min_price -> Float8,
        max_price -> Float8,
        min_order_size -> Float8,
        max_order_size -> Float8,
    }
}

diesel::table! {
    orders (id, created_at) {
        id -> Uuid,
        executed_qty -> Numeric,
        #[max_length = 255]
        market -> Varchar,
        #[max_length = 255]
        price -> Varchar,
        #[max_length = 255]
        quantity -> Varchar,
        #[max_length = 50]
        side -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    trades (id, timestamp) {
        id -> Uuid,
        is_buyer_maker -> Bool,
        #[max_length = 255]
        price -> Varchar,
        #[max_length = 255]
        quantity -> Varchar,
        #[max_length = 255]
        quote_quantity -> Varchar,
        timestamp -> Timestamp,
        #[max_length = 255]
        market -> Varchar,
    }
}

diesel::table! {
    user_assets (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 64]
        symbol -> Varchar,
        amount -> Float8,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Date,
        updated_at -> Date,
    }
}

diesel::joinable!(user_assets -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(markets, orders, trades, user_assets, users,);
