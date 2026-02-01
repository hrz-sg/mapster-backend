CREATE TABLE post_media (
    id VARCHAR(25) PRIMARY KEY,
    post_id VARCHAR(25) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    media_url TEXT NOT NULL,
    media_type VARCHAR(16) NOT NULL,
    mime_type VARCHAR(128) NOT NULL,
    width INT,
    height INT,
    file_size BIGINT,
    duration INT,
    sort_order INT NOT NULL DEFAULT 0,

    -- Timestamps / Audit
    cid VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE SET NULL,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE SET NULL,
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);