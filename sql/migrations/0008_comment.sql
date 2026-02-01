CREATE TABLE comment (
    id VARCHAR(25) PRIMARY KEY,
    owner_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    entity_type comment_entity_type NOT NULL,
    entity_id VARCHAR(25) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    parent_id VARCHAR(25) REFERENCES comment(id) ON DELETE CASCADE,
    text TEXT NOT NULL,

    -- Timestamps / Audit
    cid VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_comment_entity ON comment(entity_type, entity_id); -- Comments to entities (Post)
CREATE INDEX idx_comment_parent ON comment(parent_id); -- Comment replies

-- CommentMedia
CREATE TABLE comment_media (
    id VARCHAR(25) PRIMARY KEY,
    comment_id VARCHAR(25) NOT NULL REFERENCES comment(id) ON DELETE CASCADE,
    owner_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    media_url TEXT NOT NULL,
    media_type VARCHAR(16) NOT NULL CHECK (media_type = 'image'),
    mime_type VARCHAR(128) NOT NULL,
    width INT,
    height INT,
    file_size BIGINT,
    sort_order INT NOT NULL DEFAULT 0,
    is_blocked BOOLEAN NOT NULL DEFAULT FALSE,
    moderation_reason TEXT,

    -- Timestamps / Audit
    cid VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    mtime TIMESTAMPTZ NOT NULL DEFAULT now(),

    FOREIGN KEY (comment_id) REFERENCES comment(id) ON DELETE CASCADE
);