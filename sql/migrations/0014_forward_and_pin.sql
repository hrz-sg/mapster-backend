-- Post Forward
CREATE TABLE post_forward (
    id VARCHAR(25) PRIMARY KEY,
    post_id VARCHAR(25) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    user_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    chat_id VARCHAR(25) NOT NULL REFERENCES chat(id) ON DELETE CASCADE,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(post_id, user_id, chat_id)
);

-- Journey Forward
CREATE TABLE journey_forward (
    id VARCHAR(25) PRIMARY KEY,
    journey_id VARCHAR(25) NOT NULL REFERENCES journey(id) ON DELETE CASCADE,
    user_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    chat_id VARCHAR(25) NOT NULL REFERENCES chat(id) ON DELETE CASCADE,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE(journey_id, user_id, chat_id)
);

CREATE TABLE chat_pinned_content (
    id VARCHAR(25) PRIMARY KEY,
    chat_id VARCHAR(25) REFERENCES chat(id) ON DELETE CASCADE,

    post_id VARCHAR(25) REFERENCES post(id),
    journey_id VARCHAR(25) REFERENCES journey(id),

    pinned_by VARCHAR(25) REFERENCES "user"(id),
    sort_order INT NOT NULL DEFAULT 0,
    pinned_at TIMESTAMPTZ DEFAULT now(),

    CHECK (
        (post_id IS NOT NULL AND journey_id IS NULL) OR
        (post_id IS NULL AND journey_id IS NOT NULL)
    ),

    UNIQUE(chat_id, post_id),
    UNIQUE(chat_id, journey_id)
);