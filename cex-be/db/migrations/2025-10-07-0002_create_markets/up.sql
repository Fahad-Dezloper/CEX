-- Create markets table
CREATE TABLE IF NOT EXISTS markets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    base_asset VARCHAR(16) NOT NULL,
    quote_asset VARCHAR(16) NOT NULL,
    symbol VARCHAR(64) NOT NULL UNIQUE,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    price_precision INTEGER NOT NULL DEFAULT 2,
    quantity_precision INTEGER NOT NULL DEFAULT 6,
    min_price DOUBLE PRECISION NOT NULL DEFAULT 0.0001,
    max_price DOUBLE PRECISION NOT NULL DEFAULT 1000000000,
    min_order_size DOUBLE PRECISION NOT NULL DEFAULT 0.000001,
    max_order_size DOUBLE PRECISION NOT NULL DEFAULT 1000000000
);

CREATE INDEX IF NOT EXISTS idx_markets_enabled ON markets(enabled);

