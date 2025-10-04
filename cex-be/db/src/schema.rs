// @generated automatically by Diesel CLI.

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

diesel::allow_tables_to_appear_in_same_query!(orders, trades, users,);
