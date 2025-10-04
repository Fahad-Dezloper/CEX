-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at DATE NOT NULL DEFAULT CURRENT_DATE,
    updated_at DATE NOT NULL DEFAULT CURRENT_DATE
);

-- Create trades table (TimescaleDB compatible)
CREATE TABLE trades (
    id UUID NOT NULL,
    is_buyer_maker BOOLEAN NOT NULL,
    price VARCHAR(255) NOT NULL,
    quantity VARCHAR(255) NOT NULL,
    quote_quantity VARCHAR(255) NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    market VARCHAR(255) NOT NULL,
    PRIMARY KEY (id, timestamp)
);

-- Create orders table (TimescaleDB compatible)
CREATE TABLE orders (
    id UUID NOT NULL,
    executed_qty NUMERIC(20, 8) NOT NULL,
    market VARCHAR(255) NOT NULL,
    price VARCHAR(255) NOT NULL,
    quantity VARCHAR(255) NOT NULL,
    side VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id, created_at)
);

-- Create indexes for better performance
CREATE INDEX idx_trades_market ON trades(market);
CREATE INDEX idx_trades_timestamp ON trades(timestamp);
CREATE INDEX idx_orders_market ON orders(market);
CREATE INDEX idx_orders_side ON orders(side);
CREATE INDEX idx_orders_created_at ON orders(created_at);