-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Convert trades table to hypertable (partitioned by timestamp)
SELECT create_hypertable('trades', 'timestamp');

-- Convert orders table to hypertable (partitioned by created_at)
SELECT create_hypertable('orders', 'created_at');
