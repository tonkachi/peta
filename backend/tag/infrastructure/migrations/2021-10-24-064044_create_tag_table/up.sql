CREATE TABLE IF NOT EXISTS tags (
    hash VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tags_fusen_ids (
    tag_hash VARCHAR,
    fusen_id VARCHAR,
    FOREIGN KEY (tag_hash) REFERENCES tags (hash),
    PRIMARY KEY (tag_hash, fusen_id),
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);
