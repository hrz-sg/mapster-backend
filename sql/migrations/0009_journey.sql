-- Journey
CREATE TABLE journey (
    id VARCHAR(25) PRIMARY KEY,
    owner_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    title VARCHAR(256) NOT NULL,
    description TEXT,
    cover_object_key TEXT,
    status journey_status NOT NULL DEFAULT 'Draft',

    -- Stats (cache)
    total_likes BIGINT NOT NULL DEFAULT 0, -- the sum of all likes from posts in journey
    save_count BIGINT NOT NULL DEFAULT 0,
    forward_count BIGINT NOT NULL DEFAULT 0,

    -- Timestamps / Audit
    cid VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_journey_owner ON journey(owner_id); -- user journeys
CREATE INDEX idx_journey_feed ON journey (ctime DESC) WHERE status = 'Published'; -- journeys feed

CREATE TABLE journey_post (
    journey_id VARCHAR(25) NOT NULL REFERENCES journey(id) ON DELETE CASCADE,
    post_id VARCHAR(25) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    sort_order INT NOT NULL,

    PRIMARY KEY (journey_id, post_id),
    UNIQUE (journey_id, sort_order),

    -- Timestamps
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);