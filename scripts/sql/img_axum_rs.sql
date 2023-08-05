CREATE TABLE IF NOT EXISTS images (
    id CHAR(20) PRIMARY KEY,
    path VARCHAR(255) NOT NULL,
    hash CHAR(64) NOT NULL,
    dateline TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE(path),
    UNIQUE(hash)
);