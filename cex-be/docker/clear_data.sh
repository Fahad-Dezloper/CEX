#!/usr/bin/env bash
set -euo pipefail

# Empty TimescaleDB/Postgres data and Redis cache inside docker containers.
# Defaults align with docker-compose service names and env in this repo.

DB_CONTAINER=${DB_CONTAINER:-timescaledb}
REDIS_CONTAINER=${REDIS_CONTAINER:-redis}

POSTGRES_DB=${POSTGRES_DB:-cex_db}
POSTGRES_USER=${POSTGRES_USER:-postgres}
POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-exchange}

COMPOSE_FILE_DIR=$(cd "$(dirname "$0")" && pwd)
COMPOSE_FILE="$COMPOSE_FILE_DIR/docker-compose.yml"

RESTART_CONTAINERS=${RESTART_CONTAINERS:-false}

echo "[clear_data] Target DB container: $DB_CONTAINER (db=$POSTGRES_DB user=$POSTGRES_USER)"
echo "[clear_data] Target Redis container: $REDIS_CONTAINER"
echo "[clear_data] Restart containers: $RESTART_CONTAINERS"

container_exists() {
  docker ps -a --format '{{.Names}}' | grep -qx "$1"
}

container_running() {
  docker ps --format '{{.Names}}' | grep -qx "$1"
}

ensure_running() {
  local name="$1"
  if container_running "$name"; then
    return 0
  fi
  if container_exists "$name"; then
    echo "[clear_data] Starting existing container $name ..."
    if docker start "$name" >/dev/null 2>&1; then
      return 0
    else
      echo "[clear_data] Warning: failed to start $name; will try alternatives if possible."
      return 1
    fi
  fi
  echo "[clear_data] Container $name not found; starting via docker compose..."
  if docker compose -f "$COMPOSE_FILE" up -d "$name" >/dev/null 2>&1; then
    return 0
  fi
  # If compose failed due to name conflict, try starting by name directly
  if ! container_running "$name"; then
    if container_exists "$name"; then
      if docker start "$name" >/dev/null 2>&1; then
        return 0
      fi
    fi
  fi
  return 1
}

# Ensure containers are created/running to exec into them
if ! ensure_running "$DB_CONTAINER"; then
  echo "[clear_data] Error: database container $DB_CONTAINER is not running and could not be started. Aborting."
  exit 1
fi

ensure_running "$REDIS_CONTAINER" || true

REDIS_RUNNING=false
if container_running "$REDIS_CONTAINER"; then
  REDIS_RUNNING=true
fi

if [ "$RESTART_CONTAINERS" = "true" ]; then
  echo "[clear_data] Restarting containers before reset..."
  docker stop "$REDIS_CONTAINER" >/dev/null 2>&1 || true
  docker stop "$DB_CONTAINER" >/dev/null 2>&1 || true
  docker start "$DB_CONTAINER" >/dev/null
  docker start "$REDIS_CONTAINER" >/dev/null
fi

echo "[clear_data] Flushing Redis (FLUSHALL)..."
if [ "$REDIS_RUNNING" = true ]; then
  docker exec "$REDIS_CONTAINER" redis-cli FLUSHALL
else
  if command -v redis-cli >/dev/null 2>&1; then
    echo "[clear_data] Redis container not running; using host redis-cli on 127.0.0.1:6379"
    redis-cli -h 127.0.0.1 -p 6379 FLUSHALL || echo "[clear_data] Warning: host redis-cli flush failed."
  else
    echo "[clear_data] Warning: Redis not flushed (container not running and redis-cli not found on host)."
  fi
fi

echo "[clear_data] Truncating application tables (excluding system and TimescaleDB internal schemas)..."
docker exec -e PGPASSWORD="$POSTGRES_PASSWORD" -i "$DB_CONTAINER" psql -v ON_ERROR_STOP=1 -h 127.0.0.1 -U "$POSTGRES_USER" -d "$POSTGRES_DB" <<'SQL'
DO $do$
DECLARE
  stmt text;
BEGIN
  SELECT 'TRUNCATE TABLE ' || string_agg(format('%I.%I', schemaname, tablename), ', ') || ' RESTART IDENTITY CASCADE'
    INTO stmt
  FROM pg_tables
  WHERE schemaname NOT IN ('pg_catalog', 'information_schema')
    AND schemaname NOT LIKE '\_timescaledb%'
    AND schemaname NOT IN ('timescaledb_information');

  IF stmt IS NOT NULL THEN
    EXECUTE stmt;
  END IF;
END
$do$;
SQL

echo "[clear_data] Done. Redis is empty and Postgres tables are truncated; schema and extensions preserved."



# Apply schema for markets table (idempotent) and seed enabled markets
echo "[clear_data] Ensuring markets schema exists and seeding default markets..."
docker exec -e PGPASSWORD="$POSTGRES_PASSWORD" -i "$DB_CONTAINER" psql -v ON_ERROR_STOP=1 -h 127.0.0.1 -U "$POSTGRES_USER" -d "$POSTGRES_DB" <<'SQL'
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

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

-- Seed default enabled markets (idempotent)
INSERT INTO markets (base_asset, quote_asset, symbol, enabled, price_precision, quantity_precision, min_price, max_price, min_order_size, max_order_size) VALUES
 ('BTC','USDC','BTC-USDC', TRUE, 2, 8, 0.01, 1000000, 0.001, 100.0),
 ('ETH','USDC','ETH-USDC', TRUE, 2, 6, 0.01, 100000, 0.01, 1000.0),
 ('SOL','USDC','SOL-USDC', TRUE, 4, 4, 0.0001, 1000, 0.1, 10000.0),
 ('BNB','USDC','BNB-USDC', TRUE, 2, 6, 0.01, 100000, 0.01, 1000.0),
 ('DOGECOIN','USDC','DOGECOIN-USDC', TRUE, 4, 4, 0.0001, 1000, 1.0, 1000000.0),
 ('SUI','USDC','SUI-USDC', TRUE, 4, 4, 0.0001, 1000, 0.1, 100000.0),
 ('HYPERLIQUID','USDC','HYPERLIQUID-USDC', TRUE, 4, 4, 0.0001, 1000, 0.1, 100000.0)
ON CONFLICT (symbol) DO UPDATE SET
  enabled = EXCLUDED.enabled,
  price_precision = EXCLUDED.price_precision,
  quantity_precision = EXCLUDED.quantity_precision,
  min_price = EXCLUDED.min_price,
  max_price = EXCLUDED.max_price,
  min_order_size = EXCLUDED.min_order_size,
  max_order_size = EXCLUDED.max_order_size;
SQL

echo "[clear_data] Markets schema ensured and defaults seeded. API will cache them to Redis on startup."
