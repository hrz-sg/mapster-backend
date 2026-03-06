-- Post
CREATE TABLE post (
    id VARCHAR(25) PRIMARY KEY,
    owner_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    location_id VARCHAR(25) REFERENCES location(id) ON DELETE SET NULL,
    title VARCHAR(256) NOT NULL,
    description TEXT NOT NULL,
    status post_status NOT NULL DEFAULT 'Draft', -- processing | ready
    cover_media_key TEXT,
    media_count INT NOT NULL DEFAULT 0,
    like_count BIGINT NOT NULL DEFAULT 0,
    comment_count BIGINT NOT NULL DEFAULT 0,
    save_count BIGINT NOT NULL DEFAULT 0,
    forward_count BIGINT NOT NULL DEFAULT 0,

    -- Audit
    cid VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_post_owner_ctime ON post (owner_id, ctime DESC); -- User posts
CREATE INDEX idx_post_feed ON post (ctime DESC) WHERE status = 'Published'; -- public feed