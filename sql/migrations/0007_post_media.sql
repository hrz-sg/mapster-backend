CREATE TABLE post_media (
    id VARCHAR(25) PRIMARY KEY,
    post_id VARCHAR(25) NOT NULL REFERENCES post(id) ON DELETE CASCADE,

    media_type media_type NOT NULL, -- image / video
    object_key TEXT NOT NULL,

    mime_type VARCHAR(128) NOT NULL,
    etag VARCHAR(64) NOT NULL,

    width INT,
    height INT,
    duration INT,
    file_size BIGINT,

    sort_order INT NOT NULL DEFAULT 0,
    status media_status NOT NULL DEFAULT 'Draft',

    -- Audit
    cid VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);
