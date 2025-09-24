-- schema for indexer
CREATE TABLE IF NOT EXISTS transfers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    block_number BIGINT NOT NULL,
    tx_hash TEXT NOT NULL,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    amount TEXT NOT NULL,
    timestamp INTEGER DEFAULT (strftime('%s','now'))
);

CREATE TABLE IF NOT EXISTS net_flows (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    exchange TEXT NOT NULL UNIQUE,
    cumulative_in TEXT NOT NULL DEFAULT '0',
    cumulative_out TEXT NOT NULL DEFAULT '0',
    net_flow TEXT NOT NULL DEFAULT '0',
    last_updated DATETIME DEFAULT CURRENT_TIMESTAMP
);
