CREATE TABLE IF NOT EXISTS audit_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    btc_reserve REAL NOT NULL,
    sdac_supply REAL NOT NULL,
    reserve_ratio REAL NOT NULL,
    compliance_score REAL NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_timestamp ON audit_history(timestamp);
