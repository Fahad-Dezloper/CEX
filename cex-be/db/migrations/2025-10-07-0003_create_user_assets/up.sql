CREATE TABLE IF NOT EXISTS user_assets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    symbol VARCHAR(64) NOT NULL,
    amount DOUBLE PRECISION NOT NULL DEFAULT 0,
    UNIQUE (user_id, symbol)
);

CREATE INDEX IF NOT EXISTS idx_user_assets_user_id ON user_assets(user_id);

