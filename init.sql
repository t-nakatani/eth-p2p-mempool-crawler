-- Ethereum P2P Mempool Crawler Database Initialization Script
-- This combines all migrations into a single initialization file

-- Create transactions table
CREATE TABLE IF NOT EXISTS transactions (
    hash VARCHAR(66) PRIMARY KEY,
    -- tx_type SMALLINT NOT NULL,
    -- sender VARCHAR(42),
    -- receiver VARCHAR(42),
    -- value_wei TEXT NOT NULL,
    -- gas_limit BIGINT NOT NULL,
    -- gas_price_or_max_fee_wei TEXT,
    -- max_priority_fee_wei TEXT,
    -- input_len INTEGER NOT NULL,
    -- first_seen_at TIMESTAMPTZ NOT NULL,
    -- is_private BOOLEAN NOT NULL DEFAULT FALSE
);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_transactions_first_seen_at ON transactions(first_seen_at DESC);
-- CREATE INDEX IF NOT EXISTS idx_transactions_tx_type ON transactions(tx_type);
-- CREATE INDEX IF NOT EXISTS idx_transactions_sender ON transactions(sender);
-- CREATE INDEX IF NOT EXISTS idx_transactions_receiver ON transactions(receiver);
-- CREATE INDEX IF NOT EXISTS idx_transactions_is_private ON transactions(is_private);
